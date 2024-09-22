#[cfg(test)]
mod tests {
    #[test]
    fn test_filescanner_invalid_path() {
        use crate::backend::yara_scanner::YaraScanner;
        use std::path::Path;

        let path = Path::new("/this/path/does/not/exist");
        let scanner = YaraScanner::new().set_path(path.to_path_buf());

        assert!(scanner.is_err());
    }

    #[test]
    fn test_filescanner_valid_path() {
        use crate::backend::yara_scanner::YaraScanner;
        use std::path::Path;

        let path = Path::new("./");
        let scanner = YaraScanner::new().set_path(path.to_path_buf());

        assert!(scanner.is_ok());
    }

    #[test]
    fn test_scan_file_found_none() {
        use crate::backend::yara_scanner::YaraScanner;
        use iced::futures::channel::mpsc;
        use std::{
            path::Path,
            sync::{Arc, Mutex},
        };

        std::fs::write(
            Path::new("./clean"),
            "Test content of a file with no particular malicious intent".to_owned(),
        )
        .unwrap();
        let mut scanner = YaraScanner::new()
            .set_path(Path::new("./clean").to_path_buf())
            .unwrap();

        let channel = mpsc::channel(1);
        scanner.progress_sender = Some(Arc::new(Mutex::new(channel.0)));

        let result = scanner.start();
        dbg!(&result);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.len(), 0);
        std::fs::remove_file("./clean").unwrap();
    }

    #[test]
    fn test_scan_file_found_one() {
        use crate::backend::yara_scanner::YaraScanner;
        use iced::futures::channel::mpsc;
        use std::{
            path::Path,
            sync::{Arc, Mutex},
        };

        std::fs::write(
            Path::new("./tag"),
            "X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*".to_owned(),
        )
        .unwrap();
        let mut scanner = YaraScanner::new()
            .set_path(Path::new("./tag").to_path_buf())
            .unwrap();
        let channel = mpsc::channel(1);
        scanner.progress_sender = Some(Arc::new(Mutex::new(channel.0)));

        let result = scanner.start();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.len(), 1);
        std::fs::remove_file("./tag").unwrap();
    }
}
