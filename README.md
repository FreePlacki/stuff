# stuff
A simple command line app for managing todos.

## Running

```bash
cargo build --release

./target/release/stuff
```

## Usage

```bash
stuff [COMMAND] [SUBCOMMAND]
```

Running `stuff` without any arguments will put you in an interactive mode.

## Commands
- `show` - Shows one important todo, one urgent and a random one. You can also run `show all` to display all todos.

- `add` - Opens interactive mode to add a new todo.

- `random` - Displays a random todo.

### Interactive mode commands
- `add [index]` - opens interactive mode to add a new todo. You can specify an index of a todo to add a sub-task to it.

- `edit [index]` - opens interactive mode to edit a todo with specified index.

- `show [index]` - shows all todos or only one if index is specified.

- `info [index]` - shows all information stored in a todo.

- `remove [index]` - removes a todo and all its sub-tasks.

- `sort [options]` - sorts todos by specified options. Options are:
    - `created` (default) - sorts by creation date.
    - `due` - sorts by due date.
    - `important` - sorts by importance.

- `quit` - quits interactive mode.
