use sqlx::pool::Pool;
use sqlx::Sqlite;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::hash;

#[derive(Debug)]
pub struct FileHashes {
    pub crc: u32,
    pub sha1: [u8; 20],
}

#[derive(Debug, Default)]
pub struct HashedArchive {
    pub f_name: String,
    pub files: Vec<(String, FileHashes)>,
}

async fn walker(start_path: PathBuf) {
    let tasks: Vec<_> = WalkDir::new(start_path.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| {
            e.ok().map(|entry| {
                let task_name = format!(
                    "hash: {:?}",
                    entry
                        .clone()
                        .into_path()
                        .to_str()
                        .unwrap()
                        .replace(start_path.to_str().unwrap(), "")
                );
                tokio::task::Builder::new()
                    .name(task_name.as_str())
                    .spawn(async move {
                        hash_worker(entry.into_path()).await;
                    })
                    .unwrap()
            })
        })
        .collect();

    for task in tasks {
        task.await.unwrap();
    }
}

async fn hash_worker(f_name: PathBuf) {
    let file_hashes = match hash::calc_sha1(f_name.to_str().unwrap()) {
        None => return,
        Some(s) => s,
    };

    printer(file_hashes)
}

fn printer(archive: HashedArchive) {
    println!("{}", archive.f_name);
    for (name, h) in archive.files {
        println!("\t{}", name);
        println!("\t\tcrc: {:x}", h.crc);
        println!("\t\tsha1: {}", hash::hex_string_from_slice(&h.sha1));
    }
}

pub async fn scan(start_path: PathBuf, _pool: Pool<Sqlite>) {
    walker(start_path).await;
}
