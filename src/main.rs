use crate::settings::{Mode, Settings};

mod database;
mod utils;
mod operations;
mod command;
mod settings;
mod shell;
mod server;

fn main() {
    let settings = Settings::new(); 

    match settings.mode {
        Mode::Shell => shell::entry::start(),
        Mode::Server => server::entry::start(),
    }
}
