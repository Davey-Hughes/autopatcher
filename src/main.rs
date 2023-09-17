mod db;
mod hash;
mod scan;

use db::establish_connection;
use sqlx::{Pool, Sqlite};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;

async fn setup_db() -> Pool<Sqlite> {
    let data_directory = match env::var("AUTOPATCHER_DATA_DIRECTORY") {
        Ok(data_directory) => PathBuf::from(data_directory),
        Err(_) => dirs::data_dir()
            .map(PathBuf::from)
            .unwrap()
            .join("autopatcher"),
    };

    if !data_directory.is_dir() {
        fs::create_dir_all(&data_directory).unwrap();
    }

    let db_file = data_directory.join("autopatcher.db");
    fs::File::create(&db_file).unwrap();

    establish_connection(db_file.as_os_str().to_str().unwrap()).await
}

#[cfg(debug_assertions)]
fn setup_tracing() {
    console_subscriber::init();
    std::env::set_var("RUST_LOG", "tokio=trace,runtime=trace,debug");
}

#[cfg(not(debug_assertions))]
fn setup_tracing() {}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        process::exit(-1);
    }

    setup_tracing();

    let start_path = std::path::Path::new(&*args[1]);
    scan::scan(start_path.to_path_buf(), setup_db().await).await;

    Ok(())
}
