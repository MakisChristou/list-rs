use std::rc::Rc;
use std::sync::Arc;

use rusqlite::{types::FromSql, Result};

use cursive::direction::Orientation;
use cursive::event::EventResult;
use cursive::traits::Identifiable;
use cursive::views::{Checkbox, Dialog, EditView, SelectView, TextView};
use cursive::Cursive;

mod db_handler;
mod task;

use crate::db_handler::DatabaseHandler;
use crate::task::Task;
use crate::task::TaskStatus;
use cursive::view::Resizable;
use cursive::view::Scrollable;
use cursive::views::LinearLayout;
use cursive::views::ListView;
use cursive::CursiveExt;

fn main() -> Result<()> {
    let db_handler = Arc::new(DatabaseHandler::new("tasks.db"));

    // let default_task = Task {
    //     id: 0,
    //     title: "Task title".to_string(),
    //     text: "Hello world!".to_string(),
    //     status: TaskStatus::Undone,
    //     tag: None,
    //     due_date: None,
    // };

    // // db_handler.create_task(default_task);

    let tasks = db_handler.read_tasks();

    let mut siv = Cursive::default();

    // Global key callbacks
    siv.add_global_callback('q', |s| {
        s.quit();
    });

    let local_db_handler = Arc::clone(&db_handler);

    siv.add_global_callback('n', move |s| {
        let db_handler = Arc::clone(&local_db_handler);

        s.add_layer(
            Dialog::new()
                .title("New Task")
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    LinearLayout::new(Orientation::Vertical)
                        .child(TextView::new("Title:"))
                        .child(EditView::new().with_name("edit_title"))
                        .child(TextView::new("Text:"))
                        .child(EditView::new().with_name("edit_text")),
                )
                .button("Create", move |s| {
                    let title = s
                        .call_on_name("edit_title", |view: &mut EditView| view.get_content())
                        .unwrap();

                    let text = s
                        .call_on_name("edit_text", |view: &mut EditView| view.get_content())
                        .unwrap();

                    let new_task = Task {
                        id: 0, // You need to implement this
                        title: title.to_string(),
                        text: text.to_string(),
                        status: TaskStatus::Undone,
                        tag: None,
                        due_date: None,
                    };

                    db_handler.create_task(new_task);

                    s.pop_layer();
                })
                .button("Cancel", |s| {
                    s.pop_layer();
                }),
        );
    });

    let local_db_handler = Arc::clone(&db_handler);

    if tasks.is_empty() {
        siv.add_layer(
            Dialog::around(TextView::new(
                "Empty task list, press 'n' to create a new task.",
            ))
            .title("Task List"),
        );
    } else {
        let mut select_view = SelectView::new().on_submit(move |siv: &mut Cursive, item: &Task| {
            let item = item.clone();
            let task_details = format!(
                "ID: {}\nText: {}\nStatus: {}\nTag: {}\nDue Date: {}",
                item.id,
                item.text,
                item.status,
                item.tag.clone().unwrap_or_else(|| String::from("No Tag")),
                item.due_date
                    .clone()
                    .unwrap_or_else(|| String::from("No Due Date"))
            );
            siv.add_layer(
                Dialog::text(task_details)
                    .title("Task Details")
                    .button("Edit", move |s| {
                        s.pop_layer(); // Remove the current dialog
                        s.add_layer(
                            Dialog::new()
                                .title("Edit Task")
                                .padding_lrtb(1, 1, 1, 0)
                                .content(
                                    LinearLayout::new(Orientation::Vertical)
                                        .child(
                                            if item.status == TaskStatus::Done {
                                                Checkbox::new().checked()
                                            } else {
                                                Checkbox::new()
                                            }
                                            .on_change(|s, is_checked| {
                                                // Here you could update the status of the task in the database
                                            }),
                                        )
                                        .child(
                                            EditView::new()
                                                .content(&item.text)
                                                .on_edit(|s, text, _| {
                                                    // Here you could update the text of the task in the database
                                                })
                                                .with_id("edit_task"),
                                        ),
                                )
                                .button("Save", |s| {
                                    // Here you could save the updated task to the database
                                    s.pop_layer();
                                })
                                .button("Cancel", |s| {
                                    s.pop_layer();
                                }),
                        );
                    })
                    .button("Back", |s| {
                        s.pop_layer();
                    }),
            );
        });

        for task in tasks {
            select_view.add_item(
                format!(
                    "[{}] {}",
                    if task.status == TaskStatus::Done {
                        "x"
                    } else {
                        " "
                    },
                    task.title
                ),
                task,
            );
        }

        siv.add_layer(
            Dialog::around(select_view.scrollable())
                .title("Task List")
                .button("Quit", |s| s.quit()),
        );
    }
    siv.run();

    Ok(())
}
