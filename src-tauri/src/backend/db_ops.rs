use std::time;

use log::{debug, error, info, warn};
use reqwest::StatusCode;
use rusqlite::{params, Connection};

#[allow(unused)]
pub struct DBOps {
    db_conn: Connection,
    db_file: String,
    file_nr: i32,
}

impl DBOps {
    //returns new DBOps struct
    pub fn new(db_file: &str) -> Result<Self, rusqlite::Error> {
        let conn = match Connection::open(db_file) {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };

        let ret = DBOps {
            db_conn: conn,
            db_file: db_file.to_owned(),
            file_nr: 0,
        };
        ret.init_table()?;
        Ok(ret)
    }
    //initializes table if it doesnt exist
    pub fn init_table(&self) -> Result<(), rusqlite::Error> {
        info!("Creating table if not present...");
        match self.db_conn.execute(
            "CREATE TABLE IF NOT EXISTS signatures (
                                      hash varchar(32) PRIMARY KEY,
                                      file_nr varchar(5))",
            [],
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    //updates the db
    pub fn update_db(&mut self) {
        info!("Updating database...");
        let web_files = self.get_diff_file();
        if web_files.len() > 0 {
            info!("Database not up-to-date!");
            info!("Downloading {} file(s)", web_files.len());
            self.download_files(web_files);
        }
        info!("Total hashes in DB: {}", self.count_hashes().unwrap_or(0));
    }
    //downloads files and inserts them into db
    pub fn download_files(&mut self, files: Vec<i32>) {
        if files.len() == 0 {
            return;
        }

        info!("Trying to fetch files...");
        let mut _retry = false;
        let mut i = 0;
        while i < files.len() {
            _retry = false;
            match Self::download_file(files[i]) {
                Ok(hashes) => match hashes {
                    Some(hashes) => {
                        match self.insert_hashes(hashes) {
                            Ok(_) => i += 1,
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
                    _retry = true;
                }
            };
        }
        info!("Done updating DB");
    }
    //downloads file and returns content and file number
    pub fn download_file(file_nr: i32) -> Result<Option<Vec<(String, String)>>, reqwest::Error> {
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
                hashes.push((l.to_owned(), format!("{}", file_nr.clone())));
            }
        }
        let big_toc = time::Instant::now();
        info!(
            "=> Downloaded file in {} seconds, Parsing took {} seconds",
            big_tuc.duration_since(big_tic).as_secs_f64(),
            big_toc.duration_since(big_tuc).as_secs_f64(),
        );
        Ok(Some(hashes))
    }
    //inserts hash and file number into db
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
            "=> Inserted: {}, Skipped: {}, Time: {} seconds",
            inserted,
            skipped,
            big_toc.duration_since(big_tic).as_secs_f64()
        );
        transact.commit()?;
        Ok(())
    }
    //checks if hash exists in db
    pub fn hash_exists(&self, hash_str: &str) -> Result<bool, rusqlite::Error> {
        let mut stmt = self
            .db_conn
            .prepare("SELECT hash FROM signatures WHERE hash = ?")?;
        let hash: String = stmt.query_row(params![hash_str], |row| row.get(0))?;
        Ok(!hash.is_empty())
    }
    //counts hashes in db
    pub fn count_hashes(&self) -> Result<u64, rusqlite::Error> {
        let mut stmt = self.db_conn.prepare("SELECT COUNT(hash) FROM signatures")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count as u64)
    }
    //removes hash from db
    pub fn _remove_hash(&self, hash_str: &str) -> Result<(), rusqlite::Error> {
        self.db_conn
            .execute("DELETE FROM signatures WHERE hash = ?", &[hash_str])?;
        Ok(())
    }
    //returns array of file numbers present online
    pub fn get_file_list(&self) -> i32 {
        let mut curr_fn = 0;
        let mut err_retry = false;
        //+10 loop
        loop {
            //if file exists
            if match Self::file_exists(curr_fn) {
                Ok(val) => val,
                Err(err) => {
                    warn!("Retrying because of error: {err}");
                    err_retry = true;
                    false
                }
            } {
                curr_fn += 10;
            } else {
                //if err is true retry otherwise break
                if err_retry {
                    err_retry = false;
                } else {
                    break;
                }
            }
        }
        curr_fn -= 1;
        //-1 loop
        loop {
            //if file exists
            if !match Self::file_exists(curr_fn) {
                Ok(val) => val,
                Err(err) => {
                    warn!("Retrying because of error: {err}");
                    err_retry = true;
                    false
                }
            } {
                curr_fn -= 1;
            } else {
                //if err is true retry otherwise break
                if err_retry {
                    err_retry = false;
                } else {
                    break;
                }
            }
        }

        curr_fn
    }
    //checks if file exists online
    pub fn file_exists(file_nr: i32) -> Result<bool, reqwest::Error> {
        let url = format!(
            "https://virusshare.com/hashfiles/VirusShare_{}.md5",
            format!("{:0>5}", file_nr)
        );
        let client = reqwest::blocking::Client::new();
        info!("Checking if file {file_nr} exists...");
        let code = client.get(url).send()?;
        if code.status() == StatusCode::NOT_FOUND {
            Ok(false)
        } else {
            Ok(true)
        }
    }
    //returns array of file numbers present in db
    pub fn get_db_files(&self) -> Option<Vec<i32>> {
        let mut stmt = match self
            .db_conn
            .prepare("SELECT DISTINCT file_nr FROM signatures")
        {
            Ok(stmt) => stmt,
            Err(err) => {
                warn!("Failed preparing statement: {err}");
                return None;
            }
        };
        let mut rows = match stmt.query(params![]) {
            Ok(stmt) => stmt,
            Err(err) => {
                warn!("Failed preparing statement: {err}");
                return None;
            }
        };

        let mut file_nr_values = Vec::new();
        loop {
            match rows.next() {
                Ok(row) => {
                    let tmp = match row {
                        Some(row) => row,
                        None => break,
                    };
                    let value = match tmp.get(0) {
                        Ok(value) => {
                            let a: String = value;
                            a.parse::<i32>().unwrap_or(0)
                        }
                        Err(err) => {
                            warn!("Failed getting value: {err}");
                            break;
                        }
                    };
                    file_nr_values.push(value);
                }
                Err(err) => {
                    warn!("Failed getting row: {err}");
                    break;
                }
            }
        }
        Some(file_nr_values)
    }
    //returns array of file numbers present online but not in db
    pub fn get_diff_file(&self) -> Vec<i32> {
        let mut web_files: Vec<i32> = (0..=self.get_file_list()).collect();
        //let mut web_files: Vec<i32> = (0..=20).collect();
        let db_files = match self.get_db_files() {
            Some(db_files) => db_files,
            None => Vec::new(),
        };
        web_files.retain(|x| !db_files.contains(x));
        web_files
    }
}
