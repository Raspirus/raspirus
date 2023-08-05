#[cfg(test)]
mod tests {
    use crate::backend::file_scanner::FileScanner;
    use std::env;

    const DB_FILE_LOC: &str = "signatures.db";

    #[test]
    fn test_new_filescanner_valid_path() {
        let t_win = None;

        // Get the parent directory of the current test file as the scan location
        let scanloc = env::current_dir().expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        println!("SCAN LOCATION: {:?}", scanloc);

        let scanner = FileScanner::new(&scanloc, DB_FILE_LOC, t_win).unwrap();
        // Check if the scanner is initialized properly
        assert_eq!(scanner.scanloc, scanloc);
        assert_eq!(scanner.dirty_files.len(), 0);
    }

    #[test]
    fn test_new_filescanner_invalid_path() {
        let t_win = None;
        let result = FileScanner::new("/nonexistent/path", DB_FILE_LOC, t_win);
        // Check if the function returns an error for an invalid path
        assert!(result.is_err());
    }

    #[test]
    fn test_search_files() {
        let t_win = None;

        // Get the parent directory of the current test file as the scan location
        let scanloc = env::current_dir().expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        let mut scanner = FileScanner::new(&scanloc, DB_FILE_LOC, t_win).unwrap();
        let dirty_files = scanner.search_files(false).unwrap();
        // Assert that the list of dirty_files is empty since we didn't add any malicious files
        assert_eq!(dirty_files.len(), 0);
    }

    #[test]
    fn test_create_hash_valid_file() {
        let t_win = None;
        
        // Get the parent directory of the current test file as the scan location
        let scanloc = env::current_dir().expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        // We are using the current file to test a valid hashable file
        let exe_path = env::current_exe().expect("Failed to get current executable path");
        let current_file_path = std::env::current_dir().unwrap().join(exe_path)
            .to_string_lossy()
            .to_string();

        let mut scanner = FileScanner::new(&scanloc, DB_FILE_LOC, t_win).unwrap();
        let hash = scanner.create_hash(&current_file_path).unwrap();
        println!("VALID FILE HASH: {:?}", hash);
        // Assert that the hash is generated correctly for a valid file
        assert_eq!(hash.len(), 32); // Assuming MD5 hash length is 32 characters
    }

    #[test]
    fn test_create_hash_invalid_file() {
        let t_win = None;

        // Get the parent directory of the current test file as the scan location
        let scanloc = env::current_dir().expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        let mut scanner = FileScanner::new(&scanloc, DB_FILE_LOC, t_win).unwrap();
        let hash = scanner.create_hash("/path/to/invalid/file.txt");
        println!("INVALID FILE HASH: {:?}", hash);
        // Assert that the function returns None for an invalid file path
        assert!(hash.is_none());
    }
}
