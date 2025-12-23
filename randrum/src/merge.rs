use rand::seq::IteratorRandom;
use std::path::PathBuf;

pub struct ChosenMIDI {
    // strings will be paths most likely (potentially change to PathBufs depending on midly's arguments for Smf)
    kick: Option<String>,
    snare: Option<String>,
    hat: Option<String>,
    crash: Option<String>,
    ride: Option<String>,
    tom: Option<String>,
    // have user input decide if Some or None based on instruments included in args
    // hacky way: choose a random for all (worst case: bad for runtime), then convert to None based on what user wants before having midly merge the files
}

pub fn pick_rand(path: PathBuf) {//-> ChosenMIDI {
    let paths = path.read_dir().expect("Path should never change so it should be fine.");
    
    for dir_res in paths { // only iterate over dir if res == ok
        if let Ok(dir) = dir_res {
            let subfolder = dir.path();

            let mut rng = rand::rng();
            println!("fold: {:?}", subfolder.file_name().unwrap());
            
            if let Some(file) = subfolder.read_dir().expect("Path shouldn't change").filter_map(Result::ok).choose(&mut rng) {
                // above: all valid directory entries go into a map and one is chosen at random
                println!("chose: {:?}", file.file_name());
            }
        }
    }
}