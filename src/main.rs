//! A terminal todo list application.
//! The tasks are printed nicely, in color to the terminal.
//! Tasks can contain sub tasks.
//! You can save the tasks to a file.
mod date;

use chrono::{Local, TimeZone};
use date::DateFormat;

#[derive(Debug)]
struct Task {
    title: String,
    description: String,
    importance: u8,
    due_date: Option<DateFormat>,
    date_created: DateFormat,
    sub_tasks: Vec<Task>,
}

impl Task {
    fn new(
        title: String,
        description: String,
        importance: u8,
        due_date: Option<DateFormat>,
    ) -> Task {
        Task {
            title,
            description,
            importance,
            due_date,
            date_created: Local::now(),
            sub_tasks: Vec::new(),
        }
    }

    fn print_header(&self) {
        match self.importance {
            3 => print!("\x1b[1;37;31m{}\x1b[0m", self.title),
            2 => print!("\x1b[1;37;33m{}\x1b[0m", self.title),
            1 => print!("\x1b[1;37;36m{}\x1b[0m", self.title),
            _ => print!("{}", self.title),
        }

        if let Some(due_date) = self.due_date {
            let time_left = date::get_time_left(due_date);

            println!("\t\x1b[1;37;30m [due in {}]\x1b[0m", time_left);
        }
    }

    fn print_task(&self) {
        self.print_header();

        if self.description != "" {
            println!("{}", self.description);
        }

        if !self.sub_tasks.is_empty() {
            println!("\x1b[1;37;37mSub tasks ({}):\x1b[0m", self.sub_tasks.len());
            for task in &self.sub_tasks {
                task.print_header();
            }
        }
    }

    fn print_info(&self) {
        println!("\x1b[1;37;37mTitle:\x1b[0m {}", self.title);
        println!("\x1b[1;37;37mDescription:\x1b[0m {}", self.description);
        println!("\x1b[1;37;37mImportance:\x1b[0m {}", self.importance);
        println!(
            "\x1b[1;37;37mDate created:\x1b[0m {}",
            self.date_created.format("%Y-%m-%d %H:%M:%S")
        );
        println!(
            "\x1b[1;37;37mDate due:\x1b[0m {}",
            if let Some(due_date) = self.due_date {
                due_date.format("%Y-%m-%d %H:%M:%S").to_string()
            } else {
                "None".to_string()
            }
        );
    }

    fn add_sub_task(&mut self, sub_task: Task) {
        self.sub_tasks.push(sub_task);
    }
}

fn main() {
    let mut task = Task::new(
        "Task 1".to_string(),
        "This is a task".to_string(),
        1,
        Some(Local.ymd(2022, 4, 17).and_hms(0, 0, 0)),
    );
    task.add_sub_task(Task::new(
        "Sub task 1".to_string(),
        "This is a sub task".to_string(),
        2,
        Some(Local.ymd(2022, 4, 17).and_hms(0, 0, 0)),
    ));

    task.print_task();
    println!();
    task.print_info();
    task.sub_tasks[0].print_info();
}
