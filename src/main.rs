use std::io;
use std::io::Write;

use clap::{Parser, Subcommand};

use crate::database::TodoDatabase;
use crate::handlers::{handle_add, handle_done, handle_list, handle_remove, handle_remove_all};
use crate::utils::request_user_input;

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
        name = "interactive", aliases = ["i", "interactive"], about = "Start interactive mode"
    )]
    Interactive {},

    #[command(name = "add", aliases = ["a", "add"], about = "Add a new task")]
    Add {
        task: Vec<String>,
    },

    #[command(name = "list", aliases = ["l", "ls", "list"], about = "List all tasks")]
    List {
        #[arg(short, long, help = "Include tasks marked as done")]
        all: bool
    },

    #[command(name = "done", aliases = ["d", "done"], about = "Mark a task as done")]
    Done {
        task: i32,
    },

    #[command(name = "remove", aliases = ["r", "rm", "remove"], about = "Remove a task")]
    Remove {
        task: i32,
    },
    #[command(name = "remove all", aliases = ["removeAll"], about = "Remove all tasks")]
    RemoveAll,
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
            request_user_input("Enter a command: ");
            //
            // let mut input = String::new();
            // io::stdin().read_line(&mut input).expect("Failed to read input");
            //
            // let input = input.trim().parse().expect("Invalid input");
        }
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
    }
}
