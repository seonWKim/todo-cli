use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Command>,
}

#[derive(Subcommand, PartialEq, Debug)]
pub enum Command {
    #[command(
        name = "i", aliases = ["interactive"], about = "Start interactive mode"
    )]
    Interactive {},

    #[command(name = "a", aliases = ["add"], about = "Add new todo")]
    Add {
        todo: Vec<String>,

        #[arg(short, long, help = "Priority of the todo")]
        priority: Option<i32>,
    },

    #[command(name = "u", aliases = ["update"], about = "Update todo")]
    Update {
        #[arg(help = "Todo id to update")]
        id: i32,

        #[arg(short = 't', long = "todo", help = "New todo text", num_args = 1..)]
        todo: Vec<String>,
    },

    #[command(name = "l", aliases = ["ls", "list"], about = "List all todos")]
    List {
        #[arg(short, long, help = "Include tasks marked as done")]
        all: bool,

        #[arg(short, long, help = "Sort by date")]
        date: bool,
    },

    #[command(name = "f", aliases = ["find"], about = "Find todo")]
    Find {
        keyword: Vec<String>,

        #[arg(short, long, help = "Include tasks marked as done")]
        all: bool,

        #[arg(short, long, help = "Sort by date")]
        date: bool,
    },

    #[command(name = "d", aliases = ["done"], about = "Mark todo as done")]
    Done {
        #[arg(help = "Todo ids to update")]
        ids: Vec<i32>,
    },

    #[command(name = "undone", about = "Mark todo as undone")]
    UNDONE {
        #[arg(help = "Todo ids to update")]
        ids: Vec<i32>,
    },

    #[command(name = "r", aliases = ["remove"], about = "Remove todo")]
    Remove {
        #[arg(help = "Todo ids to update")]
        ids: Vec<i32>,
    },

    #[command(name = "rs", aliases = ["reset"], about = "Reset todos")]
    Reset,

    #[command(name = "t", aliases = ["timer"], about = "Start timer")]
    Timer {
        #[arg(help = "Minutes to run the timer")]
        minutes: u64,
    },
}
