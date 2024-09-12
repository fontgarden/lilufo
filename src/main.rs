// main.rs
use std::path::PathBuf;
use clap::Parser;
use norad::Font;
use anyhow::Result;

mod basic;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the UFO file
    #[arg(short, long)]
    ufo_path: PathBuf,

    /// Display basic font information
    #[arg(long)]
    basic_info: bool,

    /// Round all points to nearest even integer
    #[arg(long)]
    round_to_even: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Load the UFO file
    let font = Font::load(&cli.ufo_path)?;

    if cli.basic_info {
        basic::display_basic_info(&font);
    } else if cli.round_to_even {
        basic::round_points_to_even(&cli.ufo_path)?;
    } else {
        println!("UFO file loaded. Use --basic-info to see basic font information or --round-to-even to round all points.");
    }

    Ok(())
}
