mod archive;

use sha1::{Digest, Sha1};

pub fn calc_sha1(file_path: &str) -> Vec<u8> {
    let mut hasher = Sha1::new();

    // note: doesn't work on headerless NES roms
    let kind = infer::get_from_path(file_path)
        .expect("file read successfully")
        .expect("file type is known");

    match kind.mime_type() {
        "application/zip" => archive::read_zip(file_path, &mut hasher),
        "application/x-7z-compressed" => archive::read_7z(file_path, &mut hasher),
        "application/x-nintendo-nes-rom" => archive::read_unarchived(file_path, &mut hasher),
        _ => panic!("unsupported file"),
    };

    hasher.finalize().to_vec()
}

pub fn string_from_sha1(input: Vec<u8>) -> String {
    input
        .iter()
        .map(|&v| format!("{:x}", v))
        .collect::<String>()
}
