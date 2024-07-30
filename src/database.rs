use std::fs;

use rusqlite::{Connection, params, Result};

const DB_NAME: &str = "todo.db";

pub fn initialize() -> Result<()> {
    if is_initialized() {
        return Ok(());
    }

    let db_path = format!("{}/.tc", std::env::var("HOME").unwrap());
    match fs::create_dir_all(&db_path) {
        Ok(_) => {
            println!("Created directory: {}", db_path);
        }
        Err(_) => {
            panic!("Failed to create directory: {}, stopping todo-cli", db_path);
        }
    }

    let conn = Connection::open(format!("{}/{}", db_path, DB_NAME))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
                  id INTEGER PRIMARY KEY,
                  title TEXT NOT NULL,
                  created_at DATE NOT NULL,
                  updated_at DATE NOT NULL,
                  done BOOLEAN NOT NULL
                  )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_todos_done ON todos (done)",
        [],
    )?;

    Ok(())
}

fn is_initialized() -> bool {
    let db_path = get_db_path();
    fs::metadata(db_path).is_ok()
}

#[derive(Debug)]
pub struct Todo {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) created_at: String,
    updated_at: String,
    done: bool,
}

pub fn add_todo(todo: &String) -> Result<()> {
    let conn = Connection::open(get_db_path())?;
    let now = chrono::Local::now().to_rfc3339();

    conn.execute(
        "INSERT INTO todos (title, created_at, updated_at, done) VALUES (?1, ?2, ?3, ?4)",
        params![todo, now, now, false],
    )?;

    Ok(())
}

pub fn list_todos(include_done: bool) -> Result<Vec<Todo>> {
    let conn = Connection::open(get_db_path())?;
    let mut stmt = conn.prepare("SELECT id, title, created_at, updated_at, done FROM todos WHERE done = ?1")?;

    let todos = stmt
        .query_map(params![include_done], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
                done: row.get(4)?,
            })
        })?
        .map(|r| r.unwrap())
        .collect();

    Ok(todos)
}

fn get_db_path() -> String {
    format!("{}/.tc/{}", std::env::var("HOME").unwrap(), DB_NAME)
}
