use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a task to the list
    #[command(name = "add", alias = "a")]
    Add {
        /// The task to add
        task: Vec<String>,
    },
    /// List all tasks
    #[command(name = "list", alias = "ls")]
    List,
    /// Mark a task as done
    #[command(name = "done", alias = "d")]
    Done {
        /// The task to mark as done
        task: Vec<String>,
    },
    /// Remove a task from the list
    #[command(name = "remove", alias = "rm")]
    Remove {
        /// The task to remove
        task: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // Handle subcommands
    match &cli.command {
        Some(Commands::Add { task }) => {
            println!("Adding task: {}", task.join(", "));
            // Add logic to add the task
        }
        Some(Commands::List) => {
            println!("Listing all tasks");
            // Add logic to list all tasks
        }
        Some(Commands::Done { task }) => {
            println!("Marking task as done: {}", task.join(", "));
            // Add logic to mark the task as done
        }
        Some(Commands::Remove { task }) => {
            println!("Removing task: {}", task.join(", "));
            // Add logic to remove the task
        }
        None => {}
    }

    // Continued program logic goes here...
}
