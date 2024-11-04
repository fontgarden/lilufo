// main.rs
use std::path::PathBuf;        // For handling file paths
use clap::Parser;              // For parsing command-line arguments
use norad::Font;               // UFO font handling library
use anyhow::Result;            // For error handling

mod basic;                     // Imports a separate module named "basic" (defined in basic.rs)
mod kerning;

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

    /// Display kerning groups
    #[arg(long)]
    show_kerning_groups: bool,

    /// Display kerning pairs
    #[arg(long)]
    show_kerning: bool,

    /// Add a new kerning group
    #[arg(long)]
    add_kerning_group: bool,

    /// Name for the new kerning group
    #[arg(long)]
    group_name: Option<String>,

    /// Side for the kerning group (left or right)
    #[arg(long)]
    group_side: Option<String>,

    /// Members of the kerning group (comma-separated)
    #[arg(long)]
    group_members: Option<String>,

    /// Edit an existing kerning group
    #[arg(long)]
    edit_kerning_group: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse(); // Parses the command-line arguments into a Cli struct
    
    if cli.ufo_path.is_none() && !cli.basic_info && !cli.round_to_even && !cli.show_kerning_groups && !cli.show_kerning {
        println!("    .     *     .           .     ");
        println!("   .-----.                        ");
        println!(" _/___@_@_\\_              .      ");
        println!("(___________)      *              ");
        println!("                                  ");
        println!("Lil' UFO - UFO Font File Tool v{}", env!("CARGO_PKG_VERSION"));
        //println!("");
        println!("Usage: lilufo --ufo-path <path-to-ufo-file> [OPTIONS]");
        //println!("");
        println!("For more information about available options, run: lilufo --help");
        //println!("  lilufo --help");
        return Ok(());
    }

    let ufo_path = cli.ufo_path.ok_or_else(|| anyhow::anyhow!("UFO path is required"))?;
    
    // Load the UFO file
    let font = Font::load(&ufo_path)?;

    if cli.basic_info {
        basic::display_basic_info(&font);
    } else if cli.round_to_even {
        basic::round_points_to_even(&ufo_path)?;
    } else if cli.show_kerning_groups {
        kerning::display_kerning_groups(&ufo_path)?;
    } else if cli.show_kerning {
        kerning::display_kerning(&ufo_path)?;
    } else if cli.add_kerning_group {
        // Validate required parameters
        let group_name = cli.group_name
            .ok_or_else(|| anyhow::anyhow!("--group-name is required for adding a kerning group"))?;
        let group_side = cli.group_side
            .ok_or_else(|| anyhow::anyhow!("--group-side is required for adding a kerning group"))?;
        let members = cli.group_members
            .ok_or_else(|| anyhow::anyhow!("--group-members is required for adding a kerning group"))?;
        
        // Split comma-separated members into a vector
        let members: Vec<String> = members.split(',')
            .map(|s| s.trim().to_string())
            .collect();
        
        kerning::add_kerning_group(&ufo_path, &group_name, &group_side, &members)?;
    } else if cli.edit_kerning_group {
        // Validate required parameters
        let group_name = cli.group_name
            .ok_or_else(|| anyhow::anyhow!("--group-name is required for editing a kerning group"))?;
        let group_side = cli.group_side
            .ok_or_else(|| anyhow::anyhow!("--group-side is required for editing a kerning group"))?;
        let members = cli.group_members
            .ok_or_else(|| anyhow::anyhow!("--group-members is required for editing a kerning group"))?;
        
        // Split comma-separated members into a vector
        let members: Vec<String> = members.split(',')
            .map(|s| s.trim().to_string())
            .collect();
        
        kerning::edit_kerning_group(&ufo_path, &group_name, &group_side, &members)?;
    } else {
        println!("UFO file loaded. Use --basic-info to see basic font information, --round-to-even to round all points, --show-kerning-groups to display kerning groups, or --show-kerning to display kerning pairs.");
    }

    Ok(())
}
