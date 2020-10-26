use structopt::StructOpt;

#[structopt(about = "ToDo List Manager written in Rust.")]
#[derive(StructOpt, Debug)]
pub struct TodoCli {
    #[structopt(subcommand)]
    sbcmd: Option<SbCmd>,
    /// Select to Display all the Tasks
    #[structopt(short, long)]
    all: bool,
    // TODO: Maybe add a author option
    // Change name of the author
    // #[structopt(long)]
    // author: Option<String>,
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
#[structopt(name = "ToDo Add", about = "Add a new Task in your ToDo List")]
struct AddArgs {
    /// Title for the Task
    title: String,
    /// Small description for the Task
    description: String,
    /// Deadline in Number of days.
    #[structopt(long, default_value = "-1")]
    deadline: i32,
    /// Set Urgent Priority
    #[structopt(short, conflicts_with("m"))]
    u: bool,
    /// Set Moderate Priority
    #[structopt(short, conflicts_with("u"))]
    m: bool,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "ToDo Remove", about = "Remove a task from your ToDo List")]
struct RemoveArgs {
    /// Id's for tasks to remove from the list
    title: Vec<String>,
    // id: Option<u32>,
}
