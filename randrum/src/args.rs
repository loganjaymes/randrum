use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "RANDRUM")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool that randomly merges individual drumhead MIDI files into full drum grooves.")]
pub struct Arguments {
    // Flags (ie. -g for generate)
    #[arg(short, long)]
    /// List all valid instruments
    pub list: bool,

    #[arg(short, long)]
    pub generate: bool,

    #[arg(short, long, required_if_eq("generate","true"), num_args = 1..)]
    /// Instruments to use in the final export
    pub instruments: Option<Vec<String>>, 

    #[arg(short, long, required_if_eq("generate","true"))]
    /// Name of exported MIDI file
    pub name: Option<String>,

    /* later add -X flags and etc. for time sigs
    pub second: String,
    pub third: String,
    */
}