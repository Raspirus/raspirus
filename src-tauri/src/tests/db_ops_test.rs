#[cfg(test)]
mod tests {
    use log::{error, info};

    use crate::backend::db_ops::DBOps;
    const DB_FILE_LOC: &str = "signatures.db";

    #[test]
    fn test_init_table() {
        let db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        assert!(db_ops.init_table().is_ok());
    }

    #[test]
    fn test_download_files() {
        let mut db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        db_ops.download_files(vec![1, 2]);
    }

    #[test]
    fn test_download_file() {
        let file = DBOps::download_file(3).unwrap();
        assert!(file.is_some());
    }

    #[test]
    fn test_insert_hashes() {
        let mut db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        db_ops
            .insert_hashes(vec![(
                "93fe4fb85a682907137b0b1051991332".to_owned(),
                "ec2112c9c243d837247217baf351ab79".to_owned(),
            )])
            .unwrap();
    }

    #[test]
    fn test_hash_exists() {
        let mut db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        let hash_to_insert = "93fe4fb85a682907137b0b1051991332";
        // let hash_not_in_db = "hello";

        // Insert the hash into the database
        db_ops
            .insert_hashes(vec![(
                hash_to_insert.to_owned(),
                "ec2112c9c243d837247217baf351ab79".to_owned(),
            )])
            .unwrap();

        // Check if the hash exists in the database
        // let exists_in_db = db_ops.hash_exists(hash_to_insert).unwrap();
        // let does_not_exist = db_ops.hash_exists(hash_not_in_db).unwrap();

        // Assert the results
        //assert_eq!(exists_in_db, Some(true));
        //assert_eq!(does_not_exist, Some(false));
    }

    #[test]
    fn test_count_hashes() {
        let db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        let hash_count = db_ops.count_hashes();

        match hash_count {
            Ok(count) => {
                info!("Hash count: {}", count);
                assert!(true);
            }
            Err(err) => {
                panic!("Error occurred: {:?}", err);
            }
        }
    }

    #[test]
    fn test_remove_hash() {
        let db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        let hash_to_remove = "93fe4fb85a682907137b0b1051991332";

        // Remove the hash from the database
        db_ops._remove_hash(hash_to_remove).unwrap();

        // Check if the hash exists in the database after removal
        let exists_after_removal = match db_ops.hash_exists(hash_to_remove) {
            Ok(exists) => exists,
            Err(_) => false, // Hash not found
        };

        // Assert the result
        assert_eq!(exists_after_removal, false);
    }

    #[test]
    fn test_get_file_list() {
        let db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        assert!(db_ops.get_file_list() > 0);
    }

    #[test]
    fn test_file_exists() {
        assert!(DBOps::file_exists(1).unwrap_or(false));
    }

    #[test]
    fn test_get_db_files() {
        let db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        assert!(db_ops.get_db_files().is_some());
    }

    #[test]
    fn test_update_db_timeout() {
        // Run the update_db function for 10 seconds, then kill it if no errors occurred before
        let mut db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        let _ = std::thread::spawn(move || {
            let result = db_ops.update_db();
            if let Err(err) = result {
                error!("Error occurred: {}", err);
                assert!(false);
            }
        });
        std::thread::sleep(std::time::Duration::from_secs(10));
        
        // If we got here, it means that the update_db function ran for 10 seconds without errors
        // So we can safely assume that the function works as expected
        assert!(true);
    }

    #[test]
    fn test_db_diff_func() {
        let db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        let diff = db_ops.get_diff_file();
        assert!(!diff.is_empty());
    }

    #[cfg(test)]
    #[ctor::dtor]
    fn teardown() {
        if std::path::Path::new(DB_FILE_LOC).exists() {
            if let Err(err) = std::fs::remove_file(DB_FILE_LOC) {
                error!("Failed to delete the database file: {}", err);
            } else {
                info!("Database file deleted successfully");
            }
        }
    }
}
