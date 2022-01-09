//! The main body of the application.

// Uses
#[cfg(not(windows))]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::{symlink_dir, symlink_file};
use std::{
	collections::{HashMap, VecDeque},
	fs::{
		read_to_string as read_file_to_string,
		remove_dir,
		remove_file,
		symlink_metadata,
		write as write_string_to_file,
	},
	path::Path,
};

use anyhow::{Context, Error, Result};
use serde::Deserialize;
use toml::from_str as from_toml_str;

use crate::constants::STARTER_CONFIG_CONTENTS;

// Type Definitions
pub type LoadoutName = String;
pub type FileTarget = String;
pub type FilePath = String;

/// The loadouts config.
#[derive(Clone, Debug, Deserialize)]
pub struct LoadoutsConfig {
	pub targets: HashMap<FileTarget, FilePath>,
	pub loadouts: Vec<Loadout>,
}

/// A loadout, defining destination files for each target.
#[derive(Clone, Debug, Deserialize)]
pub struct Loadout {
	pub name: LoadoutName,
	pub parent: Option<LoadoutName>,
	pub files: HashMap<FileTarget, FilePath>,
}

/// Load the [`LoadoutsConfig`] from the [`Path`] `config_path`.
pub fn load_config(config_path: &Path) -> Result<LoadoutsConfig> {
	let file_contents = match read_file_to_string(config_path) {
		Ok(contents) => contents,
		Err(error) => {
			return Err(error).with_context(|| {
				format!(
					"unable to read loadouts config file \"{}\"",
					config_path.display()
				)
			});
		}
	};

	// Deserialize the config
	from_toml_str::<LoadoutsConfig>(file_contents.as_str()).with_context(|| {
		format!(
			"unable to deserialize the loadouts config file \"{}\"",
			config_path.display()
		)
	})
}

/// Create a starter config at the [`Path`] `config_path`.
pub fn create_starter_config(config_path: &Path) -> Result<()> {
	write_string_to_file(config_path, STARTER_CONFIG_CONTENTS).with_context(|| {
		format!(
			"unable to write the starter config file \"{}\"",
			config_path.display()
		)
	})
}

/// Loads a loadout, managing the symlinks as necessary.
pub fn load_loadout(config: &LoadoutsConfig, loadout_name: &str) -> Result<()> {
	// Build a chain of loadouts
	// This could be sped up, but it's not likely to ever get enough significant use
	// to be worth the changes necessary
	let mut loadout_chain = VecDeque::with_capacity(1);
	let mut search_name = loadout_name;
	'chain_builder: loop {
		'name_search: for loadout in &config.loadouts {
			if loadout.name.as_str() == search_name {
				loadout_chain.push_back(loadout);
				if let Some(parent_name) = &loadout.parent {
					search_name = parent_name.as_str();
					break 'name_search;
				}
				break 'chain_builder;
			}
		}
	}

	// Follow the parental chain, and load the set of file targets to replace
	let mut file_mappings = HashMap::new();
	while let Some(loadout) = loadout_chain.pop_back() {
		for mapping in &loadout.files {
			file_mappings.insert(mapping.0, mapping.1);
		}
	}

	// Load all the file links
	for (target_name, source) in file_mappings {
		// Get the target path based on the identifier provided
		let target = config.targets.get(target_name).ok_or_else(|| {
			Error::msg(format!(
				"target with identifier \"{}\" could not be found",
				target_name
			))
		})?;

		// Get the source path metadata and error if it does not exist
		let source_metadata = symlink_metadata(source)
			.with_context(|| format!("source path \"{}\" does not exist", source))?;

		// Remove the existing target symlink unless it's an actual file
		// We don't want to accidentally delete a user's real file
		if let Ok(file_type) = symlink_metadata(target).map(|m| m.file_type()) {
			if file_type.is_symlink() {
				// Windows directory symlinks must be removed as directories
				#[cfg(windows)]
				{
					remove_file(target)
						.or_else(|_| remove_dir(target))
						.with_context(|| {
							format!(
								"unable to remove the symlink \"{}\" as both a file and a \
								 directory",
								target
							)
						})?;
				}
				#[cfg(not(windows))]
				{
					remove_file(target).with_context(|| {
						Error::msg(format!("unable to remove the symlink \"{}\"", target))
					})?;
				}
			} else {
				return Err(Error::msg(format!(
					"target path \"{}\" exists already and is not a symbolic link",
					target
				)));
			}
		}

		// Create a new symlink for the target
		#[cfg(windows)]
		{
			if source_metadata.is_dir() {
				symlink_dir(source, target).with_context(|| {
					Error::msg(format!(
						"unable to create a symbolic link (directory) at \"{}\" pointing to \"{}\"",
						target, source
					))
				})?;
			} else {
				symlink_file(source, target).with_context(|| {
					Error::msg(format!(
						"unable to create a symbolic link (file) at \"{}\" pointing to \"{}\"",
						target, source
					))
				})?;
			}
		}
		#[cfg(not(windows))]
		{
			symlink(source, target).with_context(|| {
				Error::msg(format!(
					"unable to create a symbolic link at \"{}\" pointing to \"{}\"",
					target, source
				))
			})?;
		}
	}

	Ok(())
}
