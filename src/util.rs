//! Provides utility functions for use by the rest of the program.

// Uses
use std::path::PathBuf;

use home::home_dir;
#[cfg(all(windows, feature = "gui"))]
use winapi::{shared::winerror::ERROR_SUCCESS, um::errhandlingapi::GetLastError};

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
/// Asserts that the last WinAPI call returned [`ERROR_SUCCESS`]. (debug-only)
///
/// This function is thread-local.
///
/// See
/// [here](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/18d8fbe8-a967-4f1c-ae50-99ca8e491d2d)
/// for the error code reference.
pub fn assert_winapi_success() {
	#[cfg(debug_assertions)]
	{
		let last_error = unsafe { GetLastError() };
		assert_eq!(
			ERROR_SUCCESS, last_error,
			"the last WinAPI call failed with error code: {:#010X}",
			last_error
		);
	}
}
