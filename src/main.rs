//! A terminal todo list application.
//! The tasks are printed nicely, in color to the terminal.
//! Tasks can contain sub tasks.
//! You can save the tasks to a file.
mod date;
mod task;

use chrono::prelude::*;
use task::{Task, TaskList};

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
    task_list.save_to_file("saved_stuff.json");
}
