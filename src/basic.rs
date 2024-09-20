// basic.rs
use norad::Font;
use std::path::Path;
use xmltree::Element;
use glob::glob;
use std::fs;
use anyhow::{Result, anyhow};
use std::io::Cursor;

pub fn display_basic_info(font: &Font) {
    println!("    .     *     .           .     ");
    println!("   .-----.                        ");
    println!(" _/___@_@_\\_              .      ");
    println!("(___________)      *              ");
    println!("                                  ");
    println!("Lil' UFO");
    println!("");
    println!("Font Information:");
    println!("Family Name: {}", font.font_info.family_name.as_deref().unwrap_or("N/A"));
    println!("Style Name: {}", font.font_info.style_name.as_deref().unwrap_or("N/A"));
    println!("Version Major: {}", font.font_info.version_major.unwrap_or(0));
    println!("Version Minor: {}", font.font_info.version_minor.unwrap_or(0));
    println!("Number of Glyphs: {}", font.iter_layers().next().map(|l| l.len()).unwrap_or(0));
}

pub fn round_points_to_even(ufo_path: &Path) -> Result<()> {
    let glif_pattern = ufo_path.join("glyphs").join("*.glif");
    let glif_pattern = glif_pattern.to_str().ok_or(anyhow!("Invalid UFO path"))?;

    for entry in glob(glif_pattern)? {
        let path = entry?;
        let mut xml = Element::parse(fs::read_to_string(&path)?.as_bytes())?;

        round_element_points(&mut xml);

        let mut writer = Cursor::new(Vec::new());
        xml.write_with_config(&mut writer, xmltree::EmitterConfig::new().perform_indent(true))?;
        let modified_xml = String::from_utf8(writer.into_inner())?;

        fs::write(&path, modified_xml)?;

        if verify_even_points(&xml) {
            println!("{}: All points rounded to even integers", path.display());
        } else {
            println!("{}: Warning - Not all points are even integers", path.display());
        }
    }

    Ok(())
}

fn round_element_points(element: &mut Element) {
    if element.name == "point" {
        if let Some(x) = element.attributes.get_mut("x") {
            *x = round_to_even(x);
        }
        if let Some(y) = element.attributes.get_mut("y") {
            *y = round_to_even(y);
        }
    }

    for child in &mut element.children {
        if let Some(child) = child.as_mut_element() {
            round_element_points(child);
        }
    }
}

fn round_to_even(value: &str) -> String {
    let parsed: f64 = value.parse().unwrap_or(0.0);
    let rounded = (parsed / 2.0).round() * 2.0;
    rounded.to_string()
}

fn verify_even_points(element: &Element) -> bool {
    if element.name == "point" {
        if let (Some(x), Some(y)) = (element.attributes.get("x"), element.attributes.get("y")) {
            if !is_even(x) || !is_even(y) {
                return false;
            }
        }
    }

    for child in &element.children {
        if let Some(child) = child.as_element() {
            if !verify_even_points(child) {
                return false;
            }
        }
    }

    true
}

fn is_even(value: &str) -> bool {
    value.parse::<f64>().map(|v| v.fract() == 0.0 && v as i64 % 2 == 0).unwrap_or(false)
}
