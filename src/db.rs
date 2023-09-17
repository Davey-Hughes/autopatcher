use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Executor;
use std::time::Duration;

pub async fn establish_connection(url: &str) -> SqlitePool {
    let locking_mode = "EXCLUSIVE";

    let pool = SqlitePoolOptions::new()
        .min_connections(1)
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(url)
        .await
        .unwrap();

    pool.execute(
        format!(
            "
            PRAGMA foreign_keys = ON;
            PRAGMA locking_mode = {};
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA temp_store = MEMORY;
            PRAGMA mmap_size = 30000000000;
            PRAGMA auto_vacuum = INCREMENTAL;
            ",
            locking_mode
        )
        .as_str(),
    )
    .await
    .expect("Failed to setup the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}
