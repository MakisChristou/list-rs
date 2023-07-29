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
  add       Adds a todo
  remove    Removes a todo
  update    Updates a todo with a given id
  list      Lists a single todo or all
  all       List all tasks
  archived  List archived tasks
  archive   Set a task to Archived
  done      Sets a task to done
  undone    Sets a task to done
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