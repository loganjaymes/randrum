use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "RANDRUM")]
#[command(version = "0.1.0")]
#[command(about = "A TUI tool used to mix-and-match files to create random MIDI drum grooves."
)]
pub struct RanDrumArgs {
    /*
    /// Flags (ie. -g for generate)
    pub options: String,
    */
    /// Instruments to use
    pub instruments: String, // TODO list instruments
    /// Name of MIDI file output
    pub name: String,
    /* later add -X flags and shit for time sigs
    pub second: String,
    pub third: String,
    */
}