use std::{fs::File, path::PathBuf};

use yara_x::Rules;

pub struct YaraScanner {
    pub rules: Rules,
}

impl YaraScanner {
    pub fn new(yar_path: PathBuf) -> Result<Self, String> {
        let reader = File::open(yar_path).map_err(|err| format!("Failed to load yar file: {}", err.to_string()))?;
        let rules = Rules::deserialize_from(reader).map_err(|err| format!("Failed to deserialize yar file: {}", err.to_string()))?;
        Ok(Self { rules })
    }
}
