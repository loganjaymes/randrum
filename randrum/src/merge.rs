use rand::seq::IteratorRandom;
use std::{collections::HashMap, path::{PathBuf}};
use midly::Smf;
use std::fs;

#[derive(Debug)]
pub struct ChosenMIDI {
    // NOTE: Initially had struct as Option<PathBuf> since a user may only want a few instruments
    kick: Option<PathBuf>,
    snare: Option<PathBuf>,
    hat: Option<PathBuf>,
    crash: Option<PathBuf>,
    ride: Option<PathBuf>,
    high_tom: Option<PathBuf>,
    low_tom: Option<PathBuf>,
    // have user input decide if Some or None based on instruments included in args
    // hacky way: choose a random for all (worst case: bad for runtime), then convert to None based on what user wants before having midly merge the files
}

impl ChosenMIDI {
    pub fn idk_yet(cm: ChosenMIDI) {
        /* stopping for tn, final outline: 
        TODO: 
        1. read from struct into midi (check if Path or PathBuf or String idfk)
            (unsure if needing to store in a struct or just return the file. we shall see)
        2. merge each Smf object into one
        3. """""export""""" .mid file 
            (unsure if right terminology)
        4. set up CLI w/ CLAP
        5. allow users to select what drums they want
        6. ??? (67)
        7. Done ! Smile.
        */

        // 1. unwrap & get path
        let kick = cm.kick.unwrap();
        let snare = cm.snare.unwrap();
        let hat = cm.hat.unwrap();
        let crash = cm.crash.unwrap();
        let ride = cm.ride.unwrap();
        let high_tom = cm.high_tom.unwrap();
        let low_tom = cm.low_tom.unwrap();

        // 2. get all midi (if not none or sumshi)
        let test_bytes = fs::read(kick).unwrap();
        let test_smf = Smf::parse(&test_bytes).unwrap();


    }
}

pub fn pick_rand(path: PathBuf) -> HashMap<String, PathBuf> {
    let paths = path.read_dir().expect("Path should never change so it should be fine.");
    let mut hmap: HashMap<String, PathBuf> = HashMap::new(); 
    // above is string and not pathbuf to ensure hashmap ownership
    
    for dir_res in paths { // only iterate over dir if res == ok
        if let Ok(dir) = dir_res {
            let subfolder = dir.path();
            let mut rng = rand::rng();
            
            if let Some(file) = subfolder.read_dir().expect("Path shouldn't change").filter_map(Result::ok).choose(&mut rng) {
                // above: all valid directory entries go into a map and one is chosen at random
                let instrument = subfolder.file_name().unwrap();
                let midi = file.path();
                hmap.insert(instrument.to_string_lossy().to_string(), midi);
                // convert to cow pointer to allow ownership into hashmap after it is dropped
            }
        }
    }

    hmap
}

pub fn hmap_to_struct(mut hmap: HashMap<String, PathBuf>) -> Option<ChosenMIDI> {
    let kick = hmap.remove("kick");
    let snare = hmap.remove("snare");
    let hat = hmap.remove("hat");
    let crash = hmap.remove("crash");
    let ride = hmap.remove("ride");
    let high_tom = hmap.remove("high_tom");
    let low_tom = hmap.remove("low_tom");

    Some(ChosenMIDI {kick, snare, hat, crash, ride, high_tom, low_tom})
}