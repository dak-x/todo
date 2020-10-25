use structopt::StructOpt;

#[structopt(about = "ToDo List Manager written in Rust.")]
#[derive(StructOpt, Debug)]
pub struct TodoCli {
    #[structopt(subcommand)]
    sbcmd: Option<SbCmd>,

    /// Select to Display all the Tasks
    #[structopt(short, long)]
    all: bool,
    #[structopt(long)]
    author: bool,
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

// #[derive(Debug, StructOpt)]
// struct ShowArgs {}

#[derive(Debug, StructOpt)]
#[structopt(name = "ToDo Add", about ="Add a new Task in your ToDo List")]
struct AddArgs {
    /// Title for the Task
    title: String,
    /// Small description for the Task
    description: String,
    /// Deadline in Number of days.
    #[structopt(long, default_value = "-1")]
    deadline: i32,

    /// Set Urgent Priority
    #[structopt(short)]
    u: bool,
    /// Set Moderate Priority
    #[structopt(short)]
    m: bool,
}

#[derive(Debug, StructOpt)]
struct RemoveArgs {}
