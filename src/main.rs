use clap::{Parser, Subcommand};

use crate::database::initialize;

mod database;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "add", alias = "a", about = "Add a new task")]
    Add {
        task: Vec<String>,
    },

    #[command(name = "list", alias = "ls", about = "List all tasks")]
    List,

    #[command(name = "done", alias = "d", about = "Mark a task as done")]
    Done {
        task: Vec<String>,
    },

    #[command(name = "remove", alias = "rm", about = "Remove a task")]
    Remove {
        task: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    initialize().expect("Database is not initialized");

    match &cli.command {
        Some(Commands::Add { task }) => {
            println!("Adding task: {}", task.join(", "));
        }
        Some(Commands::List) => {
            println!("Listing all tasks");
        }
        Some(Commands::Done { task }) => {
            println!("Marking task as done: {}", task.join(", "));
        }
        Some(Commands::Remove { task }) => {
            println!("Removing task: {}", task.join(", "));
        }
        None => {}
    }
}
