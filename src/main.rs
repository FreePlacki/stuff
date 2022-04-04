//! A terminal todo list application.
//! The tasks are printed nicely, in color to the terminal.
//! Tasks can contain sub tasks.
//! You can save the tasks to a file.
mod commands;
mod date;
mod task;
mod interface;

use task::TaskList;
use interface::{get_input, add_prompt};


fn run_prompt(task_list: &mut TaskList) {
    'outer: loop {
        let input = get_input!("\n> ", "").to_lowercase();
        let mut input = input.split_whitespace();
        let command = input.next().unwrap_or("");
        let arg = input.next().unwrap_or("");

        if command == "" { continue; }

        let command_list = commands::Commands::new();
        for c in command_list {
            if c.keywords().contains(&command) {
                match c.execute(arg, task_list) {
                    Ok(_) => task_list.save_to_file(),
                    Err(e) => println!("{}", e),
                }
                continue 'outer;
            }
        }

        println!("Unknown command: {}", command);
    }
}

fn main() {
    let mut task_list = TaskList::new();
    task_list.load_from_file();

    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            task_list.print_tasks();
            run_prompt(&mut task_list);
        }
        2 => match args[1].as_str() {
            "show" | "s" => {
                if task_list.tasks.is_empty() {
                    return;
                }

                if args.len() > 2 && args[2] == "all" {
                    task_list.print_tasks();
                    return;
                }

                let max_priority = task_list.sorted_by_importance();
                let tasks = task_list.get_by_importance(max_priority[0].importance);
                let task = TaskList::get_random(&tasks);
                println!("\x1b[1;4;37mHigh priority\x1b[0m:");
                task.print_header();
                println!("\n");

                let least_due = task_list.sorted_by_due();
                let task = &least_due[0];
                println!("\x1b[1;4;37mUrgent\x1b[0m:");
                task.print_header();
                println!("\n");

                let task = task_list.random_task().unwrap();
                println!("\x1b[1;4;37mRandom\x1b[0m:");
                task.print_header();
                println!("\n");
            }
            "add" | "a" => {
                let task = add_prompt();
                task_list.add_task(task);
                task_list.save_to_file();
            }
            "random" | "rand" | "r" => {
                if let Some(task) = task_list.random_task() {
                    println!("Random Task:");
                    task.print_task();
                } else {
                    println!("You have no tasks!");
                }
            }
            _ => {
                println!("Unknown command: {}", args[1]);
            }
        },
        _ => {
            println!("Usage: {} [option]", args[0]);
        }
    }
}
