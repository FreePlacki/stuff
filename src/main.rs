//! A terminal todo list application.
//! The tasks are printed nicely, in color to the terminal.
//! Tasks can contain sub tasks.
//! You can save the tasks to a file.
mod date;
mod task;

use std::io::Write;

use chrono::prelude::*;
use task::{Task, TaskList};

use crate::date::DateFormat;

macro_rules! get_input {
    ($prompt:expr, $default:expr) => {{
        let mut input = String::new();
        print!("{}", $prompt);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            $default.to_string()
        } else {
            input.trim().to_string()
        }
    }};
}

fn add_prompt() -> Task {
    let mut title;
    loop {
        title = get_input!("Title*: ", "");
        if title != "" {
            break;
        } else {
            println!("Title cannot be empty!");
        }
    }

    let description = get_input!("Description: ", "");

    let mut importance: u8;
    loop {
        importance = get_input!("Importance: ", "0").parse().unwrap_or(69);
        if importance > task::IMPORTANCE_MAX {
            println!(
                "Importance must be a number between 0 and {}!",
                task::IMPORTANCE_MAX
            );
        } else {
            break;
        }
    }

    let mut due: Option<DateFormat>;
    loop {
        let inp = get_input!("Due date: ", "").to_string();
        // allow empty due
        if inp == "" {
            due = None;
            break;
        }
        let parsed_date = date::date_from_time(inp.as_str());
        if let Ok(date) = parsed_date {
            due = Some(date);
            break;
        }

        let inp = inp + ":00 " + Local::now().offset().to_string().as_str();

        let mut invalid_date = false;
        due = Some(inp.parse().unwrap_or_else(|_| {
            println!("Invalid date! (use format: YYYY-MM-DD HH:MM or Xw Xd Xh Xm)");
            invalid_date = true;
            Local::now()
        }));
        if !invalid_date {
            break;
        }
    }
    let task = Task::new(title, description, importance, due);
    task
}

fn edit_prompt(task: &Task) -> Task {
    let mut edited_task = Task::new("".to_string(), "".to_string(), 0, None);
    println!(
        "Editing task: {}\n(Press enter to leave unchanged)",
        task.title
    );
    let title = get_input!("Title*: ", task.title);
    if title != "" {
        edited_task.title = title;
    }

    let description = get_input!("Description: ", task.description);
    edited_task.description = description;

    let mut importance: u8;
    loop {
        importance = get_input!("Importance: ", task.importance.to_string())
            .parse()
            .unwrap_or(69);
        if importance > task::IMPORTANCE_MAX {
            println!(
                "Importance must be a number between 0 and {}!",
                task::IMPORTANCE_MAX
            );
        } else {
            edited_task.importance = importance;
            break;
        }
    }

    let mut due: Option<DateFormat>;
    loop {
        let inp = get_input!("Due date: ", "").to_string();
        // allow empty due
        if inp == "" {
            due = task.due_date;
            break;
        }
        let inp = inp + " " + Local::now().offset().to_string().as_str();

        // TODO support other formats
        let mut invalid_date = false;
        due = Some(inp.parse().unwrap_or_else(|_| {
            println!("Invalid date! (use format: YYYY-MM-DD HH:MM:SS)");
            invalid_date = true;
            Local::now()
        }));
        if !invalid_date {
            break;
        }
    }
    edited_task.due_date = due;
    edited_task
}

fn run_prompt(task_list: &mut TaskList) {
    loop {
        let input = get_input!("\n> ", "").to_lowercase();
        let mut input = input.split_whitespace();
        let command = input.next().unwrap_or("");
        let arg = input.next().unwrap_or("");

        match command {
            "add" | "a" => {
                if arg == "" {
                    let task = add_prompt();
                    task_list.add_task(task);
                } else {
                    let task_ind: usize = arg.parse().unwrap_or(0);
                    if task_ind < 1 || task_ind > task_list.tasks.len() {
                        println!("Task index must be a number between 1 and {}!", task_list.tasks.len());
                        continue;
                    }
                    let task = add_prompt();
                    task_list.tasks[task_ind - 1].add_sub_task(task);
                }
                task_list.save_to_file(task::SAVE_FILE_PATH);
            }
            "edit" | "e" => {
                let task_id: usize = if arg != "" {
                    arg.parse().unwrap_or(0)
                } else {
                    get_input!("Task ID: ", "").parse().unwrap_or(0)
                };

                if task_id > 0 && task_id <= task_list.tasks.len() {
                    let task = edit_prompt(&task_list.tasks[task_id - 1]);
                    task_list.tasks[task_id - 1] = task;
                    task_list.save_to_file(task::SAVE_FILE_PATH);
                } else if task_id > task_list.tasks.len() {
                    println!("Last id is {}!", task_list.tasks.len());
                } else {
                    println!("Task ID must be a positive number!");
                }
            }
            "show" | "s" => {
                if arg != "" {
                    let ind = arg.parse().unwrap_or(0);
                    if let Some(task) = &task_list.last_shown {
                        if ind > 0 && ind <= task.sub_tasks.len() {
                            task.sub_tasks[ind - 1].print_task();
                        } else if ind > task.sub_tasks.len() {
                            println!("Last id is {}!", task.sub_tasks.len());
                        } else {
                            println!("Task ID must be a positive number!");
                        }
                        continue;
                    }

                    if ind > 0 && ind <= task_list.tasks.len() {
                        task_list.tasks[ind - 1].print_task();
                        task_list.last_shown = Some(task_list.tasks[ind - 1].clone());
                    } else if ind > task_list.tasks.len() {
                        println!("Last id is {}!", task_list.tasks.len());
                    } else {
                        println!("Task ID must be a positive number!");
                    }
                } else {
                    task_list.last_shown = None;
                    task_list.print_tasks();
                }
            }
            "info" | "i" => {
                let ind = arg.parse().unwrap_or(0);
                if ind > 0 && ind <= task_list.tasks.len() {
                    task_list.tasks[ind - 1].print_info();
                    task_list.last_shown = Some(task_list.tasks[ind - 1].clone());
                } else if ind > task_list.tasks.len() {
                    println!("Last id is {}!", task_list.tasks.len());
                } else {
                    println!("Task ID must be a positive number!");
                }
            }
            "remove" | "r" => {
                let ind = arg.parse().unwrap_or(0);
                if ind > 0 && ind <= task_list.tasks.len() {
                    task_list.tasks.remove(ind - 1);
                    task_list.save_to_file(task::SAVE_FILE_PATH);
                    // task_list.tasks.clear();
                    // task_list.load_from_file(task::SAVE_FILE_PATH);
                } else if ind > task_list.tasks.len() {
                    println!("Last id is {}!", task_list.tasks.len());
                } else {
                    println!("Task ID must be a positive number!");
                }
            }
            "exit" | "quit" | "q" => {
                break;
            }
            "" => {
                continue;
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}

fn main() {
    let mut task_list = TaskList::new();
    task_list.load_from_file(task::SAVE_FILE_PATH);

    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            task_list.print_tasks();
            run_prompt(&mut task_list);
        }
        2 => {
            if args[1] == "show" {
                task_list.print_tasks();
            } else if args[1] == "add" {
                let task = add_prompt();
                task_list.add_task(task);
                task_list.save_to_file(task::SAVE_FILE_PATH);
            } else {
                println!("Unknown command: {}", args[1]);
            }
        }
        _ => {
            println!("Usage: {} [option]", args[0]);
        }
    }
}
