use crate::FromSql;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TaskStatus {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub status: TaskStatus,
    pub tag: Option<String>,
    pub due_date: Option<String>,
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

impl Task {
    pub fn new(
        id: i32,
        title: &str,
        text: &str,
        status: TaskStatus,
        tag: Option<String>,
        due_date: Option<String>,
    ) -> Self {
        Task {
            id,
            title: title.to_string(),
            text: text.to_string(),
            status,
            tag,
            due_date,
        }
    }
}
