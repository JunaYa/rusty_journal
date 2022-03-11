mod cli;
mod tasks;
use structopt::StructOpt;

fn main() {
    let result = cli::CommandLineArgs::from_args();
    println!("{:#?}", result);
}
