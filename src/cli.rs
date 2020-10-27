#![allow(dead_code, unused)]
use structopt::StructOpt;
use todo::*;

#[structopt(
    name = "Todo",
    about = "Todo List Manager written in Rust. Use this tool to add ",
    version = "1.0.0"
)]
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
    #[structopt(name = "show", about = "Show the list of all Tasks")]
    Show,

    #[structopt(name = "add", about = "Add a new Task")]
    Add(AddArgs),

    #[structopt(name = "remove", about = "Remove a bunch of Tasks")]
    Remove(RemoveArgs),

    #[structopt(name = "RESET", about = "Clear all your Tasks")]
    Reset,
}

#[derive(Debug, StructOpt)]
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
    /// Decode the command given in cli_args.
    pub fn handle(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo_list = match self.sbcmd {
            Some(SbCmd::Reset) => TodoList::default(),
            _ => TodoList::from_config()?,
        };

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
            None if self.author.is_none() => {
                use rand::random;
                let x = random::<u32>() % 10;
                todo_list.print_till(x);
            }
            _ => {}
        }

        match self.author {
            Some(x) => todo_list.author = x.to_string(),
            _ => {}
        }
        Ok(())
    }
}
