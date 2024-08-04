use config::{Config, ConfigError};
use serde::Deserialize;

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()
            .unwrap();

        s.try_deserialize()
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
