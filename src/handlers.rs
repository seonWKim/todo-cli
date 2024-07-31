use clap::CommandFactory;

use crate::Cli;
use crate::database::TodoDatabase;
use crate::utils::log;

pub fn handle_add(tdb: &TodoDatabase, todo: &str) {
    if todo.is_empty() {
        log("Todo cannot be empty");
        return;
    }
    tdb.add_todo(todo).expect("Failed to add todo");
    log(&format!("Added task: {}", todo));
}

pub fn handle_list(tdb: &TodoDatabase, include_done: bool) {
    let todos = tdb.list_todos(include_done).expect("Failed to list todos");
    for todo in &todos {
        log(&format!("{}: {}", todo.id, todo.title));
    }
}

pub fn handle_done(tdb: &TodoDatabase, id: i32) {
    tdb.mark_as_done(id).expect("Failed to mark todo as done");
    log(&format!("Marked todo {} as done", id));
}

pub fn handle_remove(tdb: &TodoDatabase, id: i32) {
    tdb.remove_todo(id).expect("Failed to remove todo");
    log(&format!("Removed todo {}", id));
}

pub fn handle_remove_all(tdb: &TodoDatabase) {
    tdb.remove_all_todos().expect("Failed to remove all todos");
    log("Removed all todos");
}

pub fn handle_help() {
    Cli::command().print_long_help().unwrap();
}
