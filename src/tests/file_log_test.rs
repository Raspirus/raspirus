#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::{io::Write, path::Path};

    use crate::backend::file_log::FileLog;

    #[test]
    fn test_create_file() {
        let log = FileLog::new().unwrap();

        // Assert that the file is created
        assert!(log.file.is_some());
    }

    #[test]
    fn test_log() {
        let log = FileLog::new().unwrap();

        // Log a hash and file path
        log.log(
            Path::new("C:/Users/user/Desktop/file.txt").to_path_buf(),
            1,
            &Vec::new(),
        );

        // Assert that the log entry is written to the file
        let file = log.file.unwrap();

        let mut output = Vec::new();
        writeln!(&mut output, "{:?}", file).unwrap();
        let output_str = String::from_utf8_lossy(&output);

        let file_path = if cfg!(windows) {
            let re = Regex::new(r#"\\\\\?\\\\(.+)""#).unwrap();
            let captures = re.captures(&output_str).unwrap();
            let path = &captures[1];

            path.to_string()
        } else {
            let re = Regex::new(r#"path: "(.+)""#).unwrap();
            let captures = re.captures(&output_str).unwrap();
            let path = &captures[1];

            path.to_string()
        };

        let contents = std::fs::read_to_string(file_path).expect("Failed to read file");

        assert_eq!(contents, "abc123\tC:/Users/user/Desktop/file.txt\n");
    }

    #[cfg(test)]
    fn teardown() {
        use log::{error, info};

        let log = FileLog::new().unwrap();
        let file = log.file.unwrap();

        let mut output = Vec::new();
        writeln!(&mut output, "{:?}", file).unwrap();
        let output_str = String::from_utf8_lossy(&output);

        let file_path = if cfg!(windows) {
            let re = Regex::new(r#"\\\\\?\\\\(.+)""#).unwrap();
            let captures = re.captures(&output_str).unwrap();
            let path = &captures[1];

            path.to_string()
        } else {
            let re = Regex::new(r#"path: "(.+)""#).unwrap();
            let captures = re.captures(&output_str).unwrap();
            let path = &captures[1];

            path.to_string()
        };

        if std::path::Path::new(&file_path).exists() {
            if let Err(err) = std::fs::remove_file(file_path) {
                error!("Failed to delete the log file: {}", err);
            } else {
                info!("Log file deleted successfully");
            }
        } else {
            info!("Teardown skipped, file does not exist");
        }
    }
}
