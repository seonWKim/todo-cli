use std::fs;

use rusqlite::{Connection, Result};

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

    let conn = Connection::open(format!("{}/todo.db", db_path))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL,
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
    let db_path = format!("{}/.tc/todo.db", std::env::var("HOME").unwrap());
    fs::metadata(db_path).is_ok()
}

fn insert_task
