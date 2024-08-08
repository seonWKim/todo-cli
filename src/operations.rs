use crate::database::{Todo, TodoDatabase};

pub fn add_todo(tdb: &TodoDatabase, todo: &str, priority: Option<i32>) {
    tdb.add_todo(todo, priority).expect("Failed to add todo");
}

pub fn update_todo(tdb: &TodoDatabase, todo_id: i32, todo: &str) {
    tdb.update_todo(todo_id, todo).expect("Failed to update todo");
}

pub fn list_todos(tdb: &TodoDatabase, include_all: bool) -> Vec<Todo> {
    tdb.list_todos(include_all).expect("Failed to list todos")
}

pub fn find_todos(tdb: &TodoDatabase, keyword: &str, include_all: bool) -> Vec<Todo> {
    return tdb.find_todos(keyword, include_all).expect("Failed to find todos");
}

pub fn mark_todo_as_done(tdb: &TodoDatabase, ids: &Vec<i32>) -> bool {
    tdb.mark_as_done(ids).is_ok()
}

pub fn mark_todo_as_undone(tdb: &TodoDatabase, ids: &Vec<i32>) -> bool {
    tdb.mark_as_undone(ids).is_ok()
}

pub fn remove_todos_by_ids(tdb: &TodoDatabase, ids: &Vec<i32>) -> bool {
    tdb.remove_todos_by_ids(ids).is_ok()
}

pub fn remove_todos_until_date(tdb: &TodoDatabase, date: &str) -> bool {
    tdb.remove_todos_until_date(date).is_ok()
}

pub fn reset_todo(tdb: &TodoDatabase) -> bool {
    tdb.reset().is_ok()
}
