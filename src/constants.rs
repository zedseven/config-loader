//! Defines constants for use by the rest of the program.

// Constants
pub const PROJECT_URL: &str = "https://github.com/zedseven/config-loader";
pub const DEFAULT_CONFIG_FILE: &str = "loadouts-config.toml";
pub const LOADOUTS_CONFIG_PATH_VAR: &str = "LOADOUTS_CONFIG_PATH";

// The starter loadout config, customized based on whether the target is Windows
// (different line-endings, path separators, etc.)
#[cfg(windows)]
pub const STARTER_CONFIG_CONTENTS: &str =
	include_str!("../static/starter-loadouts-config-windows.toml");
#[cfg(not(windows))]
pub const STARTER_CONFIG_CONTENTS: &str =
	include_str!("../static/starter-loadouts-config-unix.toml");
