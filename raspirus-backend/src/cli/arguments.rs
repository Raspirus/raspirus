use rust_i18n::t;

use log::debug;
use std::{path::PathBuf, sync::OnceLock};

rust_i18n::i18n!("locales", fallback = "en");

static ARGUMENTS: OnceLock<Vec<Argument>> = OnceLock::new();

#[derive(Clone, Debug)]
pub enum Argument {
    /// Help about cli arguments
    Help,
    /// Does not launcht the GUI
    NoGUI,
    /// Launches the GUI in fullscreen; Does nothing when NoGUI is set
    Fullscreen,
    /// Attempts to run an update for the rules
    Update,
    /// Sets the loglevel to debug, regardless of whats configured in the config
    Debug,
    /// Silences log output, regardless of whats configured in the config
    Quiet,
    /// Scans the provided path
    Scan(Option<PathBuf>),
    /// Outputs the scan results as json for parsing with third party tools
    Json,
    /// Sets the amount of threads used for scanning
    Threads(Option<usize>),
    /// If an invalid argument has been provided and why
    Invalid(Option<Cow<'static, str>>),
    /// Sets the amount of minimum rule matches a file must produce to get flagged
    MaxMatches(Option<usize>),
    /// Sets the amount of maximum rule matches a file will produce before no more will be produced
    MinMatches(Option<usize>),
    /// Sets the remote mirror from which the update should be fetched
    Remote(Option<String>),
}

/// Parse argument from string
impl From<String> for Argument {
    /// Will remove the leading -- or - from the arguments
    fn from(value: String) -> Self {
        if value.starts_with("--") || value.starts_with("-") {
            let value = value.trim_start_matches("-");
            match value {
                "h" | "help" => Self::Help,
                "n" | "nogui" => Self::NoGUI,
                "f" | "fullscreen" => Self::Fullscreen,
                "u" | "update" => Self::Update,
                "d" | "debug" => Self::Debug,
                "q" | "quiet" => Self::Quiet,
                "s" | "scan" => Self::Scan(None),
                "j" | "json" => Self::Json,
                "t" | "threads" => Self::Threads(None),
                "x" | "max" => Self::MaxMatches(None),
                "i" | "min" => Self::MinMatches(None),
                "r" | "remote" => Self::Remote(None),
                inv => Self::Invalid(Some(t!("ARGUMENTS.INVALID", argument = inv))),
            }
        } else {
            Self::Invalid(Some(t!("ARGUMENTS.NO_DELIMITER")))
        }
    }
}

/// Parses the passed arguments and returns an array with them
pub fn get_arguments() -> Vec<Argument> {
    ARGUMENTS
        .get_or_init(|| {
            // collect args and pop executable name as it is just in the way
            let mut args = std::env::args().collect::<Vec<String>>();
            args.reverse();
            let _executable = args.pop();

            let mut all_parsed = Vec::new();

            // parse arguments
            while let Some(arg) = args.pop() {
                debug!("Parsing argument {arg}");
                let parsed = Argument::from(arg);
                match parsed {
                    Argument::Scan(_) => all_parsed.push(Argument::Scan(
                        args.pop().and_then(|path| path.parse().ok()),
                    )),
                    Argument::Threads(_) => all_parsed.push(Argument::Threads(
                        args.pop().and_then(|threads| threads.parse().ok()),
                    )),
                    Argument::MinMatches(_) => all_parsed.push(Argument::MinMatches(
                        args.pop().and_then(|matches| matches.parse().ok()),
                    )),
                    Argument::MaxMatches(_) => all_parsed.push(Argument::MaxMatches(
                        args.pop().and_then(|matches| matches.parse().ok()),
                    )),
                    Argument::Remote(_) => all_parsed.push(Argument::Remote(args.pop())),
                    parsed => all_parsed.push(parsed),
                }
            }
            all_parsed
        })
        .clone()
}

/// Gets a specific argument if it exists
pub fn get_argument(search: &Argument) -> Option<Argument> {
    get_arguments()
        .into_iter()
        .find(|arg| std::mem::discriminant(search) == std::mem::discriminant(arg))
}
