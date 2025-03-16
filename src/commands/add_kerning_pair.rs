use std::path::PathBuf;
use anyhow::Result;

/// Executes the AddKerningPair command
pub fn execute(ufo_path: &PathBuf, first: &str, second: &str, value: i32) -> Result<()> {
    crate::kerning::add_kerning_pair(ufo_path, first, second, value)
} 