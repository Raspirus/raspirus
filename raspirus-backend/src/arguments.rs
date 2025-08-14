use log::debug;
use std::{path::PathBuf, sync::OnceLock};

static ARGUMENTS: OnceLock<Vec<Argument>> = OnceLock::new();

#[derive(Clone, Debug)]
pub enum Argument {
    Help,
    NoGUI,
    Fullscreen,
    Update,
    Debug,
    Quiet,
    Scan(Option<PathBuf>),
    Json,
    Threads(Option<usize>),
    Invalid(Option<String>),
    MaxMatches(Option<usize>),
    MinMatches(Option<usize>),
    Remote(Option<String>),
}

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
                inv => Self::Invalid(Some(format!(
                    "Unrecognized argument {inv}; Try --help or -h"
                ))),
            }
        } else {
            Self::Invalid(Some(
                "Arguments should start either with --<argname> or -<short argname>".to_owned(),
            ))
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

/// Gets a specific argument
pub fn get_argument(search: &Argument) -> Option<Argument> {
    get_arguments()
        .into_iter()
        .find(|arg| std::mem::discriminant(search) == std::mem::discriminant(arg))
}
