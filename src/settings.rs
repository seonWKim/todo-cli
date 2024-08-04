use std::env;

use serde::Deserialize;

impl Settings {
    pub fn new() -> Self {
        Settings {
            mode: match env::var("TC_MODE").unwrap_or_else(|_| "shell".to_string()).to_lowercase().as_str() {
                "server" => Mode::Server,
                _ => Mode::Shell,
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub mode: Mode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Shell,
    Server,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Mode::Shell => "shell",
            Mode::Server => "server",
        };
        write!(f, "{}", s)
    }
}
