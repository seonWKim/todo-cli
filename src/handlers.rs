use std::collections::BTreeMap;

use chrono::NaiveDate;
use clap::CommandFactory;
use colored::Colorize;

use crate::Cli;
use crate::database::{Todo, TodoDatabase};
use crate::utils::{log, user_input};

pub fn handle_add(tdb: &TodoDatabase, todo: &str) {
    if todo.is_empty() {
        log("Todo cannot be empty");
        return;
    }
    tdb.add_todo(todo).expect("Failed to add todo");
    log(&format!("Added task: {}", todo));
}

pub fn handle_list(tdb: &TodoDatabase, include_all: bool, sort_by_date: bool) {
    let todos = tdb.list_todos(include_all).expect("Failed to list todos");

    if sort_by_date {
        let mut grouped_todos: BTreeMap<NaiveDate, Vec<&Todo>> = BTreeMap::new();
        for todo in &todos {
            let date = NaiveDate::parse_from_str(&todo.created_at[..10], "%Y-%m-%d").expect("Invalid date format");
            grouped_todos.entry(date).or_insert_with(Vec::new).push(todo);
        }

        for (_, group) in grouped_todos.iter_mut() {
            group.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        }

        let total_groups = grouped_todos.len();
        for (index, (date, group)) in grouped_todos.iter().rev().enumerate() {
            log(&format!("Date: {}", date).green().to_string());
            group.iter().for_each(|todo| print_todo(todo, include_all));

            if index != total_groups - 1 {
                println!();
            }
        }
    } else {
        let mut todos_sorted = todos.clone();
        todos_sorted.sort_by(|a, b| b.id.cmp(&a.id));
        todos_sorted.iter().for_each(|todo| print_todo(todo, include_all));
    }
}

fn print_todo(todo: &Todo, show_mark: bool) {
    if show_mark {
        let mark = if todo.done { "✔" } else { " " };
        log(&format!("({}) {} [{}]", todo.id, todo.title, mark));
    } else {
        log(&format!("({}) {}", todo.id, todo.title));
    }
}

pub fn handle_find(tdb: &TodoDatabase, keyword: &str, include_all: bool, sort_by_date: bool) {
    let todos = tdb.find_todos(keyword, include_all).expect("Failed to find todos");

    if sort_by_date {
        let mut grouped_todos: BTreeMap<NaiveDate, Vec<&Todo>> = BTreeMap::new();
        for todo in &todos {
            let date = NaiveDate::parse_from_str(&todo.created_at[..10], "%Y-%m-%d").expect("Invalid date format");
            grouped_todos.entry(date).or_insert_with(Vec::new).push(todo);
        }

        for (_, group) in grouped_todos.iter_mut() {
            group.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        }

        let total_groups = grouped_todos.len();
        for (index, (date, group)) in grouped_todos.iter().rev().enumerate() {
            log(&format!("Date: {}", date).green().to_string());
            for todo in group {
                print_todo_highlightened(todo, keyword, include_all);
            }

            if index != total_groups - 1 {
                println!();
            }
        }
    } else {
        let mut todos_sorted = todos.clone();
        todos_sorted.sort_by(|a, b| b.id.cmp(&a.id));
        for todo in todos_sorted {
            print_todo_highlightened(&todo, keyword, include_all);
        }
    }
}

fn print_todo_highlightened(todo: &Todo, keyword: &str, show_mark: bool) {
    if show_mark {
        let mark = if todo.done { "✅" } else { " " };
        let highlightened_title = todo.title.replace(keyword, &format!("{}", keyword).red().to_string());
        log(&format!("({}) {} [{}]", todo.id, highlightened_title, mark));
    } else {
        let highlightened_title = todo.title.replace(keyword, &format!("{}", keyword).red().to_string());
        log(&format!("({}) {}", todo.id, highlightened_title));
    }
}

pub fn handle_done(tdb: &TodoDatabase, id: i32) {
    tdb.mark_as_done(id).expect("Failed to mark todo as done");
    log(&format!("Marked todo {} as done", id));
}

pub fn handle_undone(tdb: &TodoDatabase, id: i32) {
    tdb.mark_as_undone(id).expect("Failed to mark todo as undone");
    log(&format!("Marked todo {} as undone", id));
}

pub fn handle_remove(tdb: &TodoDatabase, id: i32) {
    tdb.remove_todo(id).expect("Failed to remove todo");
    log(&format!("Removed todo {}", id));
}

pub fn handle_reset(tdb: &TodoDatabase) {
    let input = user_input("Are you sure you want to remove all todos(yes/no)?: ").expect("Failed to read input");
    if input.trim() != "yes" {
        log("Reset aborted");
        return;
    }

    tdb.reset().expect("Failed to remove all todos");
    log("Removed all todos");
}

pub fn handle_help() {
    Cli::command().print_long_help().unwrap();
}
