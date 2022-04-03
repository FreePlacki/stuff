//! A module for storing and displaying task data.

use crate::date::DateFormat;
use chrono::{Local, Duration};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

pub const IMPORTANCE_MAX: u8 = 3;
pub const SAVE_FILE_PATH: &str = "saved_stuff.json";

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
        let description = if let Some(desc) = task.description.clone() {
            desc.to_string()
        } else {
            "None".to_string()
        };

        let due_date = if let Some(due_date) = task.due_date {
            due_date.to_string()
        } else {
            "None".to_string()
        };
        let date_created = task.date_created.to_string();

        let mut sub_tasks = Vec::new();
        for t in task.sub_tasks.iter() {
            sub_tasks.push(TaskJson::new(&t));
        }

        TaskJson {
            title: task.title.clone(),
            description: description,
            importance: task.importance,
            due_date,
            date_created,
            sub_tasks,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub description: Option<String>,
    pub importance: u8,
    pub due_date: Option<DateFormat>,
    pub date_created: DateFormat,
    pub sub_tasks: Vec<Task>,
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
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

    pub fn print_header(&self) {
        match self.importance {
            3 => print!("\x1b[1;37;31m{}\x1b[0m", self.title),
            2 => print!("\x1b[1;37;33m{}\x1b[0m", self.title),
            1 => print!("\x1b[1;37;36m{}\x1b[0m", self.title),
            _ => print!("{}", self.title),
        }

        if let Some(due_date) = self.due_date {
            let time_left = crate::date::get_time_left(due_date);

            print!("\t");
            if due_date - Local::now() < chrono::Duration::days(1) {
                print!("\x1b[1;37;31m");
            } else if due_date - Local::now() < chrono::Duration::weeks(1) {
                print!("\x1b[1;37;36m");
            } else {
                print!("\x1b[1;37;37m");
            }
            print!("[due in {}]\x1b[0m", time_left);
        }
    }

    pub fn print_task(&self) {
        self.print_header();
        println!();

        if let Some(desc) = &self.description {
            println!("{}", desc);
        }

        if !self.sub_tasks.is_empty() {
            println!("\x1b[1;37;37mSub tasks ({}):\x1b[0m", self.sub_tasks.len());
            for task in &self.sub_tasks {
                task.print_header();
            }
        }
    }

    pub fn print_info(&self) {
        println!("\x1b[1;37;37mTitle:\x1b[0m {}", self.title);
        let desc = if let Some(desc) = self.description.clone() {
            desc
        } else {
            String::from("None")
        };
        println!("\x1b[1;37;37mDescription:\x1b[0m {}", desc);
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
        println!("\x1b[1;37;37mSub tasks:\x1b[0m {}", self.sub_tasks.len());
    }

    pub fn add_sub_task(&mut self, sub_task: Task) {
        self.sub_tasks.push(sub_task);
    }
}

pub struct TaskList {
    pub tasks: Vec<Task>,
    json_tasks: Vec<TaskJson>,
    pub last_shown: Option<Task>,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::new(),
            json_tasks: Vec::new(),
            last_shown: None,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn random_task(&self) -> Option<Task> {
        if self.tasks.is_empty() {
            return None;
        }

        Some(Self::get_random(&self.tasks))
    }

    pub fn get_random(task_list: &Vec<Task>) -> Task {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..task_list.len());
        task_list[index].clone()
    }

    pub fn sorted_by_priority(&self) -> Vec<Task> {
        let mut tasks = self.tasks.clone();
        tasks.sort_by(|a, b| b.importance.cmp(&a.importance));
        tasks
    }

    pub fn sorted_by_due(&self) -> Vec<Task> {
        let mut tasks = self.tasks.clone();
        // we add 99999 weeks so that the tasks that don't have a due date are at the end
        tasks.sort_by(
            |a, b|
            a.due_date.unwrap_or(Local::now() + Duration::weeks(99999)).cmp(
                &b.due_date.unwrap_or(Local::now() + Duration::weeks(99999))
            )
        );
        tasks
    }

    pub fn get_by_priority(&self, priority: u8) -> Vec<Task> {
        let mut tasks = Vec::new();
        for task in self.tasks.iter() {
            if task.importance == priority {
                tasks.push(task.clone());
            }
        }
        tasks
    }

    pub fn print_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            print!("{}: ", i + 1);
            task.print_header();
            println!();
        }
    }

    pub fn save_to_file(&mut self, file_path: &str) {
        let mut file = match std::fs::File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                println!("Could not create file: {}", e);
                return;
            }
        };

        self.json_tasks.clear();
        for t in self.tasks.iter() {
            self.json_tasks.push(TaskJson::new(&t));
        }
        let json = serde_json::to_string(&self.json_tasks).unwrap();

        if let Err(e) = file.write_all(json.as_bytes()) {
            println!("Could not write to file: {}", e);
        }
    }

    pub fn load_from_file(&mut self, file_path: &str) {
        let mut file = match std::fs::File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    if let Ok(_) = std::fs::File::create(file_path) {
                        if let Ok(f) = std::fs::File::open(file_path) {
                            f
                        } else {
                            println!("Could open created file: {}", e);
                            return;
                        }
                    } else {
                        println!("Could not create file: {}", e);
                        return;
                    }
                } else {
                    println!("Could not open file: {}", e);
                    return;
                }
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            println!("Could not read file: {}", e);
            return;
        }
        if contents.is_empty() || contents == "[]" {
            println!("You have no tasks.");
            return;
        }

        let json: Vec<TaskJson> = match serde_json::from_str(&contents) {
            Ok(json) => json,
            Err(e) => {
                println!("Could not parse file: {}", e);
                return;
            }
        };

        for t in json {
            let mut task = Task::new(
                t.title,
                if t.description == "None" {
                    None
                } else {
                    Some(t.description)
                },
                t.importance,
                if t.due_date == "None" {
                    None
                } else {
                    Some(t.due_date.parse::<DateFormat>().unwrap())
                },
            );
            task.date_created = t.date_created.parse::<DateFormat>().unwrap();

            for sub_task in t.sub_tasks {
                let mut s_task = Task::new(
                    sub_task.title,
                    if sub_task.description == "None" {
                        None
                    } else {
                        Some(sub_task.description)
                    },
                    sub_task.importance,
                    if sub_task.due_date == "None" {
                        None
                    } else {
                        Some(sub_task.due_date.parse::<DateFormat>().unwrap())
                    },
                );
                s_task.date_created = sub_task.date_created.parse::<DateFormat>().unwrap();
                task.add_sub_task(s_task);
            }

            self.tasks.push(task);
        }
    }
}
