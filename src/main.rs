use rusqlite::{types::FromSql, Result};

mod db_handler;
mod task;

use crate::db_handler::DatabaseHandler;
use crate::task::Task;
use crate::task::TaskStatus;

fn main() -> Result<()> {
    let db_handler = DatabaseHandler::new("tasks.db");

    let default_task = Task {
        id: 0,
        title: "Task title".to_string(),
        text: "Hello world!".to_string(),
        status: TaskStatus::Undone,
        tag: None,
        due_date: None,
    };

    db_handler.create_task(default_task);

    let tasks = db_handler.read_tasks();

    println!("All Tasks: {:?}", tasks);

    Ok(())
}
