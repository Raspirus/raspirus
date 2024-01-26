use std::time;
use log::{debug, error, info, warn};
use reqwest::StatusCode;
use rusqlite::{params, Connection};
use tauri::Manager;

/// The `DBOps` struct holds a connection to the database and provides functions to interact with it.
#[allow(unused)]
pub struct DBOps {
    /// Connection to the database
    db_conn: Connection,
    /// Path to the database file
    db_file: String,
    /// Current file number. Used for progress calculation
    file_nr: i32,
    /// Total amount of files. Used for progress calculation
    total_files: i32,
    /// Tauri window for events, aka. progress updates to the GUI
    /// Can be None if the backend is not running in Tauri or without a window
    tauri_window: Option<tauri::Window>,
}

/// The `TauriEvent` struct is used to send events to the frontend.
/// It basically sends serialized JSON to the frontend.
#[derive(Clone, serde::Serialize)]
struct TauriEvent {
    message: String,
}

/// The `DBOps` struct holds a connection to the database and provides functions to interact with it.
/// This is the implementation of the `DBOps` struct.
impl DBOps {
    /// Returns a new `DBOps` struct with a connection to the specified database file
    /// and initializes the table if it does not exist.
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
    /// Returns the amount of hashes in the `signatures` table.
    /// Furthermore, if the window is not None, it will send progress updates to the frontend.
    pub fn update_db(&mut self) -> Result<u64, rusqlite::Error> {
        info!("Updating database...");
        let web_files = self.get_diff_file();
        // If there are files to download
        if let Some(last_element) = web_files.last() {
            self.total_files = *last_element;
            info!("Database not up-to-date!");
            info!("Downloading {} file(s)", web_files.len());
            self.download_files(web_files);
        } else if let Some(window) = &self.tauri_window {
            // To prevent a bug where the progress bar would not reach 100% if the db is already up-to-date
            // we send a progress update of 100% here
            if window
                .emit_all(
                    "progress",
                    TauriEvent {
                        message: "100".to_string(),
                    },
                )
                .is_err()
            {
                error!("Couldn't send progress update to frontend");
            }
        } else {
            warn!("tauri_window is None, won't send progress to frontend");
        }
        info!("Total hashes in DB: {}", self.count_hashes().unwrap_or(0));
        // Return the amount of hashes in the signatures table
        Ok(self.count_hashes().unwrap_or(0))
    }

    /// Downloads the specified files and inserts their hashes into the signatures table.
    /// If any download fails, it will retry it by changing the boolean to true.
    pub fn download_files(&mut self, files: Vec<i32>) {
        // No need to download empty files
        if files.is_empty() {
            return;
        }

        info!("Trying to fetch files...");
        let mut _retry = false;
        let mut i = 0;
        let last_percentage: &mut f64 = &mut -1.0;
        // Loop through all files and call download_file on them to download them
        while i < files.len() {
            _retry = false;
            match Self::download_file(files[i]) {
                Ok(hashes) => match hashes {
                    Some(hashes) => {
                        // If the progress calculation fails, we log it
                        // This function is responsible for sending status updates to the frontend
                        // Its not critical, so we just log it and continue
                        if Self::calculate_progress(
                            self,
                            last_percentage,
                            i.try_into().expect("Issue with scanned size"),
                            self.total_files,
                        )
                        .is_err()
                        {
                            warn!("Progress calculation is broken");
                        }
                        // Here we insert the hashes we retrieved from the file into the database
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
    pub fn download_file(file_nr: i32) -> Result<Option<Vec<(String, String)>>, reqwest::Error> {
        // This is the URL we download the files from
        // TODO: Change this to the GitHub repo
        let url = format!(
            "https://virusshare.com/hashfiles/VirusShare_{:0>5}.md5",
            file_nr
        );
        info!("Downloading {url}");
        // We keep track of te time it takes to download and parse the file
        // This is used for logging reasons
        let big_tic = time::Instant::now();
        let file = reqwest::blocking::get(&url)?;
        let size = file.content_length().unwrap_or(0);
        // The file is in bytes, so we need to convert it to a string
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
        // If the file has only 9 lines, we assume it is empty
        if lines.clone().count() == 9 {
            info!("{file_as_string}");
            return Ok(None);
        }
        // We extract the hashes from the file and put them into a vector of tuples
        let mut hashes: Vec<(String, String)> = Vec::new();
        for l in lines {
            // We can ignore comments, aka strings that start with a #
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
        // Return the hashes we extracted from the file
        Ok(Some(hashes))
    }

    /// Inserts the given hashes into the signatures table.
    /// We do this using a simple for loop, because rusqlite does not support bulk inserts.
    /// Returns an error if the insert fails.
    pub fn insert_hashes(&mut self, hashes: Vec<(String, String)>) -> Result<(), rusqlite::Error> {
        info!("Inserting File {}", hashes[0].1);
        // We use transactions to speed up the process and to prevent the database from locking up
        let transact = match self.db_conn.transaction() {
            Ok(transact) => transact,
            Err(err) => return Err(err),
        };
        let big_tic = time::Instant::now();
        let mut inserted = 0;
        let mut skipped = 0;
        // We use a for loop to insert the hashes into the database
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
        // We finally close the transaction
        transact.commit()?;
        Ok(())
    }

    /// Returns true or false depending on if the given hash gets found in the database
    pub fn hash_exists(&self, hash_str: &str) -> Result<bool, rusqlite::Error> {
        info!("Now scanning: {}", hash_str);
        // Uses a simple SELECT command to check if the hash is in the database
        let mut stmt = self
            .db_conn
            .prepare("SELECT COUNT(*) FROM signatures WHERE hash = ?")?;
        let count: i64 = stmt.query_row(params![hash_str], |row| row.get(0))?;
        Ok(count > 0)
    }

    /// Returns the number of hashes in the `signatures` table.
    /// Executes a simple SELECT command and returns the count.
    pub fn count_hashes(&self) -> Result<u64, rusqlite::Error> {
        let mut stmt = self.db_conn.prepare("SELECT COUNT(hash) FROM signatures")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count as u64)
    }

    /// Removes the specified hash from the `signatures` table.
    /// Executes a simple DELETE command.
    pub fn _remove_hash(&self, hash_str: &str) -> Result<(), rusqlite::Error> {
        self.db_conn
            .execute("DELETE FROM signatures WHERE hash = ?", [hash_str])?;
        Ok(())
    }

    /// Returns the file numbers of the files that are present online.
    /// This is used to determine the amount of files that need to be downloaded.
    /// It does this by looping through the files in steps of 10 and checking if they exist.
    /// If the file does not exist, it will retry the request or go back one step
    /// at a time until it finds one that exists. That way we can find the last file
    pub fn get_file_list(&self) -> i32 {
        // TODO: We might want to adapt this for the GitHub repository
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
    pub fn file_exists(file_nr: i32) -> Result<bool, reqwest::Error> {
        // TODO: We might want to adapt this for the GitHub repository
        let url = format!(
            "https://virusshare.com/hashfiles/VirusShare_{:0>5}.md5",
            file_nr
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

    /// Returns a vector of the file numbers of the files that are present in the `signatures` table.
    /// This is used to check at what point the database is up-to-date.
    /// If the last file nuber in the database is the same as the last file number online,
    /// the database is up-to-date.
    pub fn get_db_files(&self) -> Option<Vec<i32>> {
        // Executes a simple SELECT command and returns the file numbers
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
        // For each result, we add the file number to the vector
        let mut rows = match stmt.query(params![]) {
            Ok(stmt) => stmt,
            Err(err) => {
                warn!("Failed preparing statement: {err}");
                return None;
            }
        };

        let mut file_nr_values = Vec::new();
        // We loop through the rows and add the file numbers to the vector
        // If we encounter an error, we break the loop
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
        // Return the vector with the file numbers
        Some(file_nr_values)
    }

    /// Returns a list of file numbers for which there are no corresponding hashes in the signatures table.
    /// This is used to determine which files need to be downloaded.
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

    /// Calculates the progress and sends it to the frontend.
    /// It does this by comparing the last percentage with the current percentage.
    /// If they are not the same, it sends an event to the frontend.
    /// If the window is None, it will return an error.
    fn calculate_progress(
        &mut self,
        last_percentage: &mut f64,
        scanned_size: i32,
        files_size: i32,
    ) -> Result<f64, String> {
        // Keeps track of the current percentage and file number
        debug!(
            "Called calculate_perc with last_p: {}, scanned_size: {} and file_s: {}",
            last_percentage, scanned_size, files_size
        );
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
        // Update the scanned percentage
        Ok(scanned_percentage)
    }
}
