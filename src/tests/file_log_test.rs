#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::backend::{file_log::FileLog, yara_scanner::TaggedFile};

    #[test]
    fn test_create_log() {
        let log = FileLog::new();
        assert!(log.is_ok());
    }

    #[test]
    fn test_path_determination() {
        let log = FileLog::new().unwrap();
        assert!(log.log_path.exists());
    }

    #[test]
    fn test_log() {
        let mut log = FileLog::new().unwrap();

        log.log(&TaggedFile {
            path: Path::new("").to_path_buf(),
            descriptions: vec![],
            rule_count: 0,
        });

        let output = std::fs::read_to_string(log.log_path).unwrap();

        assert_eq!(output, "[0]\t\n\n");
    }
}
