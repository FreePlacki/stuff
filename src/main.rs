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
        let inp = get_input!("Due date: ", "").to_string()
            + " "
            + Local::now().offset().to_string().as_str();
        // allow empty due
        if inp == "" {
            due = None;
            break;
        }

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
    let task = Task::new(title, description, importance, due);
    task
}

fn run_prompt(task_list: &mut TaskList) {
    loop {
        let input = get_input!("\n> ", "").to_lowercase();

        match input.as_str() {
            "add" => {
                let task = add_prompt();
                task_list.add_task(task);
                task_list.save_to_file(task::SAVE_FILE_PATH);
            }
            "exit" | "quit" | "q" | "e" => {
                break;
            }
            _ => {
                println!("Unknown command: {}", input);
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
