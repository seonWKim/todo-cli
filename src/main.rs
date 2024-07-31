use clap::{Parser, Subcommand};

use crate::database::TodoDatabase;
use crate::handlers::{handle_add, handle_done, handle_find, handle_help, handle_list, handle_remove, handle_reset, handle_undone};
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

    #[command(name = "a", aliases = ["add"], about = "Add new todo")]
    Add {
        task: Vec<String>,
    },

    #[command(name = "l", aliases = ["ls", "list"], about = "List all todos")]
    List {
        #[arg(short, long, help = "Include tasks marked as done")]
        all: bool,

        #[arg(short, long, help = "Sort by date")]
        date: bool
    },

    #[command(name = "f", aliases = ["find"], about = "Find todo")]
    Find {
        keyword: Vec<String>,

        #[arg(short, long, help = "Include tasks marked as done")]
        all: bool,

        #[arg(short, long, help = "Sort by date")]
        date: bool
    },

    #[command(name = "d", aliases = ["done"], about = "Mark todo as done")]
    Done {
        todo_id: i32,
    },

    #[command(name = "u", aliases = ["undone"], about = "Mark todo as undone")]
    UNDONE {
        todo_id: i32,
    },

    #[command(name = "r", aliases = ["remove"], about = "Remove todo")]
    Remove {
        todo_id: i32,
    },

    #[command(name = "rs", aliases = ["reset"], about = "Reset todos")]
    Reset,

    #[command(name = "t", aliases = ["timer"], about = "Start timer")]
    Timer {
        todo_id: i32
    },
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
                let input = match user_input("Enter a command (type help for more commands): ") {
                    Ok(input) => input,
                    Err(_) => {
                        log("Failed to read input, stopping...");
                        break;
                    }
                };

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
                        if input.trim() == "help" {
                            handle_help()
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
        Command::List { all , date } => {
            handle_list(&tdb, all, date)
        }
        Command::Find { keyword, all, date } => {
            let joined_keyword = keyword.join(" ");
            let keyword = joined_keyword.trim();
            handle_find(&tdb, &keyword, all, date)
        }
        Command::Done { todo_id } => {
            handle_done(&tdb, todo_id);
        }
        Command::UNDONE { todo_id } => {
            handle_undone(&tdb, todo_id);
        }
        Command::Remove { todo_id } => {
            handle_remove(&tdb, todo_id);
        }
        Command::Reset => {
            handle_reset(&tdb);
        }
        _ => {}
    }
}
