use std::{path::Path, process::exit, time};

use backend::file_scanner;
use log::{error, info};

mod backend;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    pretty_env_logger::init();
    let mut path: Option<String> = None;
    let mut update: bool = true;
    let mut dbfile: Option<String> = None;
    let len = args.len();
    for arg in args {
        if arg.starts_with("--path=") {
            let tmp: Vec<&str> = arg.split("=").collect();
            path = Some(tmp[1].to_owned());
        } else if arg.starts_with("--no-update") {
            update = false;
        } else if arg.starts_with("--help") && len == 2 || len == 1 {
            error!("Usage: [RUST_LOG=none | info | debug] binary --help --path=/path/to/scan --no-update --db-file=/path/to/db");
            exit(0);
        } else if arg.starts_with("--db-file=") {
            let tmp: Vec<&str> = arg.split("=").collect();
            dbfile = Some(tmp[1].to_owned());
        }
    }

    let patharg = match path {
        Some(path) => path,
        None => {
            error!("Did not pass --path");
            exit(-1);
        }
    };

    let mut use_db = "signatures.db".to_owned();
    match dbfile {
        Some(fpath) => {
            if Path::new(&fpath).to_owned().exists() && Path::new(&fpath).to_owned().is_file() {
                info!("Using specific DB path {}", fpath);
                use_db = fpath.to_owned();
            } else {
                info!("Falling back to default DB file (signatures.db)");
            }
        }
        None => {
            info!("Path is None; Falling back to default DB file (signatures.db)");
        }
    };

    let mut fs = match file_scanner::FileScanner::new(&patharg, &use_db) {
        Ok(fs) => fs,
        Err(err) => {
            error!("{}", err);
            exit(-1);
        }
    };
    if update {
        let big_tic = time::Instant::now();
        fs.db_conn.update_db();
        let big_toc = time::Instant::now();
        info!(
            "Updated DB in {} seconds",
            big_toc.duration_since(big_tic).as_secs_f64()
        );
    } else {
        info!("Skipped update");
    }

    fs.search_files();
}
