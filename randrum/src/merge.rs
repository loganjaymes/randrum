use rand::seq::IteratorRandom;
use std::{collections::HashMap, path::{PathBuf}};
use midly::{Smf, num::u4};
use std::fs;

#[derive(Debug)]
pub struct ChosenMIDI {
    // NOTE: Initially had struct as Option<PathBuf> since a user may only want a few instruments
    kick: Option<PathBuf>,
    snare: Option<PathBuf>,
    hat: Option<PathBuf>,
    crash: Option<PathBuf>,
    ride: Option<PathBuf>,
    rack_tom: Option<PathBuf>,
    floor_tom: Option<PathBuf>,
    // have user input decide if Some or None based on instruments included in args
    // hacky way: choose a random for all (worst case: bad for runtime), then convert to None based on what user wants before having midly merge the files
}

impl ChosenMIDI {
    pub fn export(&self) {
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

        // 1. unwrap & get path
        // TODO: Error handling, make sure not unwrapping a None
        let kick_mid = self.kick.as_ref().unwrap();
        // let snare_mid = self.snare.as_ref().unwrap();
        // let hat = self.hat.as_ref().unwrap();
        // let crash = self.crash.as_ref().unwrap();
        // let ride = self.ride.as_ref().unwrap();
        // let rack_tom = self.rack_tom.as_ref().unwrap();
        // let floor_tom = self.floor_tom.as_ref().unwrap();

        // 2. get all midi (if not none or sumshi)
        let kick_test_bytes = fs::read(kick_mid).unwrap();
        let kick_test_smf = Smf::parse(&kick_test_bytes).unwrap();

        // let snare_test_bytes = fs::read(snare_mid).unwrap();
        // let snare_test_smf = Smf::parse(&snare_test_bytes).unwrap();

        println!("{:?}\n", kick_test_smf);
        // println!("{:?}", snare_test_smf);

        /* NOTE: Since drum channels are not universal 
            (fe. someone could have a snare on A3 while another on C3), 
            just make sure each file is not on same channel (ie. alter byte)
        */

        // for each non-None midi file, call change_midi_channel and do so wrt proper channel as defined in enum

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

pub fn hmap_to_struct(mut hmap: HashMap<String, PathBuf>) -> ChosenMIDI {
    // option midi is unec. since individual fields are options (ie. it (((SHOULD BE))) fine even if all attr. are None)
    // can always change back if req
    let kick = hmap.remove("kick");
    let snare = hmap.remove("snare");
    let hat = hmap.remove("hat");
    let crash = hmap.remove("crash");
    let ride = hmap.remove("ride");
    let rack_tom = hmap.remove("rack_tom");
    let floor_tom = hmap.remove("floor_tom");

    ChosenMIDI {kick, snare, hat, crash, ride, rack_tom, floor_tom}
}

// TODO: implement fn that takes in a Smf object and changes each midi message's channel
pub fn change_midi_channel(mf: &mut midly::Smf, channel: String) {
    /*
    KickChannel = 0,
    SnareChannel = 1,
    RackTomChannel = 2,
    FloorTomChannel = 3,
    HatChannel = 4,
    RideChannel = 5,
    CrashChannel = 6,
    */

    /*
    match channel {
        "snare" => // iterate over smf and change
        _ => do nothing // INCLUDES KICK since kick is on channel 0 (default channel) 
    }   
    */
}