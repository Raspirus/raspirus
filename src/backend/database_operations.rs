mod database_sql;
use std::time::Instant;
use reqwest::Error as HTTPError;
use std::env;
use std::error::Error;

const VIRUS_API: &str = "VIRUS_API";

pub struct DatabaseOperations {
    database_sql: DatabaseSQL,
    api_key: String,
}

impl DatabaseOperations {
    pub fn new() -> Result<DatabaseOperations> {
        let database_sql = database_sql::DatabaseSQL::new().unwrap();
        let api_key = env::var(VIRUS_API)?;
        Ok(Self { api_key });
        Ok(Self { database_sql });
    }

    pub fn update_db(&self) {
        println!("Updating database...");
        let big_tic = time::Instant::now();
        self.download_files();
        let big_toc = time::Instant::now();
        println!("Executed in {} seconds", big_toc.duration_since(big_tic).as_secs_f64());
        println!("Total hashes in DB: {}", self.database_sql.count_hashes().unwrap());
    }

    fn download_files(&self) {
        if !self.db_is_updated() {
            println!("Database not up-to-date!");
            let file_nr = self.database_sql.get_latest_file_nr();
    
            loop {
                match download_file(file_nr) {
                    Ok(hashes) => {
                        self.database_sql.insert_hashes(hashes);
                        println!("DB updated with new hashes");
                        file_nr = increment_file_nr(file_nr);
                    },
                    Err(err) => {
                        if err.code == 404 {
                            println!("No more files to download");
                            break;
                        }
                        println!("ERROR: {}", err);
                        break;
                    }
                }
            }
        } else {
            println!("DB already up-to-date");
        }
    }
    
    fn download_file(file_nr: String) -> Result<Vec<(String, String)>, HTTPError> {
        let tic = Instant::now();
        let filename = format!("VirusShare_{}.md5", file_nr);
        let url = format!("https://virusshare.com/hashfiles/{}", filename);
        let file = reqwest::blocking::get(&url)?;
        let hashes = file.bytes().map(|b| b.unwrap()).filter(|&b| b != b'#').map(|b| char::from(b)).collect::<String>()
            .split('\n').map(|line| (line.to_string(), file_nr.to_string())).collect();
        println!("Downloaded {} in {} seconds", filename, tic.elapsed().as_secs_f64());
        Ok(hashes)
    }
    
    fn increment_file_nr(file_nr: String) -> String {
        let file_nr: u32 = file_nr.parse().unwrap();
        format!("{:05}", file_nr + 1)
    }

    fn db_is_updated(&self) -> Result<bool> {
        let file_nr = self.get_latest_file_nr()?;
        if file_nr == "None" {
            return Ok(false);
        }
    
        match self._check_latest_file(file_nr) {
            Ok(_) => Ok(false),
            Err(err) => {
                if err.code() == 404 {
                    println!("Database is up-to-date");
                    return Ok(true);
                }
                return Err(err);
            }
        }
    }
    
    fn check_latest_file(&self, file_nr: String) -> Result<()> {
        let file_nr: i32 = file_nr.parse().unwrap() + 1;
        let file_nr = format!("{:05}", file_nr);
        let filename = format!("VirusShare_{}.md5", file_nr);
        let url = format!("https://virusshare.com/hashfiles/{}", filename);
        reqwest::get(url)?;
        Ok(())
    }

}