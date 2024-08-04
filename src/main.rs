use clap::Parser;

use command::Cli;

use crate::settings::Settings;

mod database;
mod utils;
mod handlers;
mod command;
mod settings;

fn main() {
    let settings = Settings::new().expect("Failed to load settings");
    println!("Mode: {}", settings.mode);

    let cli = Cli::parse();

    match cli.command {
        Some(command) => command::handle_command(command),
        None => println!("[tc] No command provided"),
    }
}
