use std::str::FromStr;

use crate::task::{Task, TaskStatus};
use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::{params, Connection, Error};
pub struct DatabaseHandler {
    pub conn: Connection,
}

impl DatabaseHandler {
    fn create_tables_if_not_exist(conn: &Connection) {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Tasks (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                text        TEXT NOT NULL,
                status      TEXT NOT NULL,
                tag         TEXT,
                due_date    TEXT,
                created_at  TEXT
            )",
            (), // empty list of parameters.
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS UndoHistory (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                command     TEXT NOT NULL,
                created_at  TEXT,
                task_id          INTEGER,
                task_text        TEXT NOT NULL,
                task_status      TEXT NOT NULL,
                task_tag         TEXT,
                task_due_date    TEXT,
                task_created_at  TEXT 
            )",
            (), // empty list of parameters.
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS RedoHistory (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                command        TEXT NOT NULL,
                created_at  TEXT
            )",
            (), // empty list of parameters.
        )
        .unwrap();
    }

    fn push_update_to_undo_history(
        &self,
        id: i32,
        previous_task: Task,
        new_task: Task,
    ) -> rusqlite::Result<()> {
        let undo_query = format!(
            "UPDATE Tasks SET text = '{}', status = '{}', tag = {}, due_date = {}, created_at = '{}' WHERE id = {}",
            previous_task.text,
            previous_task.status.to_string(),
            match &previous_task.tag {
                Some(t) => format!("'{}'", t),
                None => "NULL".to_string(),
            },
            match &previous_task.due_date {
                Some(d) => format!("'{}'", d),
                None => "NULL".to_string(),
            },
            previous_task.created_at,
            id
        );

        self.conn.execute(
            "INSERT INTO UndoHistory (command, created_at, task_id, task_text, task_status, task_tag, task_due_date, task_created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&undo_query, chrono::Local::now().to_string(), new_task.id, new_task.text, new_task.status.to_string(), new_task.tag, new_task.due_date, new_task.created_at),
        )?;

        Ok(())
    }

    fn push_create_to_undo_history(&self, task: Task) -> rusqlite::Result<()> {
        // Add the opposite operation to the UndoHistory
        let undo_query = "DELETE FROM Tasks WHERE id = (SELECT MAX(id) FROM Tasks)";
        self.conn.execute(
            "INSERT INTO UndoHistory (command, created_at, task_id, task_text, task_status, task_tag, task_due_date, task_created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&undo_query, chrono::Local::now().to_string(), task.id, task.text, task.status.to_string(), task.tag, task.due_date, task.created_at),
        )?;

        Ok(())
    }

    fn push_delete_to_undo_history(&self, task: Task, id: i32) -> rusqlite::Result<()> {
        // Add the opposite operation to the UndoHistory
        let undo_query = format!(
            "INSERT INTO Tasks (id, text, status, tag, due_date, created_at) VALUES ({}, '{}', '{}', {}, {}, '{}')",
            &task.id,
            &task.text,
            &task.status.to_string(),
            match &task.tag {
                Some(t) => format!("'{}'", t),
                None => "NULL".to_string(),
            },
            match &task.due_date {
                Some(d) => format!("'{}'", d),
                None => "NULL".to_string(),
            },
            &task.created_at,
        );

        self.conn.execute(
            "INSERT INTO UndoHistory (command, created_at, task_id, task_text, task_status, task_tag, task_due_date, task_created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&undo_query, chrono::Local::now().to_string(), task.id, task.text, task.status.to_string(), task.tag, task.due_date, task.created_at),
        )?;

        Ok(())
    }

    fn update_redo_table(&self, undo_command: &str, task: Task) -> rusqlite::Result<()> {
        if undo_command.starts_with("INSERT") {
            // opposite is DELETE
            self.push_delete_to_redo_history(task)?;
        } else if undo_command.starts_with("DELETE") {
            // Opposite is INSERT
            self.push_create_to_redo_history(task)?;
        } else if undo_command.starts_with("UPDATE") {
            // Opposite is UPDATE
            self.push_update_to_redo_history(task)?;
        } else {
            panic!("Invalid undo command: {}", undo_command);
        }

        Ok(())
    }

    fn push_delete_to_redo_history(&self, task: Task) -> rusqlite::Result<()> {
        // Add the opposite operation to the RedoHistory
        let redo_query = format!("DELETE FROM Tasks WHERE id = {}", task.id);

        self.conn.execute(
            "INSERT INTO RedoHistory (command, created_at) VALUES (?1, ?2)",
            (&redo_query, chrono::Local::now().to_string()),
        )?;

        Ok(())
    }

    fn push_create_to_redo_history(&self, task: Task) -> rusqlite::Result<()> {
        // Add the opposite operation to the RedoHistory
        let undo_query = format!(
            "INSERT INTO Tasks (id, text, status, tag, due_date, created_at) VALUES ({}, '{}', '{}', {}, {}, '{}')",
            &task.id,
            &task.text,
            &task.status.to_string(),
            match &task.tag {
                Some(t) => format!("'{}'", t),
                None => "NULL".to_string(),
            },
            match &task.due_date {
                Some(d) => format!("'{}'", d),
                None => "NULL".to_string(),
            },
            &task.created_at,
        );

        self.conn.execute(
            "INSERT INTO RedoHistory (command, created_at) VALUES (?1, ?2)",
            (&undo_query, chrono::Local::now().to_string()),
        )?;

        Ok(())
    }

    fn push_update_to_redo_history(&self, task: Task) -> rusqlite::Result<()> {
        let redo_query = format!(
            "UPDATE Tasks SET text = '{}', status = '{}', tag = {}, due_date = {}, created_at = '{}' WHERE id = {}",
            task.text,
            task.status.to_string(),
            match &task.tag {
                Some(t) => format!("'{}'", t),
                None => "NULL".to_string(),
            },
            match &task.due_date {
                Some(d) => format!("'{}'", d),
                None => "NULL".to_string(),
            },
            task.created_at,
            task.id
        );

        self.conn.execute(
            "INSERT INTO RedoHistory (command, created_at) VALUES (?1, ?2)",
            (&redo_query, chrono::Local::now().to_string()),
        )?;

        Ok(())
    }

    pub fn new(database_path: &str) -> Self {
        let conn = Connection::open(database_path).unwrap();
        DatabaseHandler::create_tables_if_not_exist(&conn);

        DatabaseHandler { conn }
    }

    pub fn new_in_memory() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        DatabaseHandler::create_tables_if_not_exist(&conn);

        DatabaseHandler { conn }
    }

    pub fn create_task(&self, mut task: Task) -> rusqlite::Result<usize> {
        // Execute create query
        let _ = self.conn.execute(
            "INSERT INTO Tasks (text, status, tag, due_date, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &task.text,
                &task.status.to_string(),
                &task.tag,
                &task.due_date,
                &task.created_at,
            ),
        )?;

        let id = self.conn.last_insert_rowid();
        task.id = id as i32;

        println!("Create_task id: {}", task.id);
        self.push_create_to_undo_history(task)?;

        Ok(id as usize)
    }

    pub fn read_tasks(&self) -> Vec<Task> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, text, status, tag, due_date, created_at FROM Tasks")
            .unwrap();
        let task_iter = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    status: row.get(2)?,
                    tag: row.get(3)?,
                    due_date: row.get(4)?,
                    created_at: row.get(5)?,
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
            .prepare("SELECT id, text, status, tag, due_date, created_at FROM Tasks WHERE id = ?1")
            .unwrap();
        let task_iter = stmt
            .query_map([id], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    status: row.get(2)?,
                    tag: row.get(3)?,
                    due_date: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })
            .unwrap();

        for task in task_iter {
            return Some(task.unwrap());
        }

        None
    }

    pub fn update_task(&self, id: i32, new_task: &Task) -> rusqlite::Result<()> {
        // Save the current state of the task
        if let Some(previous_task) = self.read_task(id) {
            // Execute update query
            self.conn.execute(
                "UPDATE Tasks SET text = ?1, status = ?2, tag = ?3, due_date = ?4, created_at = ?5 WHERE id = ?6",
                params![
                    new_task.text,
                    new_task.status.to_string(),
                    new_task.tag,
                    new_task.due_date,
                    new_task.created_at,
                    id
                ],
            )?;

            self.push_update_to_undo_history(id, previous_task.clone(), new_task.clone())?;

            Ok(())
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub fn delete_task(&self, id: i32) -> rusqlite::Result<()> {
        // Execute delete query
        if let Some(task) = self.read_task(id) {
            self.push_delete_to_undo_history(task.clone(), id)?;

            self.conn.execute("DELETE FROM Tasks WHERE id = ?1", [id])?;

            Ok(())
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub fn undo(&self) -> rusqlite::Result<()> {
        let mut stmt = self
        .conn
        .prepare("SELECT id, command, created_at, task_id, task_text, task_status, task_tag, task_due_date, task_created_at FROM UndoHistory ORDER BY id DESC LIMIT 1")
        .unwrap();

        let mut undo_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i32>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, TaskStatus>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, NaiveDateTime>(8)?,
            ))
        })?;

        if let Some(Ok((
            id,
            undo_command,
            _created_at,
            task_id,
            task_text,
            task_status,
            task_tag,
            task_due_date,
            task_created_at,
        ))) = undo_iter.next()
        {
            match self.conn.execute(&undo_command, []) {
                Ok(_) => {
                    let task = Task::new_with_created_at(
                        task_id,
                        &task_text,
                        task_status,
                        task_tag,
                        task_due_date,
                        task_created_at,
                    );

                    self.update_redo_table(&undo_command, task)?;

                    // If the undo operation is successful, delete the command from the history
                    self.conn
                        .execute("DELETE FROM UndoHistory WHERE id = ?1", params![id])?;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    pub fn redo(&self) -> rusqlite::Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, command, created_at FROM RedoHistory ORDER BY id DESC LIMIT 1")
            .unwrap();

        let mut redo_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        if let Some(Ok((id, redo_command, _created_at))) = redo_iter.next() {
            match self.conn.execute(&redo_command, []) {
                Ok(_) => {
                    // If the undo operation is successful, delete the command from the history
                    self.conn
                        .execute("DELETE FROM RedoHistory WHERE id = ?1", params![id])?;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DatabaseHandler;
    use crate::task::{Task, TaskStatus};

    fn setup_single_task() -> (DatabaseHandler, Task) {
        let db_handler = DatabaseHandler::new_in_memory();
        let expected = Task::new(1, "", TaskStatus::Undone, None, None);
        (db_handler, expected)
    }

    fn setup_multiple_tasks() -> (DatabaseHandler, Vec<Task>) {
        let db_handler = DatabaseHandler::new_in_memory();

        let tasks = vec![
            Task::new(
                1,
                "Buy fruits, vegetables, and bread.",
                TaskStatus::Undone,
                None,
                None,
            ),
            Task::new(
                2,
                "Change oil and check tire pressure.",
                TaskStatus::Undone,
                None,
                None,
            ),
            Task::new(
                3,
                "Read chapter 5 of the history book.",
                TaskStatus::Undone,
                None,
                None,
            ),
            Task::new(
                4,
                "30 minutes of cardio and weight lifting.",
                TaskStatus::Undone,
                None,
                None,
            ),
            Task::new(
                5,
                "Try out the new pasta recipe.",
                TaskStatus::Undone,
                None,
                None,
            ),
        ];

        (db_handler, tasks)
    }

    #[test]
    fn create_task_should_work() {
        let (db_handler, expected) = setup_single_task();
        db_handler.create_task(expected.clone());

        let tasks = db_handler.read_tasks();
        let actual = tasks[0].clone();

        assert_eq!(expected, actual);
    }

    #[test]
    fn delete_task_should_work() {
        let (db_handler, expected) = setup_single_task();
        db_handler.create_task(expected.clone());

        let _ = db_handler.delete_task(1);
        let tasks = db_handler.read_tasks();

        assert_eq!(0, tasks.len());
    }

    #[test]
    fn create_multiple_tasks_should_work() {
        let (db_handler, expected) = setup_multiple_tasks();

        for task in &expected {
            db_handler.create_task(task.clone());
        }

        let actual = db_handler.read_tasks();
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_task_should_work() {
        let (db_handler, mut expected) = setup_multiple_tasks();

        for task in &expected {
            db_handler.create_task(task.clone());
        }

        let _ = db_handler.update_task(1, &Task::default());

        let actual = db_handler.read_tasks();
        expected[0] = Task::default();

        assert_eq!(actual, expected);
    }

    #[test]
    fn undo_create_should_work() {
        let (db_handler, expected) = setup_single_task();

        db_handler.create_task(expected.clone());
        db_handler.undo();

        let actual = db_handler.read_tasks();
        let expected: Vec<Task> = vec![];

        assert_eq!(expected, actual);
    }

    #[test]
    fn undo_delete_should_work() {
        let (db_handler, expected) = setup_single_task();

        db_handler.create_task(expected.clone());
        db_handler.delete_task(1);
        db_handler.undo();

        let actual = db_handler.read_tasks();

        assert_eq!(vec![expected], actual);
    }

    #[test]
    fn undo_update_should_work() {
        let (db_handler, expected) = setup_single_task();
        db_handler.create_task(expected.clone());

        db_handler.update_task(
            1,
            &Task::new(1234, "An updated task", TaskStatus::Undone, None, None),
        );

        db_handler.undo();

        let actual = db_handler.read_tasks();

        assert_eq!(vec![expected], actual);
    }
}
