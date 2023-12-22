#[cfg(test)]
mod tests {
    use crate::backend::scanner::Scanner;
    use crate::backend::utils::scanner_utils;
    use log::debug;
    use std::env;

    const DB_FILE_LOC: &str = "signatures.db";

    #[test]
    fn test_new_filescanner_valid_path() {
        let t_win = None;

        // Get the parent directory of the current test file as the scan location
        let scanloc = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        debug!("SCAN LOCATION: {:?}", scanloc);

        let scanner = Scanner::new(DB_FILE_LOC, t_win).unwrap();
        // Check if the scanner is initialized properly
        assert_eq!(scanner.dirty_files.len(), 0);
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

    // Test asynchronous function
    #[tokio::test]
    async fn test_start_scanner() {
        // Get the parent directory of the current test file as the scan location
        let scanloc = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        // Call the asynchronous function with appropriate parameters
        let result = scanner_utils::start_scanner(None, scanloc).await;

        // Assert that the result is what you expect
        assert!(result.is_ok()); // Modify based on your specific expectations
    }

    // Test synchronous function
    #[test]
    fn test_sync_start_scanner() {
        // Set up your test environment or create necessary mocks

        // Get the parent directory of the current test file as the scan location
        let scanloc = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        // Call the synchronous function with appropriate parameters
        let result = scanner_utils::sync_start_scanner(None, scanloc);

        // Assert that the result is what you expect
        assert!(result.is_ok()); // Modify based on your specific expectations
    }
}
