mod cli;
use cli::*;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    TodoCli::from_args().handle()?;
    Ok(())
}

