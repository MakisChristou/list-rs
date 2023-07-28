use rusqlite::{types::FromSql, Connection, Result};
use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
enum TaskStatus {
    Done,
    Undone,
    Archived,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Archived => write!(f, "Archived"),
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::Undone => write!(f, "Undone"),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Archived" => Ok(TaskStatus::Archived),
            "Done" => Ok(TaskStatus::Done),
            "Undone" => Ok(TaskStatus::Undone),
            _ => panic!("Invalid task status"),
        }
    }
}

impl FromSql for TaskStatus {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.as_str()? {
            "Done" => Ok(TaskStatus::Done),
            "Undone" => Ok(TaskStatus::Undone),
            "Archived" => Ok(TaskStatus::Archived),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug)]
struct Task {
    id: i32,
    title: String,
    text: String,
    status: TaskStatus,
    tag: Option<String>,
    due_date: Option<String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: Default::default(),
            title: Default::default(),
            text: Default::default(),
            status: TaskStatus::Undone,
            tag: Default::default(),
            due_date: Default::default(),
        }
    }
}

fn main() -> Result<()> {
    let conn = Connection::open("tasks.db")?;

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
    )?;
    let default_task = Task {
        id: 0,
        title: "Task title".to_string(),
        text: "Hello world!".to_string(),
        status: TaskStatus::Undone,
        tag: None,
        due_date: None,
    };
    conn.execute(
        "INSERT INTO Tasks (title, text, status, tag, due_date) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &default_task.title,
            &default_task.text,
            &default_task.status.to_string(),
            &default_task.tag,
            &default_task.due_date,
        ),
    )?;

    let mut stmt = conn.prepare("SELECT id, title, text, status, tag, due_date FROM Tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            text: row.get(2)?,
            status: row.get(3)?,
            tag: row.get(4)?,
            due_date: row.get(5)?,
        })
    })?;

    for task in task_iter {
        println!("Found task {:?}", task.unwrap());
    }
    Ok(())
}
