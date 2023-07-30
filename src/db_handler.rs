use crate::task::Task;
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
            "CREATE TABLE IF NOT EXISTS History (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                command        TEXT NOT NULL,
                created_at  TEXT
            )",
            (), // empty list of parameters.
        )
        .unwrap();
    }

    fn push_update_to_undo_history(&self, id: i32, current_task: Task) -> rusqlite::Result<()> {
        let undo_query = format!(
            "UPDATE Tasks SET text = '{}', status = '{}', tag = {}, due_date = {}, created_at = '{}' WHERE id = {}",
            current_task.text,
            current_task.status.to_string(),
            match current_task.tag {
                Some(t) => format!("'{}'", t),
                None => "NULL".to_string(),
            },
            match current_task.due_date {
                Some(d) => format!("'{}'", d),
                None => "NULL".to_string(),
            },
            current_task.created_at,
            id
        );

        self.conn.execute(
            "INSERT INTO History (command, created_at) VALUES (?1, ?2)",
            [&undo_query, &chrono::Local::now().to_string()],
        )?;

        Ok(())
    }

    fn push_create_to_undo_history(&self) -> rusqlite::Result<()> {
        // Add the opposite operation to the History
        let undo_query = "DELETE FROM Tasks WHERE id = (SELECT MAX(id) FROM Tasks)";
        self.conn.execute(
            "INSERT INTO History (command, created_at) VALUES (?1, ?2)",
            (&undo_query, chrono::Local::now().to_string()),
        )?;

        Ok(())
    }

    fn push_delete_to_undo_history(&self, task: Task) -> rusqlite::Result<()> {
        // Add the opposite operation to the History
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
            "INSERT INTO History (command, created_at) VALUES (?1, ?2)",
            [&undo_query, &chrono::Local::now().to_string()],
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

    pub fn create_task(&self, task: Task) -> rusqlite::Result<usize> {
        // Execute create query
        let id = self.conn.execute(
            "INSERT INTO Tasks (text, status, tag, due_date, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &task.text,
                &task.status.to_string(),
                &task.tag,
                &task.due_date,
                &task.created_at,
            ),
        )?;

        self.push_create_to_undo_history()?;

        Ok(id)
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
        let current_task = self.read_task(id).unwrap();

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

        self.push_update_to_undo_history(id, current_task.clone())?;

        Ok(())
    }

    pub fn delete_task(&self, id: i32) -> rusqlite::Result<()> {
        // Execute delete query
        let task = self.read_task(id).unwrap();
        self.conn.execute("DELETE FROM Tasks WHERE id = ?1", [id])?;

        self.push_delete_to_undo_history(task.clone())?;

        Ok(())
    }

    pub fn undo(&self) -> rusqlite::Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, command FROM History ORDER BY id DESC LIMIT 1")
            .unwrap();

        let mut undo_iter = stmt.query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;

        if let Some(Ok((id, undo_command))) = undo_iter.next() {
            match self.conn.execute(&undo_command, []) {
                Ok(_) => {
                    // If the undo operation is successful, delete the command from the history
                    self.conn
                        .execute("DELETE FROM History WHERE id = ?1", params![id])?;
                }
                Err(err) => {
                    // Handle the error that occurred during the undo operation
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
