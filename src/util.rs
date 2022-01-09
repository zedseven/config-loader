//! Provides utility functions for use by the rest of the program.

// Uses
use std::path::PathBuf;

use home::home_dir;

use crate::constants::DEFAULT_CONFIG_FILE;

/// Fetches the path for the default loadouts config.
pub fn get_default_config_path() -> Option<PathBuf> {
	home_dir().map(|dir| dir.join(DEFAULT_CONFIG_FILE))
}

/// A convenience function for removing newlines and carriage returns from user
/// input. Technically the carriage return isn't a newline character, but since
/// it always comes with a newline character we need to remove it as well.
pub fn is_newline(c: char) -> bool {
	c == '\n' || c == '\r'
}
