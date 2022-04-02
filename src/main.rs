//! A terminal todo list application.
//! The tasks are printed nicely, in color to the terminal.
//! Tasks can contain sub tasks.
//! You can save the tasks to a file.
mod date;

use chrono::{Local, TimeZone};
use date::DateFormat;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct TaskJson {
    title: String,
    description: String,
    importance: u8,
    due_date: String,
    date_created: String,
    sub_tasks: Vec<TaskJson>,
}

impl TaskJson {
    fn new(task: &Task) -> Self {
        let due_date = if let Some(due_date) = task.due_date {
            due_date.format("%Y-%m-%d %H:%M:%S").to_string()
        } else {
            "None".to_string()
        };
        let date_created = task.date_created.format("%Y-%m-%d %H:%M:%S").to_string();

        let mut sub_tasks = Vec::new();
        for t in task.sub_tasks.iter() {
            sub_tasks.push(TaskJson::new(&t.clone()));
        }

        TaskJson {
            title: task.title.clone(),
            description: task.description.clone(),
            importance: task.importance,
            due_date,
            date_created,
            sub_tasks,
        }
    }
}

#[derive(Debug, Clone)]
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

struct TaskList {
    tasks: Vec<Task>,
    json_tasks: Vec<TaskJson>,
}

impl TaskList {
    fn new() -> TaskList {
        TaskList {
            tasks: Vec::new(),
            json_tasks: Vec::new(),
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    fn save_to_file(&mut self, file_path: &str) {
        let mut file = match std::fs::File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                println!("Could not create file: {}", e);
                return;
            }
        };

        for t in self.tasks.iter() {
            self.json_tasks.push(TaskJson::new(&t));
        }
        let json = serde_json::to_string(&self.json_tasks).unwrap();

        match file.write_all(json.as_bytes()) {
            Ok(_) => println!("Saved to file: {}", file_path),
            Err(e) => println!("Could not write to file: {}", e),
        }
    }
}

fn main() {
    let mut task_list = TaskList::new();

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
    let task2 = Task::new(
        "Task 2".to_string(),
        "This is a task".to_string(),
        1,
        Some(Local.ymd(2022, 4, 17).and_hms(0, 0, 0)),
    );
    task_list.add_task(task);
    task_list.add_task(task2);

    task_list.tasks[0].print_task();
    task_list.save_to_file("test.json");
}
