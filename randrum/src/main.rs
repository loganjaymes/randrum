/*
mod args;

use args::RanDrumArgs;
use clap::{Args, Parser, Subcommand};
*/
mod merge;

fn main() {
    // let args = RanDrumArgs::parse();
    merge::run("input");

}
