#[cfg(test)]
mod tests {
    use directories_next::ProjectDirs;

    use crate::backend::config_file::Config;

    #[test]
    fn test_new_config() {

        // check if running in valid user env, otherwise abort test
        if let None = ProjectDirs::from("qual", "org", "app") {
            eprint!("Could not get user dirs. Is the system setup correctly?");
            assert!(true);
            return;
        }

        let config = Config::default();

        assert_eq!(config.config_version, crate::CONFIG_VERSION);
        assert_eq!(config.rules_version, "None");
        assert_eq!(config.min_matches, crate::DEFAULT_MIN_MATCHES);
        assert_eq!(config.max_matches, crate::DEFAULT_MAX_MATCHES);
        assert_eq!(config.max_threads, num_cpus::get());
        assert_eq!(config.logging_is_active, true);
        assert_eq!(config.mirror, crate::DEFAULT_MIRROR)
    }

    #[test]
    fn test_load_config() {
        // check if running in valid user env, otherwise abort test
        if let None = ProjectDirs::from("qual", "org", "app") {
            eprint!("Could not get user dirs. Is the system setup correctly?");
            assert!(true);
            return;
        }


        let config = Config::new();
        dbg!(&config);
        assert!(config.is_ok());
    }
}
