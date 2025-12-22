/*
mod args;

use args::RanDrumArgs;
use clap::{Args, Parser, Subcommand};
*/
use::walkdir::WalkDir;

fn main() {
    // let args = RanDrumArgs::parse();
    for entry in WalkDir::new("input") {
        println!("{}", entry.unwrap().path().display());
    }

}
