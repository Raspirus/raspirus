extern crate rusqlite;
use rusqlite::{Connection, Result, params};

pub struct DatabaseSQL {
    db_connection: Connection,
}
/*
    Is an object containing all SQL operations for the database
*/
impl DatabaseSQL {
    pub fn new() -> Result<DatabaseSQL> {
        let conn = Connection::open("database/signatures.db")?;
        let db = DatabaseSQL { db_connection: conn };
        db.init_table()?;
        Ok(db)
    }

    /* 
        Initializes the database table if it doesn't already exist
        Ideally it also creates the signatures.db file if it doesn't exist 
    */
    fn init_table(&self) -> Result<()> {
        println!("Creating table...");
        self.db_connection.execute("CREATE TABLE IF NOT EXISTS signatures (
                                      hash varchar(32) PRIMARY KEY,
                                      file_nr varchar(5)
                                      );", rusqlite::NO_PARAMS)?;
        Ok(())
    }

    /* Inserts a single hash (slow) */
    pub fn insert_hash(&self, hash_str: &str, file_nr: &str) -> Result<()> {
        self.db_connection.execute("INSERT INTO signatures(hash, file_nr) VALUES (?, ?)", &[hash_str, file_nr])?;
        Ok(())
    }

    /* Inserts many hashes at once (faster) 
        SQL may warn about an Hash already existing, thats normal and can safely be ignored
    */
    pub fn insert_hashes(&self, hashes: Vec<(&str, &str)>) -> Result<()> {
        let mut batch = self.db_connection.prepare_cached("INSERT INTO signatures(hash, file_nr) VALUES (?, ?)")?;
        for (hash, file_nr) in hashes {
            batch.execute(&[hash, file_nr])?;
        }
        Ok(())
    }

    /* Uses the SELECT command to check if a hash exists; Returns true or false */
    pub fn hash_exists(&self, hash_str: &str) -> Result<bool> {
        let mut stmt = self.db_connection.prepare("SELECT hash FROM signatures WHERE hash = ?")?;
        let hash: String = stmt.query_row(params![hash_str], |row| row.get(0))?;
        Ok(!hash.is_empty())
    }

    /* Returns the biggest file_nr. The Number in the Database is a VARCHAR! because of the 5 digit format (example: 00012) */
    pub fn get_latest_file_nr(&self) -> Result<String> {
        let mut stmt = self.db_connection.prepare("SELECT file_nr FROM signatures ORDER BY file_nr DESC LIMIT 1;")?;
        let file_nr: String = stmt.query_row(rusqlite::NO_PARAMS, |row| row.get(0))?;
        Ok(file_nr)
    }

    /* Counts all hashes in the databse and returns the number */
    pub fn count_hashes(&self) -> Result<u64> {
        let mut stmt = self.db_connection.prepare("SELECT COUNT(hash) FROM signatures")?;
        let count: i64 = stmt.query_row(rusqlite::NO_PARAMS, |row| row.get(0))?;
        Ok(count as u64)
    }

    /* Barely ever used: Removes an hash from the database */
    pub fn remove_hash(&self, hash_str: &str) -> Result<()> {
        self.db_connection.execute("DELETE FROM signatures WHERE hash = ?", &[hash_str])?;
        Ok(())
    }
}