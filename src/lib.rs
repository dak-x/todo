#![allow(dead_code, unused)]

const CONFIG_PATH: &str = "todo_list.json";

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt::Display;

type Time = DateTime<Local>;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TodoList {
    pub author: String,
    tasks: Vec<TodoTask>,
}

impl Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        writeln!(f, "{}'s {}", self.author.bright_yellow().bold(), "Todo:".bright_yellow().bold());
        for task in &self.tasks {
            writeln!(f, "       {}", task);
        }
        if self.tasks.is_empty(){
            writeln!{f, "{}" ,"Looks like you are good! No Tasks!!!".magenta()};
            write! {f,"{}","     -- -- -- -- -- -- --".green()};
        }
        write!(f,"")
    }
}

impl Drop for TodoList {
    fn drop(&mut self) {
        self.to_file(CONFIG_PATH);
    }
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

impl Display for TodoTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        let _col: &str = match self.priority {
            Priority::URGENT => "red",
            Priority::MODERATE => "green",
            Priority::CHILL => "cyan",
            Priority::NONE => "white",
        };
        write!(
            f,
            "[{}]{}: {}",
            self.id.to_string().color(_col),
            self.title.color(_col),
            self.desc.color(_col)
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    URGENT,
    MODERATE,
    CHILL,
    NONE,
}

impl TodoList {
    pub fn from_config() -> Result<Self, Box<dyn std::error::Error>> {
        TodoList::from_path(CONFIG_PATH)
    }

    fn from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::prelude::*;
        let file = File::create(CONFIG_PATH)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    /// Add a new task with given parameters
    pub fn add_task(
        &mut self,
        title: String,
        desc: String,
        priority: Option<Priority>,
        deadline: usize,
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
    pub fn remove_tasks_id(&mut self, ids: &Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {
        for id in ids {
            if *id >= self.tasks.len() {
                return Err(Box::from(format! {"Task id{:} out of bounds",id}));
            }
        }
        self.tasks.retain(|x| !ids.contains(&x.id));
        self.orderize();
        Ok(())
    }


    fn orderize(&mut self) {
        for (i, x) in self.tasks.iter_mut().enumerate() {
            x.id = i;
        }
    }

    /// Erase all contents of the list and start fresh
    pub fn reset() -> Result<(), Box<dyn std::error::Error>> {
        let _t = TodoList::default();
        _t.to_file(CONFIG_PATH)
    }
}

impl TodoTask {
    fn new(title: String, desc: String, _priority: Option<Priority>, _deadline: usize) -> Self {
        let created = Local::now();
        let deadline: Option<Time> = None;
        if _deadline > 0 {
            // TODO: ...@
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
