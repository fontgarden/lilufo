use std::path::Path;
use std::collections::BTreeMap;
use anyhow::Result;
use plist::Value;
use std::fs;

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
                println!("\n{}", group_name);
                println!("{}", "-".repeat(group_name.len()));
                for member in members {
                    if let Value::String(glyph) = member {
                        println!("  {}", glyph);
                    }
                }
            }
        }
    }

    // Sort and display right groups
    println!("\nRight Groups (prefix: public.kern2):");
    for (name, members) in groups.iter() {
        if name.starts_with("public.kern2.") {
            let group_name = name.strip_prefix("public.kern2.").unwrap_or(name);
            if let Value::Array(members) = members {
                println!("\n{}", group_name);
                println!("{}", "-".repeat(group_name.len()));
                for member in members {
                    if let Value::String(glyph) = member {
                        println!("  {}", glyph);
                    }
                }
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
