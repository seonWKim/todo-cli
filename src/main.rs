use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "add", alias = "a")]
    Add {
        task: Vec<String>,
    },

    #[command(name = "list", alias = "ls")]
    List,

    #[command(name = "done", alias = "d")]
    Done {
        task: Vec<String>,
    },

    #[command(name = "remove", alias = "rm")]
    Remove {
        task: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

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
