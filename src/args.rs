use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds a task
    Add { text: String },

    /// Removes a task
    Remove { id: i32 },

    /// Updates a task with a given id
    Update { id: i32, text: String },

    /// Lists a single task or all
    List { id: Option<i32> },

    /// List all tasks
    All {},

    /// List archived tasks
    Archived {},

    /// Set a task to Archived
    Archive { id: i32 },

    /// Sets a task to Done
    Done { id: i32 },

    /// Sets a task to Undone
    Undone { id: i32 },

    /// Search for a task by its contents
    Search { content: String },
}

impl Cli {
    pub fn parse_arguments() -> Self {
        Cli::parse()
    }
}
