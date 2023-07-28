use rusqlite::{types::FromSql, Connection, Result};

mod task;
use crate::task::Task;
use crate::task::TaskStatus;

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
    let person_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            text: row.get(2)?,
            status: row.get(3)?,
            tag: row.get(4)?,
            due_date: row.get(5)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
