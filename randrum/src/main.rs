use std::path::PathBuf;

/*
mod args;

use args::RanDrumArgs;
use clap::{Args, Parser, Subcommand};
*/
mod merge;

fn main() {
    // let args = RanDrumArgs::parse();
    let mut path = PathBuf::from("input");
    merge::pick_rand(path);
}
