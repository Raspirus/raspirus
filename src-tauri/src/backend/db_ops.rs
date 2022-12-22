use std::time;

use log::{debug, error, info, warn};
use reqwest::StatusCode;
use rusqlite::{params, Connection, Transaction};

#[allow(unused)]
pub struct DBOps {
    db_conn: Connection,
    db_file: String,
    file_nr: String,
}

impl DBOps {
    pub fn new(db_file: &str) -> Result<Self, rusqlite::Error> {
        let conn = match Connection::open(db_file) {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };

        let ret = DBOps {
            db_conn: conn,
            db_file: db_file.to_owned(),
            file_nr: "00000".to_owned(),
        };
        ret.init_table()?;
        Ok(ret)
    }

    pub fn init_table(&self) -> Result<(), rusqlite::Error> {
        info!("Creating table if not present...");
        match self.db_conn.execute(
            "CREATE TABLE IF NOT EXISTS signatures (
                                      hash varchar(32) PRIMARY KEY,
                                      file_nr varchar(5));",
            [],
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn update_db(&mut self) {
        info!("Updating database...");
        self.download_files();
        info!("Total hashes in DB: {}", self.count_hashes().unwrap_or(0));
    }

    fn download_files(&mut self) {
        if self.db_is_updated() {
            return;
        }

        info!("Database not up-to-date!");
        info!("Trying to fetch files...");
        loop {
            match Self::download_file(self.file_nr.as_str()) {
                Ok(hashes) => match hashes {
                    Some(hashes) => {
                        match self.insert_hashes(hashes) {
                            Ok(_) => {}
                            Err(err) => error!("{err}"),
                        };
                    }
                    None => break,
                },
                Err(err) => {
                    if err.status().unwrap_or(StatusCode::NO_CONTENT) == StatusCode::NOT_FOUND {
                        info!("No more files to download");
                        break;
                    }
                    warn!("Retrying download despite error: {}", err);
                }
            };
            if self
                .check_latest_file(&self.file_nr.clone())
                .unwrap_or(true)
            {
                info!("Done downloading");
                break;
            }
            self.increment_file_nr();
        }
    }

    fn download_file(file_nr: &str) -> Result<Option<Vec<(String, String)>>, reqwest::Error> {
        let url = format!(
            "https://virusshare.com/hashfiles/VirusShare_{}.md5",
            format!("{:0>5}", file_nr)
        );
        info!("Downloading {url}");
        let big_tic = time::Instant::now();
        let file = reqwest::blocking::get(&url)?;
        let size = file.content_length().unwrap_or(0);
        let file_as_string = match String::from_utf8(file.bytes()?.to_vec()) {
            Ok(file_as_string) => file_as_string,
            Err(err) => {
                warn!("Failed creating string from bytes: {}", err);
                return Ok(None);
            }
        };
        info!("Parsing file of size {} mb", size as f64 * 0.000001);
        let big_tuc = time::Instant::now();
        let lines = file_as_string.lines();
        if lines.clone().count() == 9 {
            println!("{file_as_string}");
            return Ok(None);
        }
        let mut hashes: Vec<(String, String)> = Vec::new();
        for l in lines {
            if !l.starts_with("#") {
                hashes.push((l.to_owned(), file_nr.clone().to_owned()));
            }
        }
        let big_toc = time::Instant::now();
        info!(
            "Downloaded file in {} seconds, Parsing took {} seconds",
            big_tuc.duration_since(big_tic).as_secs_f64(),
            big_toc.duration_since(big_tuc).as_secs_f64(),
        );
        Ok(Some(hashes))
    }

    fn increment_file_nr(&mut self) {
        self.file_nr = (match self.file_nr.parse::<i32>() {
            Ok(file_nr) => file_nr + 1,
            Err(_) => {
                error!("Failed incrementing file_nr; Retrying");
                return;
            }
        })
        .to_string();
    }

    fn db_is_updated(&mut self) -> bool {
        let file_nr = match self.get_latest_file_nr() {
            Ok(file_nr) => file_nr,
            Err(_) => return false,
        };

        match self.check_latest_file(&file_nr) {
            Ok(res) => res,
            Err(err) => {
                error!("Encountered error {:?}", err);
                false
            }
        }
    }

    fn check_latest_file(&self, file_nr: &str) -> Result<bool, reqwest::Error> {
        let file_nr = match file_nr.parse::<i32>() {
            Ok(file_nr) => file_nr + 1,
            Err(_) => {
                error!("Failed incrementing file_nr; Retrying");
                return Ok(false);
            }
        };
        let url = format!(
            "https://virusshare.com/hashfiles/VirusShare_{}.md5",
            format!("{:0>5}", file_nr)
        );
        info!("Trying to retrieve file from {url}");
        match reqwest::blocking::get(url) {
            Ok(resp) => {
                if resp.status() == StatusCode::NOT_FOUND {
                    info!("Database is up-to-date");
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn insert_hashes(&mut self, hashes: Vec<(String, String)>) -> Result<(), rusqlite::Error> {
        info!("Inserting File {}", hashes[0].1);
        let transact = match self.db_conn.transaction() {
            Ok(transact) => transact,
            Err(err) => return Err(err),
        };
        let big_tic = time::Instant::now();
        let mut inserted = 0;
        let mut skipped = 0;
        for (hash, file_nr) in hashes {
            match transact.execute(
                "INSERT INTO signatures(hash, file_nr) VALUES (?, ?)",
                [hash.clone(), file_nr.clone()],
            ) {
                Ok(_) => {
                    inserted += 1;
                    debug!("[File {file_nr}]: Inserted {}", hash)
                }
                Err(err) => {
                    skipped += 1;
                    debug!(
                        "Continuing after trying to insert hash and receiving: {}",
                        err
                    )
                }
            };
        }
        let big_toc = time::Instant::now();
        info!(
            "Inserted: {}, Skipped: {}, Time: {} seconds",
            inserted,
            skipped,
            big_toc.duration_since(big_tic).as_secs_f64()
        );
        transact.commit()?;
        Ok(())
    }

    pub fn _insert_hash(
        &self,
        transaction: &Transaction,
        hash_str: &str,
        file_nr: &str,
    ) -> Result<(), rusqlite::Error> {
        match transaction.execute(
            "INSERT INTO signatures(hash, file_nr) VALUES (?, ?)",
            [hash_str, file_nr],
        ) {
            Ok(_) => debug!("[File {file_nr}]: Inserted {}", hash_str),
            Err(err) => warn!(
                "Continuing after trying to insert hash and receiving: {}",
                err
            ),
        };
        Ok(())
    }

    pub fn hash_exists(&self, hash_str: &str) -> Result<bool, rusqlite::Error> {
        let mut stmt = self
            .db_conn
            .prepare("SELECT hash FROM signatures WHERE hash = ?")?;
        let hash: String = stmt.query_row(params![hash_str], |row| row.get(0))?;
        Ok(!hash.is_empty())
    }

    pub fn get_latest_file_nr(&mut self) -> Result<String, rusqlite::Error> {
        let mut stmt = self
            .db_conn
            .prepare("SELECT MAX(CAST(file_nr AS INTEGER)) FROM signatures;")?;
        let row = stmt.query_row([], |row| row.get(0));
        let file_nr: i32 = match row {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        self.file_nr = file_nr.to_string().clone();
        Ok(file_nr.to_string())
    }

    pub fn count_hashes(&self) -> Result<u64, rusqlite::Error> {
        let mut stmt = self.db_conn.prepare("SELECT COUNT(hash) FROM signatures")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count as u64)
    }

    pub fn _remove_hash(&self, hash_str: &str) -> Result<(), rusqlite::Error> {
        self.db_conn
            .execute("DELETE FROM signatures WHERE hash = ?", &[hash_str])?;
        Ok(())
    }
}
