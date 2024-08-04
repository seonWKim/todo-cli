use crate::settings::{Mode, Settings};

mod database;
mod utils;
mod handlers;
mod command;
mod settings;
mod shell;
mod server;

fn main() {
    let settings = Settings::new().expect("Failed to load settings");

    match settings.mode {
        Mode::Shell => shell::entry::start(),
        Mode::Server => server::entry::start(),
    }
}
