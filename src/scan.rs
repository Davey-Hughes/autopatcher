mod hash;

use crossbeam_channel::{unbounded, Sender};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use walkdir::WalkDir;

static NUM_THREADS: usize = 8;

fn walker(start_path: PathBuf, tx: Sender<PathBuf>) {
    for entry in WalkDir::new(start_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.into_path();
        // let sec = entry.metadata().unwrap().modified().unwrap();

        tx.send(f_name).unwrap();
    }

    drop(tx);
}

fn hash_file(f_name: PathBuf) -> Option<String> {
    match hash::calc_sha1(f_name.to_str().unwrap()) {
        None => None,
        Some(h) => {
            let s = hash::string_from_sha1(h);
            Some(s)
        }
    }
}

pub fn scan(start_path: PathBuf) {
    let roms = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    let (tx, rx) = unbounded();
    let mut threads = vec![];
    let _ = thread::spawn(|| walker(start_path, tx));

    for _ in 0..NUM_THREADS - 1 {
        let rx_clone = rx.clone();
        let roms_clone = roms.clone();
        threads.push(thread::spawn(move || {
            while let Ok(f_name) = rx_clone.recv() {
                let sha1 = match hash_file(f_name.clone()) {
                    None => continue,
                    Some(s) => s,
                };
                let mut roms = roms_clone.lock().unwrap();
                roms.insert(sha1.to_string(), f_name.display().to_string());
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    for (k, v) in roms.lock().unwrap().iter() {
        println!("{}: {}", v, k);
    }
}
