use std::path::PathBuf;
use anyhow::Result;

/// Executes the ShowKerningGroups command
pub fn execute(ufo_path: &PathBuf) -> Result<()> {
    crate::kerning::display_kerning_groups(ufo_path)
} 