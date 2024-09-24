#[cfg(test)]
mod tests {

    #[test]
    fn test_scan_file_found_none() {
        use crate::backend::{downloader, yara_scanner::YaraScanner};
        use iced::futures::channel::mpsc;
        use std::path::Path;

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let update = runtime.block_on(downloader::update());
        dbg!(&update);

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
        use crate::backend::{downloader, yara_scanner::YaraScanner};
        use iced::futures::channel::mpsc;
        use std::path::Path;

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let update = runtime.block_on(downloader::update());
        dbg!(&update);

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
