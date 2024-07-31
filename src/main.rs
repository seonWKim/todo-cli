use std::io;
use std::io::Write;

use clap::{CommandFactory, Parser, Subcommand};

use crate::database::TodoDatabase;
use crate::handlers::{handle_add, handle_done, handle_help, handle_list, handle_remove, handle_remove_all};
use crate::utils::{log, user_input};

mod database;
mod utils;
mod handlers;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    #[command(
        name = "i", aliases = ["interactive"], about = "Start interactive mode"
    )]
    Interactive {},

    #[command(name = "a", aliases = ["add"], about = "Add a new task")]
    Add {
        task: Vec<String>,
    },

    #[command(name = "l", aliases = ["ls", "list"], about = "List all tasks")]
    List {
        #[arg(short, long, help = "Include tasks marked as done")]
        all: bool
    },

    #[command(name = "d", aliases = ["done"], about = "Mark a task as done")]
    Done {
        task: i32,
    },

    #[command(name = "r", aliases = ["remove"], about = "Remove a task")]
    Remove {
        task: i32,
    },

    #[command(name = "ra", aliases = ["removeAll"], about = "Remove all tasks")]
    RemoveAll,

    #[command(name = "h", about = "Print help information")]
    Help,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => handle_command(command),
        None => println!("[tc] No command provided"),
    }
}

fn handle_command(command: Command) {
    let tdb = TodoDatabase::new();
    tdb.initialize().expect("Database is not initialized");

    match command {
        Command::Interactive {} => {
            loop {
                user_input("Enter a command(type help for more commands): ");
                io::stdout().flush().expect("Failed to flush stdout");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                let program_name = "tc";
                let formatted_input = format!("{} {}", program_name, input.trim());
                let args = formatted_input.split_whitespace().collect::<Vec<_>>();
                match Cli::try_parse_from(args) {
                    Ok(cli) => {
                        match cli.command {
                            Some(command) => {
                                handle_non_interactive_command(&tdb, command);
                            }
                            None => {
                                log("Invalid command provided, stopping...");
                                break;
                            }
                        }
                    }
                    _ => {
                        // We can't register help as command's alias because there is a conflict with the (clap's) help command
                        if (input.trim() == "help") {
                            handle_non_interactive_command(&tdb, Command::Help);
                        } else {
                            log("Invalid command provided, stopping...");
                            break;
                        }
                    }
                }

                println!()
            }
        }
        _ => handle_non_interactive_command(&tdb, command),
    }
}

fn handle_non_interactive_command(tdb: &TodoDatabase, command: Command) {
    match command {
        Command::Add { task } => {
            let todo = task.join(" ");
            handle_add(&tdb, &todo);
        }
        Command::List { all } => {
            handle_list(&tdb, all)
        }
        Command::Done { task } => {
            handle_done(&tdb, task);
        }
        Command::Remove { task } => {
            handle_remove(&tdb, task);
        }
        Command::RemoveAll => {
            handle_remove_all(&tdb);
        }
        Command::Help => {
            handle_help();
        }
        _ => {}
    }
}
