#[cfg(test)]
mod tests {
    use std::path::Path;

    use directories_next::ProjectDirs;

    use crate::backend::{file_log::FileLog, yara_scanner::TaggedFile};

    #[test]
    fn test_create_log() {
        // check if running in valid user env, otherwise abort test
        if let None = ProjectDirs::from("qual", "org", "app") {
            eprint!("Could not get user dirs. Is the system setup correctly?");
            assert!(true);
            return;
        }
        let log = FileLog::new();
        assert!(log.is_ok());
    }

    #[test]
    fn test_path_determination() {
        // check if running in valid user env, otherwise abort test
        if let None = ProjectDirs::from("qual", "org", "app") {
            eprint!("Could not get user dirs. Is the system setup correctly?");
            assert!(true);
            return;
        }

        let log = FileLog::new().unwrap();
        assert!(log.log_path.exists());
    }

    #[test]
    fn test_log() {
        // check if running in valid user env, otherwise abort test
        if let None = ProjectDirs::from("qual", "org", "app") {
            eprint!("Could not get user dirs. Is the system setup correctly?");
            assert!(true);
            return;
        }


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
