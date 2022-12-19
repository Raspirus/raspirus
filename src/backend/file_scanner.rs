use async_std::fs;
use async_std::io::BufReader;
use async_std::path::Path;
use async_std::prelude::*;
use async_std::task;
use std::error::Error;
use std::time::Instant;
use xxhash::xxh64;

pub struct FileScanner {
    pub amount_of_files: u64,
    pub hash_db: DatabaseSQL,
    pub dirty_files: Vec<String>,
    pub path: String,
}

impl FileScanner {
    pub fn new(path: String, db_location: String) -> Result<FileScanner> {
        let hash_db = database_sql::DatabaseSQL::new().unwrap();

        if Path::new(&path).exists() {
            Ok(FileScanner {
                amount_of_files: 0,
                hash_db,
                dirty_files: Vec::new(),
                path,
            })
        } else {
            Err(rusqlite::Error::InvalidPath)
        }
    }

    async fn search_files(self, directory:str);

    async fn calculate_xxhash(file_path:str);

    async fn scan_files(self);

    pub fn start_scanner(self);

}