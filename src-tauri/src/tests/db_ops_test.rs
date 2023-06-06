#[cfg(test)]
mod tests {
    use crate::backend::db_ops::DBOps;
    const DB_FILE_LOC:&str = "signatures.db";

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
            .insert_hashes(vec![("93fe4fb85a682907137b0b1051991332".to_owned(), "ec2112c9c243d837247217baf351ab79".to_owned())])
            .unwrap();
    }

    #[test]
    fn test_hash_exists() {
        let mut db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        db_ops
            .insert_hashes(vec![("93fe4fb85a682907137b0b1051991332".to_owned(), "ec2112c9c243d837247217baf351ab79".to_owned())])
            .unwrap();
        assert_eq!(db_ops.hash_exists("93fe4fb85a682907137b0b1051991332").unwrap(), true);
        assert_eq!(db_ops.hash_exists("hello").unwrap(), false);
    }

    #[test]
    fn test_count_hashes() {
        let db_ops = DBOps::new(DB_FILE_LOC, None).unwrap();
        let hash_count = db_ops.count_hashes();
    
        match hash_count {
            Ok(count) => {
                println!("Hash count: {}", count);
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
        assert!(db_ops._remove_hash("93fe4fb85a682907137b0b1051991332").is_ok());
        assert_eq!(db_ops.hash_exists("93fe4fb85a682907137b0b1051991332").unwrap(), false);
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

    #[cfg(test)]
    #[ctor::dtor]
    fn teardown() {
        if std::path::Path::new(DB_FILE_LOC).exists() {
            if let Err(err) = std::fs::remove_file(DB_FILE_LOC) {
                eprintln!("Failed to delete the database file: {}", err);
            } else {
                println!("Database file deleted successfully");
            }
        }
    }
}
