mod cli;
use cli::*;
use structopt::StructOpt;
use todo::*;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}",CONFIG_PATH);
    let _x = TodoCli::from_args();
    let todo_list = TodoList::from_config()?;
    _x.handle(todo_list);
    Ok(())
}

