//! The command trait and commands structs.

use crate::interface::{add_prompt, edit_prompt, get_input};
use crate::task::TaskList;

pub trait Command {
    fn keywords(&self) -> &[&str];
    fn execute(&self, arg: &str, task_list: &mut TaskList) -> Result<(), String>;
    fn help(&self) -> &str;
}

struct AddCommand;
impl Command for AddCommand {
    fn keywords(&self) -> &[&str] {
        &["add", "a"]
    }

    fn execute(&self, arg: &str, task_list: &mut TaskList) -> Result<(), String> {
        if arg == "" {
            if task_list.last_shown.is_some() {
                let last_task = &mut task_list.tasks[task_list.last_shown.unwrap() - 1];
                println!("Adding sub-task to: {}", last_task.title);
                last_task.add_sub_task(add_prompt());
                task_list.save_to_file();
                return Ok(());
            }
            let task = add_prompt();
            task_list.add_task(task);
        } else {
            let task_ind: usize = arg.parse().unwrap_or(0);
            if task_ind < 1 || task_ind > task_list.tasks.len() {
                println!(
                    "Task index must be a number between 1 and {}!",
                    task_list.tasks.len()
                );
                return Ok(());
            }
            let task = add_prompt();
            task_list.tasks[task_ind - 1].add_sub_task(task);
        }
        task_list.save_to_file();

        Ok(())
    }

    fn help(&self) -> &str {
        "add [index] - adds a new task. If index is specified, it will add a sub task to the task at the specified index."
    }
}

struct EditCommand;
impl Command for EditCommand {
    fn keywords(&self) -> &[&str] {
        &["edit", "e"]
    }

    fn execute(&self, arg: &str, task_list: &mut TaskList) -> Result<(), String> {
        let task_id: usize = if arg != "" {
            arg.parse().unwrap_or(0)
        } else {
            get_input!("Task ID: ", "").parse().unwrap_or(0)
        };

        if task_id > 0 && task_id <= task_list.tasks.len() {
            let task = edit_prompt(&task_list.tasks[task_id - 1]);
            task_list.tasks[task_id - 1] = task;
            task_list.save_to_file();
        } else if task_id > task_list.tasks.len() {
            println!("Last id is {}!", task_list.tasks.len());
        } else {
            println!("Task ID must be a positive number!");
        }

        Ok(())
    }

    fn help(&self) -> &str {
        "edit [index] - edits the task at the specified index. If no index is specified, it will edit the last shown task."
    }
}

struct ShowCommand;
impl Command for ShowCommand {
    fn keywords(&self) -> &[&str] {
        &["show", "s"]
    }

    fn execute(&self, arg: &str, task_list: &mut TaskList) -> Result<(), String> {
        if task_list.tasks.is_empty() {
            println!("No tasks to show!");
            return Ok(());
        }
        if arg != "" {
            let ind = arg.parse().unwrap_or(0);
            if let Some(last_shown) = task_list.last_shown {
                let task = &task_list.tasks[last_shown - 1];
                if ind > 0 && ind <= task.sub_tasks.len() {
                    task.sub_tasks[ind - 1].print_task();
                } else if ind > task.sub_tasks.len() {
                    println!("Last id is {}!", task.sub_tasks.len());
                } else {
                    println!("Task ID must be a positive number!");
                }
                return Ok(());
            }

            if ind > 0 && ind <= task_list.tasks.len() {
                task_list.tasks[ind - 1].print_task();
                task_list.last_shown = Some(ind);
            } else if ind > task_list.tasks.len() {
                println!("Last id is {}!", task_list.tasks.len());
            } else {
                println!("Task ID must be a positive number!");
            }
        } else {
            task_list.last_shown = None;
            task_list.print_tasks();
        }
        Ok(())
    }

    fn help(&self) -> &str {
        "show [index] - shows the task at the specified index. If no index is specified, it will show all the tasks."
    }
}

struct InfoCommand;
impl Command for InfoCommand {
    fn keywords(&self) -> &[&str] {
        &["info", "i"]
    }

    fn execute(&self, arg: &str, task_list: &mut TaskList) -> Result<(), String> {
        let ind = arg.parse().unwrap_or(0);
        if ind > 0 && ind <= task_list.tasks.len() {
            task_list.tasks[ind - 1].print_info();
            task_list.last_shown = Some(ind);
        } else if ind > task_list.tasks.len() {
            println!("Last id is {}!", task_list.tasks.len());
        } else {
            println!("Task ID must be a positive number!");
        }
        Ok(())
    }

    fn help(&self) -> &str {
        "info [index] - shows all stored information about the task at the specified index."
    }
}

struct RemoveCommand;
impl Command for RemoveCommand {
    fn keywords(&self) -> &[&str] {
        &["remove", "r"]
    }

    fn execute(&self, arg: &str, task_list: &mut TaskList) -> Result<(), String> {
        let ind = arg.parse().unwrap_or(0);
        if ind > 0 && ind <= task_list.tasks.len() {
            task_list.tasks.remove(ind - 1);
            task_list.save_to_file();
        } else if ind > task_list.tasks.len() {
            println!("Last id is {}!", task_list.tasks.len());
        } else {
            println!("Task ID must be a positive number!");
        }
        Ok(())
    }

    fn help(&self) -> &str {
        "remove [index] - removes the task at the specified index."
    }
}

struct SortCommand;
impl Command for SortCommand {
    fn keywords(&self) -> &[&str] {
        &["sort", "s"]
    }

    fn execute(&self, arg: &str, task_list: &mut TaskList) -> Result<(), String> {
        if arg == "" {
            task_list.sort_by_date_created();
            println!("Sorted by date created.");
            task_list.print_tasks();
        } else {
            match arg {
                "due" | "d" => {
                    task_list.sort_by_due();
                    println!("Sorted by due date.");
                }
                "importance" | "i" => {
                    task_list.sort_by_importance();
                    println!("Sorted by importance.");
                }
                "created" | "c" => {
                    task_list.sort_by_date_created();
                    println!("Sorted by date created.");
                }
                _ => {
                    println!("Invalid sort type!");
                    return Ok(());
                }
            }
            task_list.print_tasks();
            task_list.save_to_file();
        }
        Ok(())
    }

    fn help(&self) -> &str {
        "sort [type] - sorts all tasks by the specified criteria. Type can be: created, due, importance. If no type is specified, it will sort by created."
    }
}

struct QuitCommand;
impl Command for QuitCommand {
    fn keywords(&self) -> &[&str] {
        &["quit", "exit", "q"]
    }

    fn execute(&self, _arg: &str, _task_list: &mut TaskList) -> Result<(), String> {
        println!("Good luck with your tasks ;)");
        std::process::exit(0);
    }

    fn help(&self) -> &str {
        "Exits the program."
    }
}

pub struct Commands {
    pub commands: Vec<Box<dyn Command>>,
}

impl Commands {
    pub fn new() -> Self {
        let commands: Vec<Box<dyn Command>> = vec![
            Box::new(QuitCommand),
            Box::new(AddCommand),
            Box::new(EditCommand),
            Box::new(ShowCommand),
            Box::new(InfoCommand),
            Box::new(RemoveCommand),
        ];

        Commands { commands }
    }
}

impl Iterator for Commands {
    type Item = Box<dyn Command>;

    fn next(&mut self) -> Option<Self::Item> {
        self.commands.pop()
    }
}
