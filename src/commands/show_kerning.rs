use std::path::PathBuf;
use anyhow::Result;

/// Executes the ShowKerning command
pub fn execute(ufo_path: &PathBuf) -> Result<()> {
    crate::kerning::display_kerning(ufo_path)
} 