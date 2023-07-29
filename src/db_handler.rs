use rusqlite::{types::FromSql, Connection, Result};

use crate::task::Task;

pub struct DatabaseHandler {
    pub conn: Connection,
}

impl DatabaseHandler {
    pub fn new(database_path: &str) -> Self {
        let conn = Connection::open(database_path).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Tasks (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                title       TEXT NOT NULL,
                text        TEXT NOT NULL,
                status      TEXT NOT NULL,
                tag         TEXT,
                due_date    TEXT
            )",
            (), // empty list of parameters.
        )
        .unwrap();

        DatabaseHandler { conn }
    }

    pub fn create_task(&self, task: Task) {
        self.conn.execute(
            "INSERT INTO Tasks (title, text, status, tag, due_date) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &task.title,
                &task.text,
                &task.status.to_string(),
                &task.tag,
                &task.due_date,
            ),
        ).unwrap();
    }

    pub fn read_tasks(&self) -> Vec<Task> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, text, status, tag, due_date FROM Tasks")
            .unwrap();
        let task_iter = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    text: row.get(2)?,
                    status: row.get(3)?,
                    tag: row.get(4)?,
                    due_date: row.get(5)?,
                })
            })
            .unwrap();

        let mut tasks = Vec::new();

        for task in task_iter {
            tasks.push(task.unwrap());
        }

        tasks
    }

    pub fn read_task(&self, id: i32) -> Option<Task> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, text, status, tag, due_date FROM Tasks WHERE id = ?1")
            .unwrap();
        let task_iter = stmt
            .query_map([id], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    text: row.get(2)?,
                    status: row.get(3)?,
                    tag: row.get(4)?,
                    due_date: row.get(5)?,
                })
            })
            .unwrap();

        for task in task_iter {
            return Some(task.unwrap());
        }

        None
    }
}
