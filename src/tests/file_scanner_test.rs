#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use std::{
        path::Path,
        sync::{mpsc, Arc, Mutex},
    };

    use crate::backend::yara_scanner::YaraScanner;

    #[test]
    fn test_new_filescanner() {
        let channel = mpsc::channel();
        let scanner = YaraScanner::new(Arc::new(Mutex::new(channel.0)));

        assert!(scanner.is_ok());
    }

    #[test]
    fn test_filescanner_invalid_path() {
        let path = Path::new("/this/path/does/not/exist");
        let channel = mpsc::channel();
        let scanner = YaraScanner::new(Arc::new(Mutex::new(channel.0)))
            .unwrap()
            .set_path(path.to_path_buf());

        assert!(scanner.is_err());
    }

    #[test]
    fn test_filescanner_valid_path() {
        let path = Path::new("./");
        let channel = mpsc::channel();
        let scanner = YaraScanner::new(Arc::new(Mutex::new(channel.0)))
            .unwrap()
            .set_path(path.to_path_buf());

        assert!(scanner.is_ok());
    }

    #[test]
    fn test_scan_file_found_none() {
        std::fs::write(
            Path::new("./clean"),
            "Test content of a file with no particular malicious intent".to_owned(),
        )
        .unwrap();
        let channel = mpsc::channel();
        let scanner = YaraScanner::new(Arc::new(Mutex::new(channel.0)))
            .unwrap()
            .set_path(Path::new("./clean").to_path_buf())
            .unwrap();

        let result = block_on(async { scanner.start().await });

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.len(), 0);
        std::fs::remove_file("./clean").unwrap();
    }

    #[test]
    fn test_scan_file_found_one() {
        std::fs::write(
            Path::new("./tag"),
            "X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*".to_owned(),
        )
        .unwrap();
        let channel = mpsc::channel();
        let scanner = YaraScanner::new(Arc::new(Mutex::new(channel.0)))
            .unwrap()
            .set_path(Path::new("./tag").to_path_buf())
            .unwrap();

        let result = block_on(async { scanner.start().await });

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.len(), 1);
        std::fs::remove_file("./tag").unwrap();
    }
}
