use rand::seq::IteratorRandom;
use std::{fs, collections::HashMap, path::PathBuf};
use midly::{Format, Header, Smf, Timing};

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
    kick: Option<PathBuf>,
    snare: Option<PathBuf>,
    hat: Option<PathBuf>,
    crash: Option<PathBuf>,
    ride: Option<PathBuf>,
    rack_tom: Option<PathBuf>,
    floor_tom: Option<PathBuf>,
    stored_unwraps: Vec<PathBuf>,
    // have user input decide if Some or None based on instruments included in args
}

impl ChosenMIDI {
    pub fn unwrap_struct(&mut self) {
        // holy chuzz 
        if let Some(kick_mid) = &self.kick { self.stored_unwraps.push(kick_mid.to_path_buf()); }   
        if let Some(snare_mid) = &self.snare { self.stored_unwraps.push(snare_mid.to_path_buf()); }  
        if let Some(hat_mid) = &self.hat { self.stored_unwraps.push(hat_mid.to_path_buf()); }  
        if let Some(crash_mid) = &self.crash { self.stored_unwraps.push(crash_mid.to_path_buf()); }  
        if let Some(ride_mid) = &self.ride { self.stored_unwraps.push(ride_mid.to_path_buf()); }  
        if let Some(rack_mid) = &self.rack_tom { self.stored_unwraps.push(rack_mid.to_path_buf()); }  
        if let Some(floor_mid) = &self.floor_tom { self.stored_unwraps.push(floor_mid.to_path_buf()); }  
    }

    pub fn export(&mut self, name: &str) {
        self.unwrap_struct();
        println!("{:?}", self.stored_unwraps);
        
        let valid_name = export_name_validation(name.to_string());

        // 960 may be incorrect/not always true, based on time sig
        let temp_header = Header::new(Format::Parallel, Timing::Metrical(960.into()));
        let mut init_smf = Smf::new(temp_header);

        /* nop
        for (i, path) in self.stored_unwraps.iter().enumerate() {
            let temp_bytes = fs::read(path).unwrap();
            let temp_smf = Smf::parse(&temp_bytes).unwrap();
            init_smf.tracks.extend(temp_smf.tracks);
        }
        */
        
        /*
        let kick_test_bytes = fs::read(kick_mid).unwrap();
        let mut kick_test_smf = Smf::parse(&kick_test_bytes).unwrap();

        let snare_test_bytes = fs::read(snare_mid).unwrap();
        let mut snare_test_smf = Smf::parse(&snare_test_bytes).unwrap();

        let hat_test_bytes = fs::read(hat_mid).unwrap();
        let mut hat_test_smf = Smf::parse(&hat_test_bytes).unwrap();

        // export!
        let mut export_mem = Vec::new();
        init_smf.write(&mut export_mem).unwrap();
        let mut test_f = File::create(valid_name).unwrap();
        test_f.write_all(&export_mem).expect("write unsucc");
        // NOTE: cannot use Smf::save since (on windows) no perms and unable to change writer attr
        // export.save(name).unwrap();
        */
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
    let stored_unwraps: Vec<PathBuf> = Vec::new();

    ChosenMIDI {kick, snare, hat, crash, ride, rack_tom, floor_tom, stored_unwraps }
}