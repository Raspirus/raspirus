#[cfg(test)]
mod tests {
    use std::path::Path;

    use directories_next::ProjectDirs;
    use futures::channel::mpsc;

    use crate::backend::{downloader, yara_scanner::YaraScanner};

    #[test]
    fn test_scan_file_found_none() {
        // check if running in valid user env, otherwise abort test
        if let None = ProjectDirs::from("qual", "org", "app") {
            eprint!("Could not get user dirs. Is the system setup correctly?");
            assert!(true);
            return;
        }

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let update = runtime.block_on(downloader::update());

        match update {
            Err(err) => {
                dbg!(&err);
                eprintln!(
                    "Updates could not be fetched. Github API rate limit might have been reached"
                );
                assert!(true);
                return;
            }
            _ => {}
        }

        std::fs::write(
            Path::new("./clean"),
            "Test content of a file with no particular malicious intent".to_owned(),
        )
        .unwrap();
        let channel = mpsc::channel(1);

        let result = YaraScanner::new().start(channel.0, vec![Path::new("./clean").to_path_buf()]);
        dbg!(&result);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.len(), 0);
        std::fs::remove_file("./clean").unwrap();
    }

    #[test]
    fn test_scan_file_found_one() {
        // check if running in valid user env, otherwise abort test
        if let None = ProjectDirs::from("qual", "org", "app") {
            eprint!("Could not get user dirs. Is the system setup correctly?");
            assert!(true);
            return;
        }

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let update = runtime.block_on(downloader::update());

        match update {
            Err(err) => {
                dbg!(&err);
                eprintln!(
                    "Updates could not be fetched. Github API rate limit might have been reached"
                );
                assert!(true);
                return;
            }
            _ => {}
        }

        std::fs::write(
            Path::new("./tag"),
            "X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*".to_owned(),
        )
        .unwrap();
        let channel = mpsc::channel(1);
        let result = YaraScanner::new().start(channel.0, vec![Path::new("./tag").to_path_buf()]);
        dbg!(&result);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.len(), 1);
        std::fs::remove_file("./tag").unwrap();
    }
}
