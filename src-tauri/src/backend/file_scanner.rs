use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Read},
    path::Path,
    process::exit,
    time,
};

use log::{debug, error, info};
use terminal_size::terminal_size;
use walkdir::WalkDir;

use super::db_ops::DBOps;

pub struct FileScanner {
    pub db_conn: DBOps,
    pub dirty_files: Vec<String>,
    pub scanloc: String,
}

impl FileScanner {
    pub fn new(scanloc: &str, db_file: &str) -> Result<Self, Error> {
        //check path
        if Path::new(&scanloc).exists() {
            let tmpconf = match DBOps::new(db_file) {
                Ok(db_conn) => db_conn,
                Err(err) => {
                    error!("Müll: {err}");
                    exit(-1);
                }
            };
            Ok(FileScanner {
                db_conn: tmpconf,
                dirty_files: Vec::new(),
                scanloc: scanloc.to_owned(),
            })
        } else {
            Err(Error::new(ErrorKind::Other, "Invalid Path"))
        }
    }

    pub fn search_files(&mut self) {
        let mut analysed: i128 = 0;
        let mut skipped: i128 = 0;
        let big_tic = time::Instant::now();
        for file in WalkDir::new(&self.scanloc)
            .into_iter()
            .filter_map(|file| file.ok())
        {
            if (match file.metadata() {
                Ok(md) => md,
                Err(err) => {
                    error!("Failed getting file metadata: {}", err);
                    return;
                }
            })
            .is_file()
            {
                let hash = match self.create_hash(&file.path().display().to_string()) {
                    Some(hash) => {
                        analysed += 1;
                        hash
                    }
                    None => {
                        skipped += 1;
                        "".to_owned()
                    }
                };
                if match self.db_conn.hash_exists(hash.as_str()) {
                    Ok(exists) => {
                        self.dirty_files.push(file.path().display().to_string());
                        exists
                    }
                    Err(_) => false,
                } {
                    info!(
                        "Found hash {} for file {}",
                        hash,
                        file.path().display().to_string()
                    );
                }
            }
        }
        let big_toc = time::Instant::now();
        info!(
            "=> Analysed: {}, Skipped: {},  Infected: {}, Time: {} seconds",
            analysed,
            skipped,
            self.dirty_files.len(),
            big_toc.duration_since(big_tic).as_secs_f64()
        );
    }

    pub fn create_hash(&mut self, path: &str) -> Option<String> {
        let mut context = md5::Context::new();
        let mut buffer = [0; 1024];

        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                println!("{}", err);
                return None;
            }
        };
        let mut reader = BufReader::new(file);
        let mut it = 0;
        loop {
            let count = match reader.read(&mut buffer) {
                Ok(count) => count,
                Err(_) => return None,
            };

            if count == 0 {
                if it == 0 {
                    return None;
                }
                break;
            }
            context.consume(&buffer[..count]);
            it += 1;
        }
        let ret = format!("{:?}", context.compute());

        match terminal_size() {
            Some((width, _)) => {
                match width.0.checked_sub(path.len() as u16 + 2) {
                    Some(spacing) => {
                        debug!("\n {}{:>width$} ", path, ret, width = spacing as usize);
                    }
                    None => {}
                };
            }
            None => {}
        };
        Some(ret)
    }
}
