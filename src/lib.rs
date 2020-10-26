const CONFIG_PATH: &str = "~/.config/todo/todo_list.txt";

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::File;
use std::io::prelude::*;

type Time = DateTime<Local>;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct TodoList {
    author: String,
    tasks: Vec<TodoTask>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TodoTask {
    id: usize,
    title: String,
    created: Time,
    desc: String,
    deadline: Option<Time>,
    priority: Priority,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    URGENT,
    MODERATE,
    CHILL,
    NONE,
}

impl TodoList {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        TodoList::from_path(CONFIG_PATH)
    }

    fn from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents)?)
    }
    /// Add a new task with given parameters
    pub fn add_task(
        &mut self,
        title: String,
        desc: String,
        priority: Option<Priority>,
        deadline: i32,
    ) {
        let task = TodoTask::new(title, desc, priority, deadline);
        self.tasks.push(task);
        self.tasks.sort_by(TodoTask::priority_order);
        self.orderize();
    }
    /// Remove the tasks via the title
    pub fn remove_task_title(&mut self, title: &str) {
        let title = title.to_string();
        self.tasks.retain(|x| *x.title == title);
        self.orderize()
    }
    /// Remove tasks via the id
    pub fn remove_task_id(&mut self, id: usize) -> Result<(), Box<dyn std::error::Error>> {
        if id < 0 || id >= self.tasks.len() {
            self.tasks.remove(id);
            self.orderize();
            Ok(())
        } else {
            Err(Box::from("Task id out of bounds"))
        }
    }

    fn orderize(&mut self) {
        for (i, x) in self.tasks.iter_mut().enumerate() {
            x.id = i;
        }
    }
}

impl TodoTask {
    fn new(title: String, desc: String, _priority: Option<Priority>, _deadline: i32) -> Self {
        let created = Local::now();
        let deadline: Option<Time> = None;
        if _deadline > 0 {
            // TODO: ...
            // deadline = Some()
        }
        let priority = _priority.unwrap_or_else(|| Priority::NONE);
        TodoTask {
            id: 0,
            title,
            desc,
            priority,
            deadline,
            created,
        }
    }

    fn priority_order(t1: &TodoTask, t2: &TodoTask) -> std::cmp::Ordering {
        t1.priority.cmp(&t2.priority)
    }
}
