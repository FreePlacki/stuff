//! A terminal todo list application.
//! The tasks are printed nicely, in color to the terminal.
//! Tasks can contain sub tasks.
//! You can save the tasks to a file.
mod date;

use date::Date;

#[derive(Debug)]
struct Task {
    title: String,
    description: String,
    importance: u8,
    due_date: Option<Date>,
    date_created: Date,
    sub_tasks: Vec<Task>,
}

impl Task {
    fn new(title: String, description: String, importance: u8, due_date: Option<Date>) -> Task {
        Task {
            title,
            description,
            importance,
            due_date,
            date_created: Date::get_current_date(),
            sub_tasks: Vec::new(),
        }
    }

    fn print_task(&self) {
        match self.importance {
            3 => print!("\x1b[1;37;31m{}\x1b[0m", self.title),
            2 => print!("\x1b[1;37;33m{}\x1b[0m", self.title),
            1 => print!("\x1b[1;37;36m{}\x1b[0m", self.title),
            _ => print!("{}", self.title),
        }

        if let Some(due_date) = &self.due_date {
            let time_left = due_date.get_time_left();

            println!("\t\x1b[1;37;30m [due in {}]\x1b[0m", time_left);
        }

        if self.description != "" {
            println!("{}", self.description);
        }
    }

    fn add_sub_task(&mut self, sub_task: Task) {
        self.sub_tasks.push(sub_task);
    }
}

fn main() {
    let task = Task::new(
        "Task 1".to_string(),
        "This is a task".to_string(),
        1,
        Some(Date::new(2022, 4, 5, 0, 0, 0)),
    );

    task.print_task();
    println!("created {} ago", task.date_created.get_time_left());
}
