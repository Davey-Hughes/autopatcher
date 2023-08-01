use sha1::{Digest, Sha1};
use std::{fs, io, str};
use zip::ZipArchive;

pub fn calc_sha1(file: &str) -> Vec<u8> {
    let mut file = fs::File::open(file).unwrap();
    let mut hasher = Sha1::new();

    io::copy(&mut file, &mut hasher).unwrap();
    hasher.finalize().to_vec()
}

pub fn calc_sha1_zip(file: &str) -> Vec<u8> {
    let file = fs::File::open(file).unwrap();
    let mut hasher = Sha1::new();

    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut inner = archive.by_index(i).unwrap();
        io::copy(&mut inner, &mut hasher).unwrap();
    }

    hasher.finalize().to_vec()
}

pub fn string_from_sha1(input: Vec<u8>) -> String {
    input
        .iter()
        .map(|&v| format!("{:x}", v))
        .collect::<String>()
}
