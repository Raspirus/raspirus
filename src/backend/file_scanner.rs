/*
* The filescanner does the following:
* In a loop, find each file in a folder and its subfolders,
* if a file is found, create its hash and see if the hash exists in the database.
* If the hash exists, append it to an array, else continue to the next file.
*/

pub struct FileScanner {
    pub amount_of_files: u64, // Counts the amount of files 
    pub hash_db: DatabaseSQL, // Connection to the database object
    pub dirty_files: Vec<String>, // Contains all paths of the "infected" files (files whose hash was found in the database)
    pub path: String, // The path to where to search for files, should be a directory
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

    async fn calculate_hash(file_path:str);

    async fn scan_files(self);

    pub fn start_scanner(self);

}