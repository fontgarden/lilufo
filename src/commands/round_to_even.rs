use std::path::PathBuf;
use anyhow::Result;

/// Executes the RoundToEven command
pub fn execute(ufo_path: &PathBuf) -> Result<()> {
    crate::basic::round_points_to_even(ufo_path)
} 