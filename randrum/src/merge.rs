use rand::seq::IteratorRandom;
use std::{collections::HashMap, path::{PathBuf}};
use midly::{Smf, num::u4};
use std::fs;

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::fs;
    use crate::merge::ChosenMIDI;
    use midly::{Smf, num::u4};

    #[test]
    fn k4fs24() {
        let correct_bytes = fs::read("test/k4s24.MID").unwrap();
        let correct_smf = Smf::parse(&correct_bytes).unwrap();

        let test_mid = ChosenMIDI { 
            kick: Some("input/kick/4onfloor.MID".into()), 
            snare: Some("input/snare/2and4.MID".into()), 
            hat: None, 
            crash: None, 
            ride: None, 
            rack_tom: None, 
            floor_tom: None, 
        };

        test_mid.export("TESTk4fs24.MID"); 

        let test_bytes = fs::read("exports/TESTk4fs24.MID").unwrap();
        let test_smf = Smf::parse(&test_bytes).unwrap();

        assert_eq!(test_smf, correct_smf);
    }

    fn k13s24h8() {
        let correct_bytes = fs::read("exports/TESTk13s24h8.MID").unwrap();
        let correct_smf = Smf::parse(&correct_bytes).unwrap();

        let test_mid = ChosenMIDI { 
            kick: Some("input/kick/1and3.MID".into()), 
            snare: Some("input/snare/2and4.MID".into()), 
            hat: Some("input/hihat/straight8ths.MID".into()), 
            crash: None, 
            ride: None, 
            rack_tom: None, 
            floor_tom: None, 
        };

        test_mid.export("TESTk13s24h8.MID"); 

        let test_bytes = fs::read("exports/TESTk13s24h8.MID").unwrap();
        let test_smf = Smf::parse(&test_bytes).unwrap();

        assert_eq!(test_smf, correct_smf);
    }
}

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
    pub fn export(&self, name: &str) {
        /*
        TODO: 
        0. name validation (.ie ends in .MID/normalization)
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
        // let kick_mid = self.kick.as_ref().unwrap();
        // let snare_mid = self.snare.as_ref().unwrap();

        let kick_mid = "input/kick/4onfloor.MID";
        let snare_mid = "input/snare/2and4.MID";


        // let hat = self.hat.as_ref().unwrap();
        // let crash = self.crash.as_ref().unwrap();
        // let ride = self.ride.as_ref().unwrap();
        // let rack_tom = self.rack_tom.as_ref().unwrap();
        // let floor_tom = self.floor_tom.as_ref().unwrap();

        // 2. get all midi (if not none or sumshi)
        let kick_test_bytes = fs::read(kick_mid).unwrap();
        let kick_test_smf = Smf::parse(&kick_test_bytes).unwrap();

        let snare_test_bytes = fs::read(snare_mid).unwrap();
        let snare_test_smf = Smf::parse(&snare_test_bytes).unwrap();

        println!("{:?}\n\n", kick_test_smf.tracks);
        println!("{:?}", snare_test_smf);

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