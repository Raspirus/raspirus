use std::time;

use log::{debug, error, info, warn};
use reqwest::StatusCode;
use rusqlite::{params, Connection};

use tauri::Manager;

#[allow(unused)]
pub struct DBOps {
    db_conn: Connection,
    db_file: String,
    file_nr: i32,
    total_files: i32,
    /// Tauri window for events
    tauri_window: Option<tauri::Window>,
}
#[derive(Clone, serde::Serialize)]
struct TauriEvent {
    message: String,
}

impl DBOps {
    /// Returns a new `DBOps` struct with a connection to the specified database file
    /// and initializes the table if it does not exist.
    ///
    /// # Arguments
    ///
    /// * `db_file` - The path to the database file.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let db_ops = DBOps::new("signatures.db").unwrap();
    /// assert_eq!(db_ops.db_conn, Connection::open("signatures.db").unwrap());
    /// ```
    pub fn new(db_file: &str, t_win: Option<tauri::Window>) -> Result<Self, rusqlite::Error> {
        let conn = match Connection::open(db_file) {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };
        info!("New database connection at: {}", db_file);

        let ret = DBOps {
            db_conn: conn,
            db_file: db_file.to_owned(),
            file_nr: 0,
            total_files: 0,
            tauri_window: t_win,
        };
        ret.init_table()?;
        Ok(ret)
    }

    /// Initializes the `signatures` table if it does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let db_ops = DBOps::new("signatures.db").unwrap();
    /// assert!(db_ops.init_table().is_ok());
    /// ```
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

    /// Updates the database by downloading any missing files and inserting their hashes into the `signatures` table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let mut db_ops = DBOps::new("signatures.db").unwrap();
    /// db_ops.update_db();
    /// ```
    pub fn update_db(&mut self) -> Result<u64, rusqlite::Error> {
        info!("Updating database...");
        let web_files = self.get_diff_file();

        if let Some(last_element) = web_files.last() {
            self.total_files = *last_element;
            info!("Database not up-to-date!");
            info!("Downloading {} file(s)", web_files.len());
            self.download_files(web_files);
        } else if let Some(window) = &self.tauri_window {
            if window.emit_all("progress", TauriEvent { message: "100".to_string(), },).is_err() {
                error!("Couldn't send progress update to frontend");
            }
        } else {
            warn!("tauri_window is None, won't send progress to frontend");
        }
        info!("Total hashes in DB: {}", self.count_hashes().unwrap_or(0));
        Ok(self.count_hashes().unwrap_or(0))
    }

    /// Downloads the specified files and inserts their hashes into the signatures table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let mut db_ops = DBOps::new("signatures.db").unwrap();
    /// db_ops.download_files(vec![1, 2, 3]);
    /// ```
    pub fn download_files(&mut self, files: Vec<i32>) {
        if files.is_empty() {
            return;
        }

        info!("Trying to fetch files...");
        let mut _retry = false;
        let mut i = 0;
        let last_percentage: &mut f64 = &mut -1.0;
        while i < files.len() {
            _retry = false;
            match Self::download_file(files[i]) {
                Ok(hashes) => match hashes {
                    Some(hashes) => {
                        
                        if Self::calculate_progress(self, last_percentage, i.try_into().expect("Issue with scanned size"), self.total_files).is_err() {
                            warn!("Progress calculation is broken");
                        }
                        
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

    /// Downloads the specified file and returns its content and file number as a tuple in the form of (file_nr, content).
    /// Returns None if the file does not exist or if there was an error creating a String from the file's bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let file = DBOps::download_file(1).unwrap();
    /// assert!(file.is_some());
    /// ```
    pub fn download_file(file_nr: i32) -> Result<Option<Vec<(String, String)>>, reqwest::Error> {
        let url = format!("https://virusshare.com/hashfiles/VirusShare_{:0>5}.md5", file_nr);
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
            info!("{file_as_string}");
            return Ok(None);
        }
        let mut hashes: Vec<(String, String)> = Vec::new();
        for l in lines {
            if !l.starts_with('#') {
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

    /// Inserts the given hashes into the signatures table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let mut db_ops = DBOps::new("signatures.db").unwrap();
    /// db_ops.insert_hashes(vec![("abcdef".to_owned(), "1".to_owned())]).unwrap();
    /// ```
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
                    // debug!("[File {file_nr}]: Inserted {}", hash)
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

    /// Returns true or false depending on if the given hash gets found in the database
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let db_ops = DBOps::new("signatures.db").unwrap();
    /// assert_eq!(db_ops.hash_exists("abcd1234").unwrap(), false);
    /// ```
    pub fn hash_exists(&self, hash_str: &str) -> Result<Option<bool>, rusqlite::Error> {
        info!("Now scanning: {}", hash_str);
    
        let mut stmt = self.db_conn.prepare("SELECT COUNT(*) FROM signatures WHERE hash = ?")?;
        let count: i64 = stmt.query_row(params![hash_str], |row| row.get(0))?;
    
        Ok(Some(count > 0))
    }


    /// Returns the number of hashes in the `signatures` table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let db_ops = DBOps::new("signatures.db").unwrap();
    /// assert_eq!(db_ops.count_hashes().unwrap(), 0);
    /// ```
    pub fn count_hashes(&self) -> Result<u64, rusqlite::Error> {
        let mut stmt = self.db_conn.prepare("SELECT COUNT(hash) FROM signatures")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count as u64)
    }

    /// Removes the specified hash from the `signatures` table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let db_ops = DBOps::new("signatures.db").unwrap();
    /// assert!(db_ops._remove_hash("abcd1234").is_ok());
    /// ```
    pub fn _remove_hash(&self, hash_str: &str) -> Result<(), rusqlite::Error> {
        self.db_conn
            .execute("DELETE FROM signatures WHERE hash = ?", [hash_str])?;
        Ok(())
    }

    /// Returns the file numbers of the files that are present online.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let db_ops = DBOps::new("signatures.db").unwrap();
    /// assert!(db_ops.get_file_list() > 0);
    /// ```
    pub fn get_file_list(&self) -> i32 {
        let mut curr_fn = 0;
        let mut err_retry = false;
        // Loops in steps of +10
        loop {
            // If file exists
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

    /// Returns whether the file with the specified file number exists online.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let db_ops = DBOps::new("signatures.db").unwrap();
    /// assert!(db_ops.file_exists(123).unwrap_or(false));
    /// ```
    pub fn file_exists(file_nr: i32) -> Result<bool, reqwest::Error> {
        let url = format!("https://virusshare.com/hashfiles/VirusShare_{:0>5}.md5", file_nr);
        let client = reqwest::blocking::Client::new();
        info!("Checking if file {file_nr} exists...");
        let code = client.get(url).send()?;
        if code.status() == StatusCode::NOT_FOUND {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Returns a vector of the file numbers of the files that are present in the `signatures` table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let db_ops = DBOps::new("signatures.db").unwrap();
    /// assert!(db_ops.get_db_files().is_some());
    /// ```
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

    /// Returns a list of file numbers for which there are no corresponding hashes in the signatures table.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusqlite::Connection;
    /// use virus_scanner::backend::db_ops::DBOps;
    /// let db_ops = DBOps::new("signatures.db").unwrap();
    /// assert!(db_ops.get_diff_file().len() >= 0);
    /// ```
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

    fn calculate_progress(&mut self, last_percentage: &mut f64, scanned_size: i32, files_size: i32) -> Result<f64, String> {
        info!("Called calculate_perc with last_p: {}, scanned_size: {} and file_s: {}", last_percentage, scanned_size, files_size);
        let scanned_percentage = (scanned_size as f64 / files_size as f64 * 100.0).round();
        info!("Updated: {}%", scanned_percentage);
        if scanned_percentage != *last_percentage {
            if let Some(window) = &self.tauri_window {
                if window
                    .emit_all(
                        "progress",
                        TauriEvent {
                            message: scanned_percentage.to_string(),
                        },
                    )
                    .is_err()
                {
                    return Err("Couldn't send progress update to frontend".to_string());
                }
            } else {
                return Err("tauri_window is None".to_string());
            }
            
            *last_percentage = scanned_percentage;
        }
        Ok(scanned_percentage)
    }
}
