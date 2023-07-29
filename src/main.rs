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
    /// Adds a todo
    Add {
        text: String,
    },

    /// Removes a todo
    Remove {
        id: i32,
    },

    /// Updates a todo with a given id
    Update {
        id: i32,
        text: String,
    },

    /// Lists a single todo or all
    List {
        id: Option<i32>,
    },

    /// List all tasks
    All {
    },

    /// List archived tasks
    Archived {
    },

    /// Set a task to Archived
    Archive {
        id: i32,
    },

    /// Sets a task to done
    Done {
        id: i32,
    },

    /// Sets a task to done
    Undone {
        id: i32,
    },
}

fn print_empty_message_if_needed(tasks: &Vec<Task>) {
    println!("");
    if tasks.is_empty() {
        println!("Todo list is empty. Run {} to add a new task.", "todo-rs add".bold().color("Blue"));
    }
}

fn print_all_tasks(tasks: &Vec<Task>) {
    print_empty_message_if_needed(tasks);
    for task in tasks {
        println!("{}", task.to_string());
    }
    println!("");
}

fn print_non_archived_tasks(tasks: &Vec<Task>) {
    print_empty_message_if_needed(tasks);
    for task in tasks.iter().filter(|&x| x.status != TaskStatus::Archived) {
        println!("{}", task.to_string());
    }
    println!("");
}

fn print_archived_tasks(tasks: &Vec<Task>) {
    print_empty_message_if_needed(tasks);
    for task in tasks.iter().filter(|&x| x.status == TaskStatus::Archived) {
        println!("{}", task.to_string());
    }
    println!("");
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
            print_non_archived_tasks(&tasks);
        }
        Some(Commands::All {}) => {
            print_all_tasks(&tasks);
        },
        Some(Commands::Archived {  }) => {
            print_archived_tasks(&tasks);
        },
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
            print_non_archived_tasks(&tasks);
        }
    }

    Ok(())
}
