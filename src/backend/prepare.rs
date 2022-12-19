use std::time::Instant;
use reqwest::Error as HTTPError;
use std::env;
use std::error::Error;

const VIRUS_API: &str = "VIRUS_API";

pub struct DatabaseOperations {
    database_sql: DatabaseSQL,
    api_key: String,
}
/*
    Is an object that contains functions to keep the database up-to-date.
    It also has a function to interact with the Virusshare API, but thats completely optional.
*/
impl DatabaseOperations {
    pub fn new() -> Result<DatabaseOperations> {
        let database_sql = database_sql::DatabaseSQL::new().unwrap();
        let api_key = env::var(VIRUS_API)?;
        Ok(Self { api_key });
        Ok(Self { database_sql });
    }

    /*
        This is the public function that starts the database update. It uses
        the function db_is_updated() to check if the database is up-to-date
        and then uses the downlaod_files() function to update it
        if necessary
    */
    pub fn update_db(&self) {}

    /*
        Uses the download_file() function in a loop to download all new files it can find
        on the virusshare page. It keeps track of the file_nr and increments it on each loop.
        It goes on and on, until an Error 404 occurs.
    */
    fn downlaod_files() {}

    /*
        This function takes one file_nr as parameter. It then navigates to the following website:
        "https://virusshare.com/hashfiles/VirusShare_{}.md5", file_nr
        The file_nr must have 5 digits, for example:
        00045
        00123
        00002
        ...
        This function then reads the file line per line, and if the line doesn't start with an '#',
        it interpretes it as an hash and inserts it in the database. The file is not kept on disk,
        as it is basically useless. Ideally the function reads all lines online, without downloading
        the file.
    */
    fn download_file() {}

    /*
        Checks if the biggest file_nr in the database +1 can be found on the Virusshare website.
        If yes, we assume that the database is NOT updated, and the function returns false.
        If instead an error 404 Not Found occurs, we can assume that the Database is up-to-date.
    */
    fn db_is_updated() {}
}




/*

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
*/