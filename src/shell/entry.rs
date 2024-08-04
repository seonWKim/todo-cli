use clap::Parser;

use crate::command::{Cli, Command};
use crate::database::TodoDatabase;
use crate::shell::handlers::{handle_add, handle_done, handle_find, handle_help, handle_list, handle_remove, handle_reset, handle_timer, handle_undone, handle_update};
use crate::utils::{log, user_input};

pub fn start() {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => handle_command(command),
        None => log("No command provided"),
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
        Command::Add { todo, priority } => {
            let todo = todo.join(" ");
            handle_add(&tdb, &todo, priority);
        }
        Command::Update { id, todo } => {
            let todo = todo.join(" ");
            handle_update(&tdb, id, &todo);
        }
        Command::List { all, date } => {
            handle_list(&tdb, all, date)
        }
        Command::Find { keyword, all, date } => {
            let joined_keyword = keyword.join(" ");
            let keyword = joined_keyword.trim();
            handle_find(&tdb, &keyword, all, date)
        }
        Command::Done { ids } => {
            handle_done(&tdb, &ids);
        }
        Command::UNDONE { ids } => {
            handle_undone(&tdb, &ids);
        }
        Command::Remove { ids } => {
            handle_remove(&tdb, &ids);
        }
        Command::Reset => {
            handle_reset(&tdb);
        }
        Command::Timer { minutes } => {
            handle_timer(minutes);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_interactive() {
        let args = vec!["tc", "i"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::Interactive {}));
    }

    #[test]
    fn parse_add() {
        let args = vec!["tc", "a", "new", "todo"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::Add { todo: vec!["new".to_string(), "todo".to_string()], priority: None }));
    }

    #[test]
    fn parse_update() {
        let args = vec!["tc", "u", "1", "-t", "updated", "todo"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::Update { id: 1, todo: vec!["updated".to_string(), "todo".to_string()] }));
    }

    #[test]
    fn parse_list() {
        let args = vec!["tc", "l"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::List { all: false, date: false }));
    }

    #[test]
    fn parse_list_all() {
        let args = vec!["tc", "l", "-a"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::List { all: true, date: false }));
    }

    #[test]
    fn parse_list_date() {
        let args = vec!["tc", "l", "-d"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::List { all: false, date: true }));
    }

    #[test]
    fn parse_find() {
        let args = vec!["tc", "f", "keyword"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::Find { keyword: vec!["keyword".to_string()], all: false, date: false }));
    }

    #[test]
    fn parse_find_all() {
        let args = vec!["tc", "f", "keyword", "-a"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::Find { keyword: vec!["keyword".to_string()], all: true, date: false }));
    }

    #[test]
    fn parse_find_date() {
        let args = vec!["tc", "f", "keyword", "-d"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::Find { keyword: vec!["keyword".to_string()], all: false, date: true }));
    }

    #[test]
    fn parse_done() {
        let args = vec!["tc", "d", "1", "2"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::Done { ids: vec![1, 2] }));
    }

    #[test]
    fn parse_undone() {
        let args = vec!["tc", "undone", "1", "2"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::UNDONE { ids: vec![1, 2] }));
    }

    #[test]
    fn parse_remove() {
        let args = vec!["tc", "r", "1", "2"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::Remove { ids: vec![1, 2] }));
    }

    #[test]
    fn parse_reset() {
        let args = vec!["tc", "rs"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::Reset));
    }

    #[test]
    fn parse_timer() {
        let args = vec!["tc", "t", "10"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.command, Some(Command::Timer { minutes: 10 }));
    }
}
