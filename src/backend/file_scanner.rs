/*
* The filescanner does the following:
* In a loop, find each file in a folder and its subfolders,
* if a file is found, create its hash and see if the hash exists in the database.
* If the hash exists, append it to an array, else continue to the next file.
*/
extern crate walkdir;
use walkdir::WalkDir;
use std::path::Path;
use std::time::{self};
use std::fs::File;
use std::io::Read;
use md5::{Md5, Digest};

use crate::backend::sql;

pub struct FileScanner {
    pub amount_of_files: u64, // Counts the amount of files 
    pub hash_db: sql::DatabaseSQL, // Connection to the database object
    pub dirty_files: Vec<String>, // Contains all paths of the "infected" files (files whose hash was found in the database)
    pub path: String, // The path to where to search for files, should be a directory
}

/* A filescanner object that can be used in other classes.
    It has a database object through which it can check if a Hash is in the database or not.
    When creating it you give him the path that you want to scan. This path is not chnaged afterwards.
    Basically you create the FileScanner object to scan a specific path, ance thats done you just destroy it again.
    You could also not destroy it and just clear the Vector on each completed scan.
    Bear in mind, that if Virus are found, they will need to be displayed and the user can delete them through the GUI.
*/
impl FileScanner {
    pub fn new(path: String) -> Result<FileScanner, rusqlite::Error> {
        // Checks that the path exists and the path is a directory
        if Path::new(&path).exists() {
            Ok(FileScanner {
                amount_of_files: 0,
                hash_db: sql::DatabaseSQL::new().unwrap(),
                dirty_files: Vec::new(),
                path,
            })
        } else {
            Err(rusqlite::Error::InvalidPath(("").into()))
        }
    }
    /* This function searches for all files in a directory and its subdirectories.
        Then it creates the hash of each file using a function defined further below. Each hash
        gets then compared to the database using the hash_exists() function.
        If a hash is found, add the path of the file to the Vector, else ignore it
     */
    pub fn search_files(self) {
        let mut number_of_files = 0;
        let big_tic = time::Instant::now();
        for file in WalkDir::new(self.path).into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                self.create_hash(&file.path().display().to_string());
                number_of_files += 1;
            }
        }
        let big_toc = time::Instant::now(); // Measures time, interesting to see if its faster than Python ;)
        println!("Executed in {} seconds", big_toc.duration_since(big_tic).as_secs_f64());
        println!("Found {} files", number_of_files);
    }

    /*
        This function creates the hash from a given file using the Rust crate md-5
        and returns it.
     */
    fn create_hash(self, file_path:&str) {
        // Open the file in read-only mode
        let mut file = File::open(file_path).unwrap();

        // Create a new instance of the Md5 hasher
        let mut hasher = Md5::new();
    
        // Read the contents of the file into a buffer
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
    
        // Write the contents of the buffer to the hasher
        hasher.update(&buffer);
    
        // Get the result of the hash as a hexadecimal string
        let hash = hasher.finalize();
        let hash_str = format!("{:x}", hash);
        println!("The MD5 hash of the file {} is: {}", file_path, hash_str);
    }

}