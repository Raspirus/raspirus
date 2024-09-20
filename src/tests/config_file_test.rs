#[cfg(test)]
mod tests {
    use crate::backend::config_file::Config;

    #[test]
    fn test_new_config() {
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
        let config = Config::new();
        assert!(config.is_ok());
    } 
}
