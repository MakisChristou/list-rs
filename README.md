# todo-rs
A simple clli todo app written in Rust.


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
  remove    Removes a task
  update    Updates a task with a given id
  list      Lists a single task or all
  all       List all tasks
  archived  List archived tasks
  archive   Set a task to Archived
  done      Sets a task to Done
  undone    Sets a task to Undone
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