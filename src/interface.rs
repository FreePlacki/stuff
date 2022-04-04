//! Contains functions for interfacing with the user.

use chrono::prelude::*;
use crate::date::DateFormat;

use crate::task::{Task};
use crate::task;
use crate::date;

macro_rules! get_input {
    ($prompt:expr, $default:expr) => {{
        let mut input = String::new();
        print!("{}", $prompt);
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            $default.to_string()
        } else {
            input.trim().to_string()
        }
    }};
}

pub(crate) use get_input;

pub fn add_prompt() -> Task {
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
    let description = if description == "" {
        None
    } else {
        Some(description)
    };

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

pub fn edit_prompt(task: &Task) -> Task {
    let mut edited_task = Task::new("".to_string(), None, 0, None);
    println!(
        "Editing task: {}\n(Press enter to leave unchanged)",
        task.title
    );
    let title = get_input!("Title*: ", task.title);
    if title != "" {
        edited_task.title = title;
    }

    let description = get_input!("Description: ", "");
    let description = if description == "" {
        task.description.clone()
    } else {
        Some(description)
    };
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
    edited_task.due_date = due;
    edited_task
}