use std::path::PathBuf;
use anyhow::Result;

/// Executes the AddKerningGroup command
pub fn execute(ufo_path: &PathBuf, name: &str, side: &str, members: &[String]) -> Result<()> {
    crate::kerning::add_kerning_group(ufo_path, name, side, members)
} 