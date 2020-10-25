mod cli;
use todo::*;
use structopt::StructOpt;
use cli::*;

fn main() {
    let _x = TodoCli::from_args();
    print!("{:?}",_x)
}
