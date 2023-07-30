# todo-rs
A simple clli todo app written in Rust.

## Features
- CRUD Operations: Add, Delete, Update, and List all tasks.
- Option to mark tasks as "Done," "Undone," and "Archived."
- Task list persistent on disk using a database
- Undo/Redo operations
- Search a task based on its content

## Future Features
- ~~Redo operations~~
- Search a task based on date created as well
- Clear all tasks, archived tasks, done tasks
- Pretty printing 

## Building 
```bash
cargo build --release
```

## Cli Arguments
```bash
$ todo-rs --help
Usage: todo-rs [COMMAND]

Commands:
  add       Adds a task
  remove    Removes a task with a given id
  update    Updates a task with a given id
  list      Lists a single task or all
  all       List all tasks
  archived  List archived tasks
  archive   Sets a task with a given id to Archived
  done      Sets a task with a given id to Done
  undone    Sets a task with a given id to Undone
  search    Search for a task by its contents
  undo      Revert last change
  redo      Redo last change
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Crates Used
- rusqlite (database)
- clap (argument parsing)
- colored (terminal pretty printing)
- chrono (datetime stuff)