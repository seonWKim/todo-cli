use clap::Parser;

use command::Command;

mod database;
mod utils;
mod handlers;
mod command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => command::handle_command(command),
        None => println!("[tc] No command provided"),
    }
}
