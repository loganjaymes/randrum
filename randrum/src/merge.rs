use rand::seq::IteratorRandom;
use std::collections::HashSet;
use std::iter;
use std::path::PathBuf;

pub struct Export {
    kick: Option<String>,
    snare: Option<String>,
    hat: Option<String>,
    crash: Option<String>,
    ride: Option<String>,
    tom: Option<String>,
    // have user input decide if Some or None based on instruments included in args
}

pub fn pick(path: PathBuf) {//-> Export {
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let paths = path.read_dir().expect("Path should never change so it should be fine.");
    
    for dir_res in paths { // only iterate over dir if res == ok
        if let Ok(dir) = dir_res {
            let subfolder = dir.path();

            if !seen.contains(&subfolder) {
                let mut rng = rand::rng();
                println!("fold: {:?}", subfolder.file_name().unwrap());
                
                if let Some(entry) = subfolder.read_dir().expect("Path shouldn't chnage").filter_map(Result::ok).choose(&mut rng) {
                    let res = entry.file_name();
                    println!("chose: {:?}", res);
                }

                seen.insert(subfolder);
            }    
        }
    }
   
}