use yara_x::Rules;

pub struct YaraScanner {
    pub rules: Rules, 
}

impl YaraScanner {
    pub fn new(yar_path: PathBuf) -> Self {
        let rules = Rules::deserialize_from();
    }
}
