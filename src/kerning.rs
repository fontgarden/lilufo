//! Kerning tools for UFO fonts
//! 
//! Usage:
//! ```bash
//! # Display all kerning groups in a UFO
//! lilufo --ufo-path font.ufo --show-kerning-groups
//! 
//! # Display all kerning pairs in a UFO
//! lilufo --ufo-path font.ufo --show-kerning
//! 
//! # Add a new kerning group
//! lilufo --ufo-path font.ufo \
//!        --add-kerning-group \
//!        --group-name "O_group" \
//!        --group-side "left" \
//!        --group-members "O,Q,C,G"
//!
//! # Edit an existing kerning group (replaces existing members)
//! lilufo --ufo-path font.ufo \
//!        --edit-kerning-group \
//!        --group-name "O_group" \
//!        --group-side "left" \
//!        --group-members "O,Q,C,G,Ø"
//!
//! # Append members to an existing kerning group
//! lilufo --ufo-path font.ufo \
//!        --edit-kerning-group \
//!        --append-members \
//!        --group-name "O_group" \
//!        --group-side "left" \
//!        --group-members "Ø"
//!
//! # Add a new kerning pair (using group names)
//! lilufo --ufo-path font.ufo \
//!        --add-kerning-pair \
//!        --first "@O_group" \
//!        --second "@A_group" \
//!        --value -50
//!
//! # Add a new kerning pair (using glyph names)
//! lilufo --ufo-path font.ufo \
//!        --add-kerning-pair \
//!        --first "T" \
//!        --second "a" \
//!        --value -100
//! ```
//!
//! Note: For both add and edit commands:
//! - group-side must be either "left" or "right"
//! - group-members should be a comma-separated list of glyph names
//! - group-name should not include the "public.kern1." or "public.kern2." prefix
//! - use --append-members with --edit-kerning-group to add to existing members instead of replacing them

use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use anyhow::Result;
use plist::Value;
use std::fs;
use norad::{Font, Name};

pub fn display_kerning_groups(ufo_path: &Path) -> Result<()> {
    let groups_path = ufo_path.join("groups.plist");
    if !groups_path.exists() {
        println!("No groups.plist found in UFO");
        return Ok(());
    }

    let groups_data = fs::read_to_string(groups_path)?;
    let groups: BTreeMap<String, Value> = plist::from_bytes(groups_data.as_bytes())?;

    println!("Kerning Groups:");
    println!("---------------");
    
    // Sort and display left groups
    println!("\nLeft Groups (prefix: public.kern1):");
    for (name, members) in groups.iter() {
        if name.starts_with("public.kern1.") {
            let group_name = name.strip_prefix("public.kern1.").unwrap_or(name);
            if let Value::Array(members) = members {
                let members_str: Vec<String> = members
                    .iter()
                    .filter_map(|m| {
                        if let Value::String(glyph) = m {
                            Some(glyph.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                println!("@{} → {}", group_name, members_str.join(", "));
            }
        }
    }

    // Sort and display right groups
    println!("\nRight Groups (prefix: public.kern2):");
    for (name, members) in groups.iter() {
        if name.starts_with("public.kern2.") {
            let group_name = name.strip_prefix("public.kern2.").unwrap_or(name);
            if let Value::Array(members) = members {
                let members_str: Vec<String> = members
                    .iter()
                    .filter_map(|m| {
                        if let Value::String(glyph) = m {
                            Some(glyph.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                println!("@{} → {}", group_name, members_str.join(", "));
            }
        }
    }

    Ok(())
}

pub fn display_kerning(ufo_path: &Path) -> Result<()> {
    let kerning_path = ufo_path.join("kerning.plist");
    if !kerning_path.exists() {
        println!("No kerning.plist found in UFO");
        return Ok(());
    }

    let kerning_data = fs::read_to_string(kerning_path)?;
    let kerning: BTreeMap<String, Value> = plist::from_bytes(kerning_data.as_bytes())?;

    println!("Kerning Pairs:");
    println!("--------------");

    for (first, second_dict) in kerning.iter() {
        if let Value::Dictionary(pairs) = second_dict {
            let first_name = if first.starts_with("public.kern1.") {
                format!("@{}", first.strip_prefix("public.kern1.").unwrap_or(first))
            } else {
                first.clone()
            };

            for (second, value) in pairs {
                let second_name = if second.starts_with("public.kern2.") {
                    format!("@{}", second.strip_prefix("public.kern2.").unwrap_or(second))
                } else {
                    second.clone()
                };

                if let Value::Integer(kern_value) = value {
                    println!("{} {} → {}", first_name, second_name, kern_value);
                }
            }
        }
    }

    Ok(())
}

pub fn add_kerning_group(
    ufo_path: &PathBuf,
    group_name: &str,
    group_side: &str,
    members: &[String]
) -> Result<()> {
    let mut font = Font::load(ufo_path)?;
    
    // Validate group side
    if group_side != "left" && group_side != "right" {
        return Err(anyhow::anyhow!("group_side must be either 'left' or 'right'"));
    }

    // Directly borrow the groups as mutable
    let groups = &mut font.groups;
    
    // Add the new group
    let prefix = if group_side == "left" { "public.kern1." } else { "public.kern2." };
    let full_group_name = format!("{}{}", prefix, group_name);
    
    // Convert Vec<String> to Vec<Name>
    let name_members: Vec<norad::Name> = members
        .iter()
        .map(|s| norad::Name::new(s))
        .collect::<Result<Vec<_>, _>>()?;
    
    groups.insert(Name::new(&full_group_name)?, name_members);
    
    // Save the changes back to the UFO file
    font.save(ufo_path)?;
    
    println!("Successfully added kerning group '{}'", group_name);
    Ok(())
}

pub fn edit_kerning_group(
    ufo_path: &PathBuf,
    group_name: &str,
    group_side: &str,
    members: &[String],
    append: bool
) -> Result<()> {
    let mut font = Font::load(ufo_path)?;
    
    // Validate group side
    if group_side != "left" && group_side != "right" {
        return Err(anyhow::anyhow!("group_side must be either 'left' or 'right'"));
    }

    // Get the full group name with prefix
    let prefix = if group_side == "left" { "public.kern1." } else { "public.kern2." };
    let full_group_name = format!("{}{}", prefix, group_name);
    let full_name = Name::new(&full_group_name)?;
    
    // Check if the group exists
    if !font.groups.contains_key(&full_name) {
        return Err(anyhow::anyhow!("Kerning group '{}' does not exist", group_name));
    }
    
    // Convert new members to Vec<Name>
    let mut name_members: Vec<norad::Name> = members
        .iter()
        .map(|s| norad::Name::new(s))
        .collect::<Result<Vec<_>, _>>()?;
    
    // If appending, combine with existing members
    if append {
        if let Some(existing_members) = font.groups.get(&full_name) {
            let mut combined_members = existing_members.clone();
            combined_members.extend(name_members);
            // Deduplicate members
            combined_members.sort();
            combined_members.dedup();
            name_members = combined_members;
        }
    }
    
    // Update the group
    font.groups.insert(full_name, name_members);
    
    // Save the changes back to the UFO file
    font.save(ufo_path)?;
    
    println!("Successfully updated kerning group '{}'", group_name);
    Ok(())
}

pub fn add_kerning_pair(
    ufo_path: &PathBuf,
    first: &str,
    second: &str,
    value: i32
) -> Result<()> {
    let mut font = Font::load(ufo_path)?;
    
    // Validate that if groups are used, they exist and have correct prefixes
    if first.starts_with("@") {
        let group_name = format!("public.kern1.{}", &first[1..]);
        if !font.groups.contains_key(&Name::new(&group_name)?) {
            return Err(anyhow::anyhow!("First group '{}' does not exist", first));
        }
    }
    
    if second.starts_with("@") {
        let group_name = format!("public.kern2.{}", &second[1..]);
        if !font.groups.contains_key(&Name::new(&group_name)?) {
            return Err(anyhow::anyhow!("Second group '{}' does not exist", second));
        }
    }
    
    // Convert @GroupName notation to public.kern*.GroupName
    let first_key = if first.starts_with("@") {
        format!("public.kern1.{}", &first[1..])
    } else {
        first.to_string()
    };
    
    let second_key = if second.starts_with("@") {
        format!("public.kern2.{}", &second[1..])
    } else {
        second.to_string()
    };
    
    // Add or update the kerning pair
    if let Some(first_dict) = font.kerning.get_mut(&Name::new(&first_key)?) {
        first_dict.insert(Name::new(&second_key)?, value as f64);
    } else {
        let mut first_dict = BTreeMap::new();
        first_dict.insert(Name::new(&second_key)?, value as f64);
        font.kerning.insert(Name::new(&first_key)?, first_dict);
    }
    
    // Save the changes back to the UFO file
    font.save(ufo_path)?;
    
    println!("Successfully added kerning pair '{}' '{}' → {}", first, second, value);
    Ok(())
}
