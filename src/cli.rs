#![allow(dead_code, unused)]
use structopt::StructOpt;
use todo::*;

#[structopt(about = "ToDo List Manager written in Rust.")]
#[derive(StructOpt, Debug)]
pub struct TodoCli {
    #[structopt(subcommand)]
    sbcmd: Option<SbCmd>,
    /// Select to Display all the Tasks
    #[structopt(short, long)]
    all: bool,

    // Change name of the author
    #[structopt(long)]
    author: Option<String>,
}

#[derive(Debug, StructOpt)]
enum SbCmd {
    /// Display all the tasks
    Show,
    /// Add a new Todo task
    Add(AddArgs),
    /// Complete/Remove a task
    Remove(RemoveArgs),
    /// Reset the list
    Reset,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "ToDo Add", about = "Add a new Task in your ToDo List")]
struct AddArgs {
    /// Title for the Task
    title: String,
    /// Small description for the Task
    description: String,
    /// Deadline in Number of days.
    #[structopt(long, default_value = "0")]
    deadline: usize,
    /// Set Urgent Priority
    #[structopt(short, conflicts_with("m"))]
    u: bool,
    /// Set Moderate Priority
    #[structopt(short, conflicts_with("u"))]
    m: bool,
    #[structopt(short, conflicts_with("u"))]
    c: bool,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "ToDo Remove", about = "Remove a task from your ToDo List")]
struct RemoveArgs {
    /// Id's for tasks to remove from the list
    ids: Vec<usize>,
    // id: Option<u32>,
}

impl TodoCli {
    pub fn handle(self, mut todo_list: TodoList) {
        match self.sbcmd {
            Some(SbCmd::Show) => println!("{}", todo_list),
            Some(SbCmd::Add(args)) => {
                let prior = {
                    if args.u {
                        Priority::URGENT
                    } else if args.m {
                        Priority::MODERATE
                    } else if args.c {
                        Priority::CHILL
                    } else {
                        Priority::NONE
                    }
                };

                todo_list.add_task(
                    args.title,
                    args.description,
                    Some(prior),
                    args.deadline as usize,
                );
            }
            Some(SbCmd::Remove(args)) => {
                todo_list.remove_tasks_id(&args.ids);
            }
            Some(SbCmd::Reset) => {
                todo_list = TodoList::default();
            }
            _ => {}
        }

        match self.author {
            Some(x) => todo_list.author = x.to_string(),
            _ => {}
        }
    }
}
