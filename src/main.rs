mod cli;

use std::fs;
use std::fs::File;
use clap::Parser;
use hydraulic::Gzip;

use hydraulic::prelude::*;
use crate::cli::args::Args;

fn main() {
    let args = Args::parse();
}
