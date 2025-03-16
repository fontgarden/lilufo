// main.rs
use std::path::PathBuf;        // For handling file paths
use clap::{Parser, Subcommand}; // For parsing command-line arguments
use norad::Font;               // UFO font handling library
use anyhow::Result;            // For error handling

mod basic;                     // Imports a separate module named "basic" (defined in basic.rs)
mod kerning;
mod commands;                  // Import our new commands module

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the UFO file
    #[arg(short, long)]
    ufo_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Display basic font information
    BasicInfo {},
    
    /// Round all points to nearest even integer
    RoundToEven {},
    
    /// Display kerning groups
    ShowKerningGroups {},
    
    /// Display kerning pairs
    ShowKerning {},
    
    /// Add a new kerning group
    AddKerningGroup {
        /// Name for the new kerning group
        #[arg(long)]
        name: String,
        
        /// Side for the kerning group (left or right)
        #[arg(long)]
        side: String,
        
        /// Members of the kerning group (comma-separated)
        #[arg(long)]
        members: String,
    },
    
    /// Edit an existing kerning group
    EditKerningGroup {
        /// Name for the kerning group
        #[arg(long)]
        name: String,
        
        /// Side for the kerning group (left or right)
        #[arg(long)]
        side: String,
        
        /// Members of the kerning group (comma-separated)
        #[arg(long)]
        members: String,
        
        /// Append members to existing group instead of replacing them
        #[arg(long)]
        append: bool,
    },
    
    /// Add a new kerning pair
    AddKerningPair {
        /// First member of kerning pair (glyph or group name)
        #[arg(long)]
        first: String,
        
        /// Second member of kerning pair (glyph or group name)
        #[arg(long)]
        second: String,
        
        /// Kerning value (integer)
        #[arg(long)]
        value: i32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Display the ASCII art and help message if no arguments are provided
    if cli.ufo_path.is_none() && cli.command.is_none() {
        println!("    .     *     .           .     ");
        println!("   .-----.                        ");
        println!(" _/___@_@_\\_              .      ");
        println!("(___________)      *              ");
        println!("                                  ");
        println!("Lil' UFO - UFO Font File Tool v{}", env!("CARGO_PKG_VERSION"));
        println!("Usage: lilufo --ufo-path <path-to-ufo-file> [COMMAND]");
        println!("For more information about available options, run: lilufo --help");
        return Ok(());
    }
    
    // If we have a command, we need a UFO path
    if cli.command.is_some() {
        let ufo_path = cli.ufo_path.ok_or_else(|| anyhow::anyhow!("UFO path is required when using commands"))?;
        
        // Load the UFO file for commands that need it
        let font = Font::load(&ufo_path)?;
        
        match &cli.command {
            Some(Commands::BasicInfo {}) => {
                commands::execute_basic_info(&font);
            }
            Some(Commands::RoundToEven {}) => {
                commands::execute_round_to_even(&ufo_path)?;
            }
            Some(Commands::ShowKerningGroups {}) => {
                commands::execute_show_kerning_groups(&ufo_path)?;
            }
            Some(Commands::ShowKerning {}) => {
                commands::execute_show_kerning(&ufo_path)?;
            }
            Some(Commands::AddKerningGroup { name, side, members }) => {
                // Split comma-separated members into a vector
                let members_vec: Vec<String> = members.split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                
                commands::execute_add_kerning_group(&ufo_path, name, side, &members_vec)?;
            }
            Some(Commands::EditKerningGroup { name, side, members, append }) => {
                // Split comma-separated members into a vector
                let members_vec: Vec<String> = members.split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                
                commands::execute_edit_kerning_group(&ufo_path, name, side, &members_vec, *append)?;
            }
            Some(Commands::AddKerningPair { first, second, value }) => {
                commands::execute_add_kerning_pair(&ufo_path, first, second, *value)?;
            }
            None => unreachable!(), // We already checked this above
        }
    } else if let Some(ufo_path) = &cli.ufo_path {
        // If we have a UFO path but no command, just load the font and show a message
        let _font = Font::load(ufo_path)?;
        
        println!("UFO file loaded. Use a subcommand to perform operations.");
        println!("For more information about available commands, run: lilufo --help");
    }

    Ok(())
}
