const CONFIG_PATH: &str = "~/.config/todo/todo_list.txt";

use chrono::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::*;

type Time = DateTime<Local>;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TodoList {
    author: String,
    tasks: Vec<TodoTask>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TodoTask {
    created: Time,
    description: String,
    deadline: Option<Time>,
    priority: Priority,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Priority {
    URGENT,
    MODERATE,
    CHILL,
    NONE,
}
impl Default for Priority {
    fn default() -> Self {
        Priority::NONE
    }
}

impl TodoList {}
impl TodoTask {}
