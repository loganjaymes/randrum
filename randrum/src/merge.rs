use rand::seq::IteratorRandom;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};
use midly::Smf;
use std::fs;

/* IGNORE TESTS SINCE COMPARING ISNT 100% ACCURATE
#[cfg(test)]
mod test {
    use std::fs;
    use crate::merge::ChosenMIDI;
    use midly::Smf;

    #[test]
    fn k4fs24() {
        let correct_bytes = fs::read("test/2Sk4fs24.MID").unwrap();
        // let correct_smf = Smf::parse(&correct_bytes).unwrap();

        let test_mid = ChosenMIDI { 
            // combining tracks may not result in same order but correct output; potentially sort and compare bytes. somewhat inefficient but yeah
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
        // let test_smf = Smf::parse(&test_bytes).unwrap();

        assert_eq!(test_bytes, correct_bytes); // may need to compare bytes only, since event order (apparently) does not matter
    }

    /*
    fn k13s24h8() {
        let correct_bytes = fs::read("exports/TESTk13s24h8.MID").unwrap().sort();
        // let correct_smf = Smf::parse(&correct_bytes).unwrap();

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

        let test_bytes = fs::read("exports/TESTk13s24h8.MID").unwrap().sort();
        // let test_smf = Smf::parse(&test_bytes).unwrap();

        assert_eq!(test_bytes, correct_bytes);
    }
    */
}
*/ 

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
        2. merge each Smf object into one
        3. """""export""""" .mid file 
            (unsure if right terminology)
        4. set up CLI w/ CLAP
        5. allow users to select what drums they want
        6. ??? (67)
        7. Done ! Smile.
        */

        let valid_name = export_name_validation(name.to_string());

        // 1. unwrap & get path
        // TODO: Error handling, make sure not unwrapping a None; might be better practice to write unwrap_struct fn
        let kick_mid = self.kick.as_ref().unwrap();
        let snare_mid = self.snare.as_ref().unwrap();
        let hat_mid = self.hat.as_ref().unwrap();
        // let crash_mid = self.crash.as_ref().unwrap();
        // let ride_mid = self.ride.as_ref().unwrap();
        // let rack_tom_mid = self.rack_tom.as_ref().unwrap();
        // let floor_tom_mid = self.floor_tom.as_ref().unwrap();

        // 2. get all midi (if not none or sumshi)
        let kick_test_bytes = fs::read(kick_mid).unwrap();
        let mut kick_test_smf = Smf::parse(&kick_test_bytes).unwrap();

        let snare_test_bytes = fs::read(snare_mid).unwrap();
        let mut snare_test_smf = Smf::parse(&snare_test_bytes).unwrap();

        let hat_test_bytes = fs::read(hat_mid).unwrap();
        let mut hat_test_smf = Smf::parse(&hat_test_bytes).unwrap();

        // use kick as "base layer"
        kick_test_smf.tracks.append(&mut snare_test_smf.tracks);
        kick_test_smf.tracks.append(&mut hat_test_smf.tracks);


        // export!
        let mut export_mem = Vec::new();
        kick_test_smf.write(&mut export_mem).unwrap();
        let mut test_f = File::create(valid_name).unwrap();
        test_f.write_all(&export_mem).expect("write unsucc");
        // NOTE: cannot use Smf::save since (on windows) no perms and unable to change writer attr
        // export.save(name).unwrap();
    }
}

pub fn export_name_validation(mut name: String) -> String {
    if !name.ends_with(".MID") {
        name = format!("{}{}", name, ".MID");
    }

    if !name.starts_with("exports/") {
        name = format!("{}{}", "exports/", name);
    }

    name
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
    let hat = hmap.remove("hihat");
    let crash = hmap.remove("crash");
    let ride = hmap.remove("ride");
    let rack_tom = hmap.remove("rack_tom");
    let floor_tom = hmap.remove("floor_tom");

    ChosenMIDI {kick, snare, hat, crash, ride, rack_tom, floor_tom}
}