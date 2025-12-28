use rand::seq::IteratorRandom;
use std::{collections::HashMap, fs::{self, File}, io::Write, path::PathBuf};
use midly::Smf;

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
    kick: PathBuf,
    snare: PathBuf,
    hat: PathBuf,
    crash: PathBuf,
    ride: PathBuf,
    toms: PathBuf,
}

impl ChosenMIDI {
    pub fn export(&mut self, name: &str) {
        // self.unwrap_struct();
        // println!("{:?}", self.stored_unwraps);
        
        let valid_name = export_name_validation(name.to_string());

        // 960 may be incorrect/not always true, based on time sig
        // let temp_header = Header::new(Format::Parallel, Timing::Metrical(960.into()));
        // let mut init_smf = Smf::new(temp_header);
        
        let kick_test_bytes = fs::read(&self.kick).unwrap();
        let mut kick_test_smf = Smf::parse(&kick_test_bytes).unwrap();
        
        let snare_test_bytes = fs::read(&self.snare).unwrap();
        let mut snare_test_smf = Smf::parse(&snare_test_bytes).unwrap();
        
        let hat_test_bytes = fs::read(&self.hat).unwrap();
        let mut hat_test_smf = Smf::parse(&hat_test_bytes).unwrap();
        
        let crash_test_bytes = fs::read(&self.crash).unwrap();
        let mut crash_test_smf = Smf::parse(&crash_test_bytes).unwrap();

        let ride_test_bytes = fs::read(&self.ride).unwrap();
        let mut ride_test_smf = Smf::parse(&ride_test_bytes).unwrap();

        let toms_test_bytes = fs::read(&self.toms).unwrap();
        let mut toms_test_smf = Smf::parse(&toms_test_bytes).unwrap();

        kick_test_smf.tracks.append(&mut snare_test_smf.tracks);
        kick_test_smf.tracks.append(&mut hat_test_smf.tracks);
        kick_test_smf.tracks.append(&mut crash_test_smf.tracks);
        kick_test_smf.tracks.append(&mut ride_test_smf.tracks);
        kick_test_smf.tracks.append(&mut toms_test_smf.tracks);

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

pub fn pick_rand(path: PathBuf, nones: Vec<String>) -> HashMap<String, PathBuf> {
    println!("{:?}", nones);
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
                if !nones.contains(&instrument.to_string_lossy().to_string()) {
                    let midi = file.path();
                    hmap.insert(instrument.to_string_lossy().to_string(), midi);
                } else {
                    let midi = "input/none/none.MID";
                    hmap.insert(instrument.to_string_lossy().to_string(), midi.into());
                }
                // convert to cow pointer to allow ownership into hashmap after it is dropped
            }
        }
    }

    // println!("{:?}", hmap);
    hmap
}

pub fn hmap_to_struct(mut hmap: HashMap<String, PathBuf>) -> ChosenMIDI {
    let kick = hmap.remove("kick").unwrap();
    let snare = hmap.remove("snare").unwrap();
    let hat = hmap.remove("hihat").unwrap();
    let crash = hmap.remove("crash").unwrap();
    let ride = hmap.remove("ride").unwrap();
    let toms = hmap.remove("toms").unwrap();

    ChosenMIDI {kick, snare, hat, crash, ride, toms }
}