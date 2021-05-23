use clap::Clap;

mod cli;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
