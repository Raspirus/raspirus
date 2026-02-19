use serde::{Deserialize, Serialize};

use crate::{Config, LogLevel};

/// Attempts to migrate whichever string input it gets into the newest config
pub fn migrate(config_string: String) -> Config {
    migrate_6_7(config_string)
}

/// Migrates the config from version 6 to version 7. this is the baseline and there is no support
/// for migrating older configs than this
fn migrate_6_7(config_string: String) -> Config {
    /// config definition
    #[derive(Deserialize, Serialize)]
    struct ConfigOld {
        pub config_version: usize,
        pub remote_url: String,
        pub min_matches: usize,
        pub max_matches: usize,
        pub max_threads: usize,
        pub logging: LogLevel,
        pub language: String,
    }

    /// since this is the baseline config it needs a default function.
    /// the program is not backwards compatible with versions older than config 6
    impl Default for ConfigOld {
        fn default() -> Self {
            Self {
                config_version: 7,
                remote_url: crate::globals::DEFAULT_REMOTE_URL.to_owned(),
                min_matches: 1,
                max_matches: 0,
                max_threads: num_cpus::get(),
                logging: crate::globals::DEFAULT_LOG_LEVEL.clone(),
                language: crate::globals::DEFAULT_LANGUAGE.to_owned(),
            }
        }
    }

    // if config version is older than 6, load 6s defaults
    // if the parse fails on newer version, instead of defaulting we try to parse from an older
    // version
    let config_6 = serde_json::from_str::<ConfigOld>(&config_string).unwrap_or_default();
    // create upper number config to transfer transferable values to
    let mut config = crate::Config::default();

    // update all fields that config 6 already contained
    config.config_version = 7;
    config.remote_url = config_6.remote_url;
    config.scanner.min_matches = config_6.min_matches;
    config.scanner.max_matches = config_6.max_matches;
    config.scanner.max_threads = config_6.max_threads;
    config.logging = config_6.logging;
    config.language = config_6.language;

    config
}
