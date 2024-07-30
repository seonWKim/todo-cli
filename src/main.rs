use std::io;
use std::io::Write;
use clap::{Parser, Subcommand};

use crate::database::TodoDatabase;
use crate::utils::log;

mod database;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
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
    RemoveAll
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
        Command::Add { task } => {
            let todo = task.join(" ");
            if todo.is_empty() {
                log("Todo cannot be empty");
                return;
            }
            tdb.add_todo(&todo).expect("Failed to add todo");
            log(&format!("Added task: {}", todo));
        }
        Command::List { all } => {
            let todos = tdb.list_todos(all).expect("Failed to list todos");
            for todo in &todos {
                log(&format!("{}: {}", todo.id, todo.title));
            }

            log("Select a todo by entering the corresponding id: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input: usize = input.trim().parse().expect("Invalid input");

            if input < todos.len() {
                let selected_todo = &todos[input];
                log(&format!("You selected: {}: {}", selected_todo.id, selected_todo.title));
            } else {
                log("Invalid selection");
            }
        }
        Command::Done { task } => {
            tdb.mark_as_done(task).expect("Failed to mark todo as done");
            log(&format!("Marked task {} as done", task));
        }
        Command::Remove { task } => {
            tdb.remove_todo(task).expect("Failed to remove todo");
            log(&format!("Removed task {}", task));
        }
        Command::RemoveAll => {
            // TODO: confirm user
            tdb.remove_all_todos().expect("Failed to remove all todos");
            log("Removed all tasks");
        }
    }
}
