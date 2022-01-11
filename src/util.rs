//! Provides utility functions for use by the rest of the program.

// Uses
use std::path::PathBuf;

use home::home_dir;
#[cfg(all(windows, feature = "gui"))]
use winapi::um::errhandlingapi::GetLastError;

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

#[cfg(all(windows, feature = "gui"))]
/// Retrieves the error code that was last set for the thread.
///
/// This function is thread-local.
///
/// See
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/18d8fbe8-a967-4f1c-ae50-99ca8e491d2d
/// for the error code reference.
pub fn get_last_winapi_error() -> u32 {
	unsafe { GetLastError() }
}
