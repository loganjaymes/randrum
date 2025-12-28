mod merge;

mod args;
use args::Arguments;
use clap::Parser;


fn main() {
    /*
    TODO: 
    4. set up CLI w/ CLAP
    5. allow users to select what drums they want
    6. ??? (67)
    7. Done ! Smile.
    */
    println!("                         __                   ");
    println!("   _________ _____  ____/ /______  ______ ___ ");
    println!("  / ___/ __ `/ __ \\/ __  / ___/ / / / __ `__ \\");
    println!(" / /  / /_/ / / / / /_/ / /  / /_/ / / / / / /");
    println!("/_/   \\__,_/_/ /_/\\__,_/_/   \\__,_/_/ /_/ /_/ ");
    println!("                                              ");

    let args = Arguments::parse();

    if args.list {
        println!("List of valid instruments:");
        println!("    kick\n    snare\n    hihat\n    ride\n    crash\n    toms");
    }

    // if args.generate {
    //     println!("File: {:?}", args.name);
    //     println!("Instr: {:?}", args.instruments);
    // }

    match (args.instruments, args.name) {
        (Some(instruments), Some(name)) => {
            println!("Instruments: {:?}", instruments);
            println!("Name of exported file: {}", name);
        }
        _ => {
            eprintln!("Error: Missing required arguments.");
            std::process::exit(1);
        }
    }

    /*
    let include: Vec<String> = Vec::new(); // instruments to be included
    
    let picked_files = merge::pick_rand("input".into(), include);
    let mut a = merge::hmap_to_struct(picked_files);
    a.export("CLITEST");
    */
}
