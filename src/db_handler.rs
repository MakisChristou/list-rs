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

    pub fn new_in_memory() -> Self {
        let conn = Connection::open_in_memory().unwrap();

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

    pub fn delete_task(&self, id: i32) -> rusqlite::Result<usize> {
        self.conn.execute("DELETE FROM Tasks WHERE id = ?1", [id])
    }
}

#[cfg(test)]
mod tests {
    use super::DatabaseHandler;
    use crate::task::{Task, TaskStatus};

    fn setup_single_task() -> (DatabaseHandler, Task) {
        let db_handler = DatabaseHandler::new_in_memory();
        let expected = Task::new(1, "", "", TaskStatus::Undone, None, None);
        (db_handler, expected)
    }

    fn setup_multiple_tasks() -> (DatabaseHandler, Vec<Task>) {
        let db_handler = DatabaseHandler::new_in_memory();

        let tasks = vec![
            Task::new(
                1,
                "Grocery Shopping",
                "Buy fruits, vegetables, and bread.",
                TaskStatus::Undone,
                None,
                None,
            ),
            Task::new(
                2,
                "Car Maintenance",
                "Change oil and check tire pressure.",
                TaskStatus::Undone,
                None,
                None,
            ),
            Task::new(
                3,
                "Reading Assignment",
                "Read chapter 5 of the history book.",
                TaskStatus::Undone,
                None,
                None,
            ),
            Task::new(
                4,
                "Gym Session",
                "30 minutes of cardio and weight lifting.",
                TaskStatus::Undone,
                None,
                None,
            ),
            Task::new(
                5,
                "Cook Dinner",
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

        let _delete_result = db_handler.delete_task(1);
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
}
