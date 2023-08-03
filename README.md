# list-rs
A simple clli todo app written in Rust.

```bash
$ list-rs    

Welcome to list-rs, a cli todo app written in Rust 🦀!
Task list is empty. 
Run list-rs add to add a new task. 
Run list-rs --help to get all commands
```

## Example Usage 

### Add task
```bash
$ list-rs add "Watch Oppenheimer"
Task Added
```

### Remove a task by id

```bash
$ list-rs remove 3
Task 3 removed
```

### Update a task by id

```bash
$ list-rs update 3 "A new title"
Task 3 updated
```

### Set a task to Done
```bash
$ list-rs done 4
Task 4 set to Done
```

### Set a task to Undone
```bash
$ list-rs undone 4
Task 4 set to Undone
```
### Set a task to Archived
```bash
$ list-rs archive 4
Task 4 set to Archived
```

### List all pending (undone) tasks
```bash
$ list-rs                    

4) ⌛ Take vitamins
3) ⌛ Meditate
2) ⌛ Go to the gym
1) ⌛ Watch Oppenheimer
```

### List all tasks

```bash
$ list-rs all      

4) 📦 Take vitamins
3) ⌛ Meditate
2) ✅ Go to the gym
1) ⌛ Watch Oppenheimer
```

### Show archived tasks

```bash
$ list-rs archived

4) 📦 Take vitamins
```

### Search a task by its contents

```bash
$ list-rs search "Hello"   

6) ⌛ Hello World
5) ⌛ Hello
```

## Functional Requirements
- CRUD Operations: Add, Delete, Update, and List all tasks.
- Automatic sorting by date created
- Option to mark tasks as "Done," "Undone," and "Archived."
- Task list persistent on disk using a database
- Undo/Redo operations with infinite history
- Search a task based on its content
- Configurable database path

## Future Work
- Implement task due dates
- Implement task tags
- Interactive mode

## Building 
```bash
cargo b --release
```

## Run tests
```bash
cargo t --release
```

## Cli Arguments
```bash
$ list-rs --help
Usage: list-rs [COMMAND]

Commands:
  add       Adds a task
  remove    Removes a task with a given id
  update    Updates a task with a given id
  list      Lists all pending tasks
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


## Setting a custom database path
Create a `.env` file and add the following line

```bash
DB_PATH=/your/custom/path/tasks.db
```

## Crates Used
- rusqlite (database)
- clap (argument parsing)
- colored (terminal pretty printing)
- chrono (datetime stuff)
- dotenv (for configuration purposes)