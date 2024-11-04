// main.rs
use std::path::PathBuf;        // For handling file paths
use clap::Parser;              // For parsing command-line arguments
use norad::Font;               // UFO font handling library
use anyhow::Result;            // For error handling

mod basic;                     // Imports a separate module named "basic" (defined in basic.rs)

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // Defines command-line options:
    
    /// Path to the UFO file
    #[arg(short, long)]
    ufo_path: Option<PathBuf>,

    /// Display basic font information
    #[arg(long)]
    basic_info: bool,

    /// Round all points to nearest even integer
    #[arg(long)]
    round_to_even: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse(); // Parses the command-line arguments into a Cli struct
    
    if cli.ufo_path.is_none() && !cli.basic_info && !cli.round_to_even {
        println!("    .     *     .           .     ");
        println!("   .-----.                        ");
        println!(" _/___@_@_\\_              .      ");
        println!("(___________)      *              ");
        println!("                                  ");
        println!("Lil' UFO - UFO Font File Tool v{}", env!("CARGO_PKG_VERSION"));
        println!("");
        println!("Usage: lilufo --ufo-path <path-to-ufo-file> [OPTIONS]");
        println!("");
        println!("For more information about available options, run:");
        println!("  lilufo --help");
        return Ok(());
    }

    let ufo_path = cli.ufo_path.ok_or_else(|| anyhow::anyhow!("UFO path is required"))?;
    
    // Load the UFO file
    let font = Font::load(&ufo_path)?;

    if cli.basic_info {
        basic::display_basic_info(&font);
    } else if cli.round_to_even {
        basic::round_points_to_even(&ufo_path)?;
    } else {
        println!("UFO file loaded. Use --basic-info to see basic font information or --round-to-even to round all points.");
    }

    Ok(())
}
