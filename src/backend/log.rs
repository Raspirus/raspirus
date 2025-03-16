use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LogLevel {
    Off,
    Info,
    Debug,
    Trace
}
