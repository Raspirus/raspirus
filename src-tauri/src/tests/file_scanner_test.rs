#[cfg(test)]
mod tests {
    use crate::backend::scanner::Scanner;
    use std::env;

    const DB_FILE_LOC: &str = "signatures.db";

    #[test]
    fn test_new_filescanner_valid_path() {
        let scanner = Scanner::new(DB_FILE_LOC, None);
        assert!(scanner.is_ok());
    }

    #[test]
    fn test_new_filescanner_invalid_path() {
        // Invalid path to the database file
        let db_file_loc = "non-existent-dir/signatures.db";
        let scanner = Scanner::new(db_file_loc, None);
        assert!(scanner.is_err());
    }

    #[test]
    fn test_init_invalid_path() {
        let t_win = None;
        // Invalid path to the directory to scan
        let scanloc = "non-existent-dir";
        let scanner = Scanner::new(DB_FILE_LOC, t_win).unwrap();
        let dirty_files = scanner.init(false, scanloc);
        assert!(dirty_files.is_err());
    }

    #[test]
    fn test_search_files() {
        let t_win = None;

        // Get the parent directory of the current test file as the scan location
        let scanloc = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        let scanner = Scanner::new(DB_FILE_LOC, t_win).unwrap();
        let dirty_files = scanner.init(false, &scanloc).unwrap();
        // Assert that the list of dirty_files is empty since we didn't add any malicious files
        assert_eq!(dirty_files.len(), 0);
    }

    #[test]
    fn test_scan_single_file() {
        let t_win = None;

        // Get the parent directory of the current test file as the scan location
        let mut scanloc = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        panic!("scanloc: {}", scanloc);

        // Scan the current test file
        scanloc.push_str("\\src\\tests\\file_scanner_test.rs");

        let scanner = Scanner::new(DB_FILE_LOC, t_win).unwrap();
        let dirty_files = scanner.init(false, &scanloc).unwrap();
        // Assert that the list of dirty_files is empty since we didn't add any malicious files
        assert_eq!(dirty_files.len(), 0);
    }

    #[test]
    fn test_scan_zip_file() {
        let t_win = None;

        // Get the parent directory of the current test file as the scan location
        let mut scanloc = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();
        // Warning, this returns the src-tauri folder

        // Scan the zip sample file
        scanloc.push_str("\\src\\tests\\test_sample.zip");

        let scanner = Scanner::new(DB_FILE_LOC, t_win).unwrap();
        let dirty_files = scanner.init(false, &scanloc).unwrap();
        // Assert that the list of dirty_files is empty since we didn't add any malicious files
        assert_eq!(dirty_files.len(), 0);
    }
}
