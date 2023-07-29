use crate::FromSql;
use chrono::NaiveDateTime;
use colored::*;
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
    pub text: String,
    pub status: TaskStatus,
    pub tag: Option<String>,
    pub due_date: Option<String>,
    pub created_at: NaiveDateTime,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = format!("{}", self.id).bold();
        let text = match self.status {
            TaskStatus::Done => self.text.strikethrough().to_string(),
            _ => self.text.clone(),
        };
        write!(f, "{} {}", id, text)
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 1,
            text: Default::default(),
            status: TaskStatus::Undone,
            tag: Default::default(),
            due_date: Default::default(),
            created_at: Default::default(),
        }
    }
}

impl Task {
    pub fn new(
        id: i32,
        text: &str,
        status: TaskStatus,
        tag: Option<String>,
        due_date: Option<String>,
    ) -> Self {
        Task {
            id,
            text: text.to_string(),
            status,
            tag,
            due_date,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
