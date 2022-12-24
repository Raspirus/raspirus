use std::{
    fs::{self, File},
    io::Write,
};

use log::{debug, error, info, warn};

pub struct FileLog {
    file: Option<File>,
}

impl FileLog {
    pub fn new(fname: String) -> Self {
        let mut ret = FileLog { file: None };
        ret.create_file(fname);
        ret
    }

    pub fn log(&self, hash: String, fpath: String) {
        if self.file.is_some() {
            match self.file.as_ref() {
                Some(mut file) => {
                    match file.write_all(format!("{hash}\t{fpath}\n").as_bytes()) {
                        Ok(_) => {
                            debug!("Wrote {hash}\t{fpath} to {:?}", self.file.as_ref().unwrap())
                        }
                        Err(err) => error!("Failed loggin: {err}"),
                    };
                }
                None => {
                    warn!("Logfile invalid!");
                }
            }
        }
    }

    pub fn create_file(&mut self, fname: String) {
        match fs::create_dir_all("../../../../logs") {
            Ok(_) => {
                self.file = match File::create(format!("../../../../logs/{}", fname.clone())) {
                    Ok(file) => {
                        info!("Created logfile {}", fname);
                        Some(file)
                    }
                    Err(err) => {
                        error!("Failed creating logfile: {err}");
                        None
                    }
                };
            }
            Err(err) => error!("Failed createing logs folder: {err}"),
        }
    }
}
