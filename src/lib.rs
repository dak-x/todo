#![allow(dead_code, unused)]
// pub const CONFIG_PATH: &str = ".config/todo/todo_list.json";

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::env;
use std::fmt::Display;

type Time = NaiveDate;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TodoList {
    pub author: String,
    tasks: Vec<TodoTask>,
}

impl Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        writeln!(
            f,
            "    {} {}",
            self.author.bright_yellow(),
            "Todo:".bright_yellow()
        );

        for task in &self.tasks {
            writeln!(f, "   {}", task);
        }

        if self.tasks.is_empty() {
            writeln! {f, "{}" ,"    You are good. No Tasks!".yellow()};
        }
        write!(f, "")
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
    created: NaiveDate,
    desc: String,
    deadline: Option<NaiveDate>,
    priority: Priority,
}

impl Display for TodoTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        let _col: &str = match self.priority {
            Priority::URGENT => "bright red",
            Priority::MODERATE => "green",
            Priority::CHILL => "cyan",
            Priority::NONE => "white",
        };
        write!(
            f,
            "[{}] {}: {}  {}",
            self.id.to_string().color(_col),
            self.title.color(_col),
            self.desc.color(_col),
            self.created.format("%v").to_string().color("white"),
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
    // Get the todolist stored in the config json
    pub fn from_config() -> Result<Self, Box<dyn std::error::Error>> {
        TodoList::from_path(CONFIG_PATH)
    }
    // Get the todolist from any specified json
    fn from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents)?)
    }
    // Write the contents of the todolist to path.json
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
    /// Reassign the id order for the tasks
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

    // Print a subset of the tasks randomly based on priority
    pub fn print_till(&self, till: u32) {
        use Priority::*;
        let up_bnd: Priority = match till {
            x if x < 4 => URGENT,
            x if x < 6 => MODERATE,
            x if x < 8 => CHILL,
            _ => NONE,
        };

        use colored::Colorize;
        println!(
            " {} {}",
            self.author.bright_yellow(),
            "Todo:".bright_yellow()
        );
        for task in &self.tasks {
            if task.priority <= up_bnd {
                println!("   {}", task);
            }
        }
        if self.tasks.is_empty() {
            println! {"{}" ," You are good. No Tasks!".yellow()};
        }
    }
}

impl TodoTask {
    fn new(title: String, desc: String, _priority: Option<Priority>, _deadline: usize) -> Self {
        let created: NaiveDate = Local::today().naive_local();
        let deadline: Option<Time> = None;
        if _deadline > 0 {
            // TODO: Manage the deadline thing
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
const CONFIG_PATH: &str = "/home/anthrax/.config/todo/todo.json";
