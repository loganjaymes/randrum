use::walkdir::WalkDir;
use::rand::seq::IteratorRandom;

pub struct Export {
    kick: Option<String>,
    snare: Option<String>,
    hat: Option<String>,
    // crash: Option<String>,
    // have user input decide if Some or None based on instruments included in args
}

pub fn run(path: &str) {//-> Export {
    for entry in WalkDir::new(path) {
        println!("{}", entry.unwrap().path().display());
    }
}