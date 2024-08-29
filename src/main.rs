mod cli;

use clap::Parser;
use crate::cli::args::Args;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
