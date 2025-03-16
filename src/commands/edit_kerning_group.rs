use std::path::PathBuf;
use anyhow::Result;

/// Executes the EditKerningGroup command
pub fn execute(ufo_path: &PathBuf, name: &str, side: &str, members: &[String], append: bool) -> Result<()> {
    crate::kerning::edit_kerning_group(ufo_path, name, side, members, append)
} 