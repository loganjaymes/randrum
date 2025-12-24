use rand::seq::IteratorRandom;
use std::{collections::HashMap, fs::DirEntry, path::PathBuf};

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

pub fn pick_rand(path: PathBuf) -> HashMap<String, DirEntry> {
    let paths = path.read_dir().expect("Path should never change so it should be fine.");
    let mut hmap: HashMap<String, DirEntry> = HashMap::new(); // TODO potentially deserialze with Serde, else a match statement but that might be chopped
    
    for dir_res in paths { // only iterate over dir if res == ok
        if let Ok(dir) = dir_res {
            let subfolder = dir.path();
            let mut rng = rand::rng();
            
            if let Some(file) = subfolder.read_dir().expect("Path shouldn't change").filter_map(Result::ok).choose(&mut rng) {
                // above: all valid directory entries go into a map and one is chosen at random
                let instrument = subfolder.file_name().unwrap();
                hmap.insert(instrument.to_string_lossy().to_string(), file);
                // convert to cow pointer to allow ownership into hashmap after it is dropped
            }
        }
    }

    hmap
}

pub fn deserialze(hmap: HashMap<String, String>) -> Option<ChosenMIDI> {
    // Source - https://stackoverflow.com/a
    // Posted by Masklinn, modified by community. See post 'Timeline' for change history
    // Retrieved 2025-12-24, License - CC BY-SA 4.0
    serde_json::to_value(h).ok()
        .and_then(|v| ChosenMIDI::deserialize(v).ok())
}