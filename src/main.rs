use clap::{Parser, Subcommand};

use crate::database::TodoDatabase;

mod database;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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
    let tdb = TodoDatabase::new();
    tdb.initialize().expect("Database is not initialized");

    match &cli.command {
        Some(Commands::Add { task }) => {
            let todo = task.join(" ");
            if todo.is_empty() {
                println!("[tc] Todo cannot be empty");
                return;
            }
            tdb.add_todo(&todo).expect("Failed to add todo");
            println!("[tc] Added task: {}", todo);
        }
        Some(Commands::List { all }) => {
            let todos = tdb.list_todos(*all).expect("Failed to list todos");
            for todo in todos {
                println!("[tc] {}: {}", todo.id, todo.title);
            }
        }
        Some(Commands::Done { task }) => {
            tdb.mark_as_done(task).expect("Failed to mark todo as done");
            println!("[tc] Marked task {} as done", task);
        }
        Some(Commands::Remove { task }) => {
            tdb.remove_todo(task).expect("Failed to remove todo");
            println!("[tc] Removed task {}", task);
        }
        Some(Commands::RemoveAll) => {
            // TODO: confirm user
            tdb.remove_all_todos().expect("Failed to remove all todos");
            println!("[tc] Removed all tasks");
        }
        None => {}
    }
}
