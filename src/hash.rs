use std::{
    collections::{hash_map::Entry, HashMap},
    fs, io,
};

use compress_tools::{ArchiveContents, ArchiveIterator};
use sha1::{Digest, Sha1};

use crate::scan::{FileHashes, HashedArchive};

pub fn calc_sha1(file_path: &str) -> Option<HashedArchive> {
    // note: doesn't work on headerless NES roms
    let kind = match infer::get_from_path(file_path) {
        Err(_) => return None,
        Ok(k) => k?,
    };

    match kind.mime_type() {
        "application/zip" => Some(read_7z(file_path)),
        "application/x-7z-compressed" => Some(read_7z(file_path)),
        "application/x-nintendo-nes-rom" => Some(read_unarchived(file_path)),
        _ => None,
    }
}

pub fn hex_string_from_slice(input: &[u8]) -> String {
    input
        .iter()
        .map(|&v| format!("{:x}", v))
        .collect::<String>()
}

pub fn read_unarchived(file_path: &str) -> HashedArchive {
    let mut file = fs::File::open(file_path).unwrap();
    let mut hasher = Sha1::new();
    let mut file_contents: Vec<u8> = Vec::new();
    io::copy(&mut file, &mut file_contents).unwrap();
    hasher.update(&file_contents);

    HashedArchive {
        f_name: file_path.to_string(),
        files: vec![(
            file_path.to_string(),
            FileHashes {
                sha1: hasher
                    .finalize()
                    .as_slice()
                    .try_into()
                    .expect("Wrong length"),
                crc: crc32fast::hash(&file_contents),
            },
        )],
    }
}

pub fn read_7z(file_path: &str) -> HashedArchive {
    let mut ret = HashedArchive {
        f_name: file_path.to_string(),
        files: Vec::new(),
    };

    let mut data: HashMap<String, Vec<u8>> = HashMap::new();
    let mut name = String::default();

    let file = fs::File::open(file_path).unwrap();
    let mut iter = ArchiveIterator::from_read(file).unwrap();

    // TODO: remove usage of hashmap
    for content in &mut iter {
        match content {
            ArchiveContents::StartOfEntry(s, _) => {
                name = s;
            }
            ArchiveContents::DataChunk(mut v) => {
                match data.entry(name.clone()) {
                    Entry::Vacant(e) => {
                        e.insert(v);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().append(&mut v);
                    }
                };
            }
            ArchiveContents::EndOfEntry => {}
            ArchiveContents::Err(_) => {}
        }
    }
    iter.close().unwrap();

    for (k, v) in data {
        let mut sha1_hasher = Sha1::new();
        sha1_hasher.update(&v);
        ret.files.push((
            k,
            FileHashes {
                sha1: sha1_hasher
                    .finalize()
                    .as_slice()
                    .try_into()
                    .expect("Wrong length"),
                crc: crc32fast::hash(&v.as_slice()),
            },
        ));
    }

    ret
}
