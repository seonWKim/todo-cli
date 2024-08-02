use std::fs;

use rusqlite::{Connection, params, Result};
use crate::utils::log;

pub struct TodoDatabase {
    db_dir_path: String,
    db_name: String,
    db_todo_table_ddl: String,
    db_todo_index_ddl: String,
}

impl TodoDatabase {
    pub fn new() -> TodoDatabase {
        Self::new0(
            format!("{}/.tc", std::env::var("HOME").unwrap()),
            "todo.db".to_string()
        )
    }

    fn new0(
        db_dir_path: String,
        db_name: String
    ) -> TodoDatabase {
        TodoDatabase {
            db_dir_path,
            db_name,
            db_todo_table_ddl: r#"
            CREATE TABLE IF NOT EXISTS todos
            (
                id         INTEGER PRIMARY KEY,
                title      TEXT            NOT NULL, -- title of the todo
                done       BOOLEAN         NOT NULL, -- whether the todo is done or not
                priority   INTEGER         NOT NULL DEFAULT 0, -- priority of the todo
                created_at DATE            NOT NULL,
                updated_at DATE            NOT NULL
            )
            "#.to_string(),
            db_todo_index_ddl: r#"
            CREATE INDEX IF NOT EXISTS idx_todos_done ON todos (done)
            "#.to_string(),
        }
    }

    pub fn get_db_path(&self) -> String {
        format!("{}/{}", self.db_dir_path, self.db_name)
    }

    pub fn initialize(&self) -> Result<()> {
        if self.is_initialized() {
            return Ok(());
        }

        match fs::create_dir_all(&self.db_dir_path) {
            Ok(_) => {
                log(&format!("Created directory: {}", self.db_dir_path));
            }
            Err(_) => {
                panic!("Failed to create directory: {}, stopping todo-cli", self.db_dir_path);
            }
        }

        let conn = Connection::open(self.get_db_path())?;

        conn.execute(&*self.db_todo_table_ddl, [])?;
        conn.execute(&*self.db_todo_index_ddl, [])?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn teardown(&self) -> Result<()> {
        let db_path = self.get_db_path();
        if fs::metadata(&db_path).is_ok() {
            match fs::remove_file(&db_path) {
                Ok(_) => {
                    log(&format!("Removed database: {}", db_path));
                }
                Err(_) => {
                    panic!("Failed to remove database: {}, stopping todo-cli", db_path);
                }
            }
        }

        Ok(())
    }

    fn is_initialized(&self) -> bool {
        let db_path = self.get_db_path();
        fs::metadata(db_path).is_ok()
    }

    pub fn add_todo(&self, todo: &str, priority: Option<i32>) -> Result<()> {
        let conn = Connection::open(self.get_db_path())?;
        let now = chrono::Local::now().to_rfc3339();

        // Mark todo with same name as done
        conn.execute(
            "UPDATE todos SET done = ?1, updated_at = ?2 WHERE title = ?3",
            params![true, now, todo],
        )?;

        conn.execute(
            "INSERT INTO todos (title, done, priority, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![todo, false, priority.unwrap_or(0), now, now],
        )?;

        Ok(())
    }

    pub fn update_todo(&self, id: i32, todo: &str) -> Result<()> {
        let conn = Connection::open(self.get_db_path())?;
        let now = chrono::Local::now().to_rfc3339();

        conn.execute(
            "UPDATE todos SET title = ?1, updated_at = ?2 WHERE id = ?3",
            params![todo, now, id],
        )?;

        Ok(())
    }

    pub fn list_todos(&self, include_all: bool) -> Result<Vec<Todo>> {
        let conn = Connection::open(self.get_db_path())?;
        let sql = if include_all {
            "SELECT id, title, done, priority, created_at, updated_at FROM todos"
        } else {
            "SELECT id, title, done, priority, created_at, updated_at FROM todos WHERE done = ?1"
        };
        let mut stmt = conn.prepare(sql)?;

        let params = if include_all { params![] } else { params![false] };
        let todos = stmt
            .query_map(params, |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    done: row.get(2)?,
                    priority: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })?
            .map(|r| r.unwrap())
            .collect();

        Ok(todos)
    }

    pub fn find_todos(&self, keyword: &str, include_all: bool) -> Result<Vec<Todo>> {
        let conn = Connection::open(self.get_db_path())?;
        let sql = if include_all {
            format!("SELECT id, title, done, priority, created_at, updated_at FROM todos WHERE title LIKE '%{}%'", keyword)
        } else {
            format!("SELECT id, title, done, priority, created_at, updated_at FROM todos WHERE title LIKE '%{}%' AND done = ?1", keyword)
        };
        let mut stmt = conn.prepare(sql.as_str())?;

        let params = if include_all { params![] } else { params![false] };
        let todos = stmt
            .query_map(params, |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    done: row.get(2)?,
                    priority: row.get(3)?, 
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })?
            .map(|r| r.unwrap())
            .collect();

        Ok(todos)
    }

    pub fn mark_as_done(&self, id: i32) -> Result<()> {
        let conn = Connection::open(self.get_db_path())?;
        let now = chrono::Local::now().to_rfc3339();

        conn.execute(
            "UPDATE todos SET done = ?1, updated_at = ?2 WHERE id = ?3",
            params![true, now, id],
        )?;

        Ok(())
    }

    pub fn mark_as_undone(&self, id: i32) -> Result<()> {
        let conn = Connection::open(self.get_db_path())?;
        let now = chrono::Local::now().to_rfc3339();

        conn.execute(
            "UPDATE todos SET done = ?1, updated_at = ?2 WHERE id = ?3",
            params![false, now, id],
        )?;

        Ok(())
    }

    pub fn remove_todo(&self, id: i32) -> Result<()> {
        let conn = Connection::open(self.get_db_path())?;

        conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;

        Ok(())
    }

    pub fn reset(&self) -> Result<()> {
        let conn = Connection::open(self.get_db_path())?;
        conn.execute("DELETE FROM todos", [])?;
        Ok(())
    }
}


#[derive(Clone, Debug)]
pub struct Todo {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) done: bool,
    pub(crate) priority: i32,
    pub(crate) created_at: String,
    #[allow(dead_code)]
    updated_at: String,
}


#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[allow(dead_code)]
    fn setup_test_db(db_name: &str) -> TodoDatabase {
        let db_path = format!("{}/.tc_test", env::var("HOME").unwrap()).to_string();
        let tdb = TodoDatabase::new0(db_path, db_name.to_string());
        tdb.initialize().expect("Failed to initialize test database");

        return tdb;
    }

    #[allow(dead_code)]
    fn tear_down_test_db(tdb: &TodoDatabase) {
        tdb.teardown().expect("Failed to teardown test database");
    }

    #[test]
    fn test_add_and_list_todos() {
        let tdb = setup_test_db("test_add_and_list_todos.db");

        let todo1 = "Test Todo 1".to_string();
        let todo2 = "Test Todo 2".to_string();
        let todo3 = "Test Todo 3".to_string();

        tdb.add_todo(&todo1, None).unwrap();
        tdb.add_todo(&todo2, None).unwrap();
        tdb.add_todo(&todo3, Some(1)).unwrap();

        let todos = tdb.list_todos(false).unwrap();
        assert_eq!(todos.len(), 3);
        assert_eq!(todos[0].title, todo1);
        assert_eq!(todos[1].title, todo2);
        assert_eq!(todos[2].title, todo3);
        assert_eq!(todos[2].priority, 1);

        tear_down_test_db(&tdb);
    }

    #[test]
    fn test_add_mark_and_list_todos() {
        let tdb = setup_test_db("test_add_mark_and_list_todos.db");

        let todo = "Test Todo".to_string();
        tdb.add_todo(&todo, None).unwrap();

        let todos = tdb.list_todos(false).unwrap();
        assert_eq!(todos.len(), 1);
        let todo_id = todos[0].id;

        tdb.mark_as_done(todo_id).unwrap();

        let todos = tdb.list_todos(false).unwrap();
        assert_eq!(todos.len(), 0);

        tear_down_test_db(&tdb);
    }

    #[test]
    fn test_undone_todo_should_be_shown() {
        let tdb = setup_test_db("test_undone_todo_should_be_shown.db");

        let todo = "Test Todo".to_string();
        tdb.add_todo(&todo, None).unwrap();

        let todos = tdb.list_todos(false).unwrap();
        assert_eq!(todos.len(), 1);
        let todo_id = todos[0].id;

        tdb.mark_as_done(todo_id).unwrap();

        let todos = tdb.list_todos(false).unwrap();
        assert_eq!(todos.len(), 0);

        tdb.mark_as_undone(todo_id).unwrap();

        let todos = tdb.list_todos(false).unwrap();
        assert_eq!(todos.len(), 1);

        tear_down_test_db(&tdb);
    }

    #[test]
    fn test_add_remove_and_list_todos() {
        let tdb = setup_test_db("test_add_remove_and_list_todos.db");

        let todo = "Test Todo".to_string();
        tdb.add_todo(&todo, None).unwrap();

        let todos = tdb.list_todos(false).unwrap();
        assert_eq!(todos.len(), 1);
        let todo_id = todos[0].id;

        tdb.remove_todo(todo_id).unwrap();

        let todos = tdb.list_todos(false).unwrap();
        assert_eq!(todos.len(), 0);

        tear_down_test_db(&tdb);
    }

    #[test]
    fn test_default_priority_should_be_zero() {
        let tdb = setup_test_db("test_default_priority_should_be_zero.db");

        let todo = "Test Todo".to_string();
        tdb.add_todo(&todo, None).unwrap();

        let todos = tdb.list_todos(false).unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, todo);
        assert_eq!(todos[0].done, false);
        assert_eq!(todos[0].priority, 0);

        tear_down_test_db(&tdb);
    }
}
