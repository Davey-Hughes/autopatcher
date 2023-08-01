use std::{
    fs,
    io::{self, Seek},
};

use compress_tools::list_archive_files;
use sha1::{digest::core_api::CoreWrapper, Sha1Core};
use zip::ZipArchive;

pub fn read_unarchived(file_path: &str, hasher: &mut CoreWrapper<Sha1Core>) {
    let mut file = fs::File::open(file_path).unwrap();
    io::copy(&mut file, hasher).unwrap();
}

pub fn read_zip(file_path: &str, hasher: &mut CoreWrapper<Sha1Core>) {
    let file = fs::File::open(file_path).unwrap();

    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..1 {
        let mut inner = archive.by_index(i).unwrap();
        io::copy(&mut inner, hasher).unwrap();
    }
}

pub fn read_7z(file_path: &str, hasher: &mut CoreWrapper<Sha1Core>) {
    let mut file = fs::File::open(file_path).unwrap();

    let list = list_archive_files(&mut file).unwrap();
    file.rewind().unwrap();

    let item = list[0].as_str();
    compress_tools::uncompress_archive_file(&file, hasher, item).unwrap();
}
