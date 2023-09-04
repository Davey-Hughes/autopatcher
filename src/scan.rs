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

        tx.send(f_name).unwrap();
    }

    drop(tx);
}

pub fn scan(start_path: PathBuf) {
    let roms = Arc::new(Mutex::new(HashMap::<String, Vec<(String, String)>>::new()));
    let (tx, rx) = unbounded();
    let mut threads = vec![];
    let _ = thread::spawn(|| walker(start_path, tx));

    for _ in 0..NUM_THREADS - 1 {
        let rx_clone = rx.clone();
        let roms_clone = roms.clone();
        threads.push(thread::spawn(move || {
            while let Ok(f_name) = rx_clone.recv() {
                let file_hashes = match hash::calc_sha1(f_name.to_str().unwrap()) {
                    None => continue,
                    Some(s) => s,
                };

                let mut roms = roms_clone.lock().unwrap();
                roms.insert(f_name.display().to_string(), file_hashes);
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    for (k, v) in roms.lock().unwrap().iter() {
        println!("{}:", k);
        for (name, h) in v {
            println!("\t{}: {}", name, h);
        }
    }
}
