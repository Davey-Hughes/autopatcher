mod archive;

use sha1::{Digest, Sha1};

pub fn calc_sha1(file_path: &str) -> Option<Vec<u8>> {
    let mut hasher = Sha1::new();

    // note: doesn't work on headerless NES roms
    let kind = match infer::get_from_path(file_path) {
        Err(_) => return None,
        Ok(k) => k.unwrap(),
    };

    match kind.mime_type() {
        "application/zip" => archive::read_zip(file_path, &mut hasher),
        "application/x-7z-compressed" => archive::read_7z(file_path, &mut hasher),
        "application/x-nintendo-nes-rom" => archive::read_unarchived(file_path, &mut hasher),
        _ => panic!("unsupported file"),
    };

    Some(hasher.finalize().to_vec())
}

pub fn string_from_sha1(input: Vec<u8>) -> String {
    input
        .iter()
        .map(|&v| format!("{:x}", v))
        .collect::<String>()
}
