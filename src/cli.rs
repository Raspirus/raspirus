use clap::Parser;
use std::path::PathBuf;

/// Raspirus - A simple yara-based virus scanner
#[derive(Parser, Debug, Clone)]
#[command(name = "raspirus")]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Does not attempt to open a GUI. Will result in the app not doing anything without any other arguments
    #[arg(long)]
    pub nogui: bool,

    /// Sets the UI to be fullscreen on launch
    #[arg(long)]
    pub fullscreen: bool,

    /// Attempts an update from the CLI
    #[arg(long)]
    pub update: bool,

    /// Sets a scan thread limit (overrides config setting)
    #[arg(long)]
    pub threads: Option<usize>,

    /// Attempts to scan a path
    #[arg(long)]
    pub scan: Option<PathBuf>,

    /// Suppresses the log messages
    #[arg(long)]
    pub nolog: bool,

    /// Sets the log to output debug messages, even in production builds
    #[arg(long)]
    pub debug: bool,

    /// Outputs the scan results as JSON. Useful for parsing output in scripts
    #[arg(long)]
    pub json: bool,
}

impl CliArgs {
    /// Parse CLI arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Check if any action was requested (update or scan)
    pub fn has_action(&self) -> bool {
        self.update || self.scan.is_some()
    }

    /// Check if GUI should be launched
    pub fn should_launch_gui(&self) -> bool {
        !self.nogui && !self.has_action()
    }
}
