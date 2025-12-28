use std::path::PathBuf;
/*
mod args;

use args::RanDrumArgs;
use clap::{Args, Parser, Subcommand};
*/
mod merge;

fn main() {
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

    // let args = RanDrumArgs::parse();
    let path = PathBuf::from("input");
    let picked_files = merge::pick_rand(path, ["kick".to_string(), "rack_tom".to_string(), "floor_tom".to_string()].to_vec());
    // let mut a = merge::hmap_to_struct(picked_files);
    // a.export("test");
}
