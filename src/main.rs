use colored::Colorize;
use rusqlite::{types::FromSql, Result};

mod args;
mod db_handler;
mod task;

use crate::db_handler::DatabaseHandler;
use crate::task::Task;
use crate::task::TaskStatus;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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
}

fn print_tasks<F: Fn(&Task) -> bool>(tasks: &Vec<Task>, filter: F) {
    println!("");
    if tasks.is_empty() {
        println!(
            "task list is empty. Run {} to add a new task.",
            "task-rs add".bold().color("Blue")
        );
    } else {
        for task in tasks.iter().filter(|&x| filter(x)) {
            println!("{}", task.to_string());
        }
        println!("");
    }
}

fn main() -> Result<()> {
    let db_handler = DatabaseHandler::new("tasks.db");

    let tasks = db_handler.read_tasks();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { text }) => {
            match db_handler.create_task(Task::new(1, &text, TaskStatus::Undone, None, None)) {
                Ok(_) => {
                    println!("Task Added")
                }
                Err(e) => {
                    println!("Error creating task {}", e)
                }
            }
        }
        Some(Commands::List { id }) => {
            print_tasks(&tasks, |task| task.status != TaskStatus::Archived);
        }
        Some(Commands::All {}) => {
            print_tasks(&tasks, |_| true);
        }
        Some(Commands::Archived {}) => {
            print_tasks(&tasks, |task| task.status == TaskStatus::Archived);
        }
        Some(Commands::Remove { id }) => match db_handler.delete_task(*id) {
            Ok(_) => {
                println!("Task {} removed", id);
            }
            Err(e) => {
                println!("Error removing task {}", e)
            }
        },
        Some(Commands::Update { id, text }) => {
            let task = db_handler.read_task(*id);

            match task {
                Some(mut task) => {
                    task.text = (*text).clone();
                    match db_handler.update_task(*id, &task) {
                        Ok(_) => {
                            println!("Task {} updated", id)
                        }
                        Err(e) => {
                            println!("Error updating task {}", e)
                        }
                    }
                }
                None => {
                    println!("Task with id {} does not exist", *id);
                }
            }
        }
        Some(Commands::Done { id }) => {
            let task = db_handler.read_task(*id);

            match task {
                Some(mut task) => {
                    task.status = TaskStatus::Done;
                    match db_handler.update_task(*id, &task) {
                        Ok(_) => {
                            println!("Task {} set to Done", id)
                        }
                        Err(e) => {
                            println!("Error modifying task {}", e)
                        }
                    }
                }
                None => {
                    println!("Task with id {} does not exist", *id);
                }
            }
        }
        Some(Commands::Archive { id }) => {
            let task = db_handler.read_task(*id);

            match task {
                Some(mut task) => {
                    task.status = TaskStatus::Archived;
                    match db_handler.update_task(*id, &task) {
                        Ok(_) => {
                            println!("Task {} set to Archived", id)
                        }
                        Err(e) => {
                            println!("Error modifying task {}", e)
                        }
                    }
                }
                None => {
                    println!("Task with id {} does not exist", *id);
                }
            }
        }
        Some(Commands::Undone { id }) => {
            let task = db_handler.read_task(*id);

            match task {
                Some(mut task) => {
                    task.status = TaskStatus::Undone;
                    match db_handler.update_task(*id, &task) {
                        Ok(_) => {
                            println!("Task {} set to Undone", id)
                        }
                        Err(e) => {
                            println!("Error modifying task {}", e)
                        }
                    }
                }
                None => {
                    println!("Task with id {} does not exist", *id);
                }
            }
        }
        None => {
            print_tasks(&tasks, |task| task.status != TaskStatus::Archived);
        }
    }

    Ok(())
}
