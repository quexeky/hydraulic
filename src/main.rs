mod cli;

use clap::Parser;
use crate::cli::args::{Cli};

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
