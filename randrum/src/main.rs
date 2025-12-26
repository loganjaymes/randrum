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

    /*
    TODO: 
    2. merge each Smf object into one
    3. """""export""""" .mid file 
        (unsure if right terminology)
    4. set up CLI w/ CLAP
    5. allow users to select what drums they want
    6. ??? (67)
    7. Done ! Smile.
    */

    a.export("a");
}
