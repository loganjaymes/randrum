use std::path::PathBuf;

/*
mod args;

use args::RanDrumArgs;
use clap::{Args, Parser, Subcommand};
*/
mod merge;

fn main() {
    // let args = RanDrumArgs::parse();
    let path = PathBuf::from("input");
    let picked_files = merge::pick_rand(path);

    let a = merge::hmap_to_struct(picked_files);

    // println!("{:?}", a.unwrap());
    
    a.export();
}
