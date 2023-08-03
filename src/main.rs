use colored::Colorize;
use dotenv::dotenv;
use rusqlite::{types::FromSql, Result};
use std::env;

mod args;
mod db_handler;
mod task;

use crate::args::{Cli, Commands};
use crate::db_handler::DatabaseHandler;
use crate::task::Task;
use crate::task::TaskStatus;

fn print_tasks<F: Fn(&Task) -> bool>(tasks: &Vec<Task>, filter: F, should_show_archived: bool) {
    println!();
    let undone_tasks: Vec<_> = tasks
        .iter()
        .filter(|x| x.status == TaskStatus::Undone)
        .collect();
    if tasks.is_empty() {
        println!(
            "Welcome to list-rs, a cli todo app written in Rust 🦀!\nTask list is empty. \nRun {} to add a new task. \nRun {} to get all commands",
            "list-rs add".bold().color("Blue"),
            "list-rs --help".bold().color("Blue")
        );
    } else if undone_tasks.is_empty() && !should_show_archived {
        println!("Great, no pending tasks 🎉");
    } else {
        for task in tasks.iter().filter(|&x| filter(x)) {
            println!("{}", task);
        }
    }
    println!();
}

fn main() -> Result<()> {
    dotenv().ok();
    let database_path = match env::var("DB_PATH") {
        Ok(value) => value,
        Err(_) => String::from("tasks.db"),
    };

    let db_handler = DatabaseHandler::new(&database_path);

    let mut tasks = db_handler.read_tasks();
    tasks.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let cli = Cli::parse_arguments();

    match &cli.command {
        Some(Commands::Add { text }) => {
            match db_handler.create_task(Task::new(1, text, TaskStatus::Undone, None, None)) {
                Ok(_) => {
                    println!("Task Added");
                }
                Err(e) => {
                    println!("Error creating task {}", e)
                }
            }
        }
        Some(Commands::List {}) => {
            print_tasks(&tasks, |task| task.status != TaskStatus::Archived, false);
        }
        Some(Commands::All {}) => {
            print_tasks(&tasks, |_| true, true);
        }
        Some(Commands::Archived {}) => {
            print_tasks(&tasks, |task| task.status == TaskStatus::Archived, true);
        }
        Some(Commands::Remove { id }) => match db_handler.delete_task(*id) {
            Ok(_) => {
                println!("Task {} removed", id);
            }
            Err(e) => {
                println!("Error removing task {}", e)
            }
        },
        Some(Commands::Search { content }) => {
            print_tasks(
                &tasks,
                |task| task.text.to_lowercase().contains(&content.to_lowercase()),
                true,
            );
        }
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
        Some(Commands::Undo {}) => match db_handler.undo() {
            Ok(_) => {}
            Err(e) => {
                println!("Error undoing task {}", e)
            }
        },
        Some(Commands::Redo {}) => match db_handler.redo() {
            Ok(_) => {}
            Err(e) => {
                println!("Error redoing task {}", e)
            }
        },
        None => {
            print_tasks(&tasks, |task| task.status != TaskStatus::Archived, false);
        }
    }

    Ok(())
}
