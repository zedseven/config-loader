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
	io::stdin,
	path::Path,
};

use anyhow::{Context, Error, Result};
use serde::Deserialize;
use toml::from_str as from_toml_str;
use yansi::Paint;

use super::{HEADER_STYLE, INPUT_STYLE, MESSAGE_STYLE, RESULT_STYLE, VALUE_STYLE};
use crate::ERROR_STYLE;

// Constants
#[cfg(windows)]
const STARTER_CONFIG_CONTENTS: &str =
	include_str!("../static/starter-loadouts-config-windows.toml");
#[cfg(not(windows))]
const STARTER_CONFIG_CONTENTS: &str = include_str!("../static/starter-loadouts-config-unix.toml");

// Type Definitions
type LoadoutName = String;
type FileTarget = String;
type FilePath = String;

/// The loadouts config.
#[derive(Debug, Deserialize)]
struct LoadoutsConfig {
	targets: HashMap<FileTarget, FilePath>,
	loadouts: Vec<Loadout>,
}

/// A loadout, defining destination files for each target.
#[derive(Debug, Deserialize)]
struct Loadout {
	name: LoadoutName,
	parent: Option<LoadoutName>,
	files: HashMap<FileTarget, FilePath>,
}

/// The main loop of the application. On each loop it reads the config, provides
/// the user with options, then awaits the user's decision and acts upon it.
pub fn loadout_loop(config_path: &Path) -> Result<()> {
	let mut previous_selection = None;
	let mut input_buffer = String::new();
	let stdin = stdin();
	loop {
		// Read the config contents or prompt the user to make a starter one if it
		// doesn't exist
		let file_contents = match read_file_to_string(config_path) {
			Ok(contents) => contents,
			Err(error) => {
				println!(
					"{} {}",
					MESSAGE_STYLE.paint(
						"Unable to read the loadouts config file. Would you like a starter one to \
						 be created?"
					),
					INPUT_STYLE.paint("(y/n)")
				);

				input_buffer.clear();
				stdin
					.read_line(&mut input_buffer)
					.with_context(|| "failed to get user input successfully")?;
				input_buffer = input_buffer.to_lowercase();
				let user_input = input_buffer.trim_start();

				if user_input.starts_with('y') {
					write_string_to_file(config_path, STARTER_CONFIG_CONTENTS).with_context(
						|| {
							format!(
								"unable to write the starter config file \"{}\"",
								config_path.display()
							)
						},
					)?;
					println!(
						"{}",
						MESSAGE_STYLE.paint(format!(
							"A starter config file has been created at \"{}\". You will have to \
							 edit it to add your loadouts before you can use this tool.",
							VALUE_STYLE.paint(config_path.display())
						))
					);
					continue;
				}

				return Err(error).with_context(|| {
					format!(
						"unable to read loadouts config file \"{}\" and the user declined to make \
						 a starter copy",
						config_path.display()
					)
				});
			}
		};

		// Deserialize the config
		let loadouts_config = from_toml_str::<LoadoutsConfig>(file_contents.as_str())
			.with_context(|| "unable to deserialize the loadouts config file")?;

		// Calculate the width to pad entries to so they remain lined up
		let number_width = (loadouts_config.loadouts.len() - 1).log10() as usize + 1;

		// Give the user their options
		if previous_selection.is_none() {
			println!("{}", HEADER_STYLE.paint("Actions:"));
			println!(
				"\t{} Refresh config",
				INPUT_STYLE.paint(format!("{:>width$}.", "R", width = number_width))
			);
			println!(
				"\t{} Exit",
				INPUT_STYLE.paint(format!(
					"{:>width$}.",
					if number_width >= 5 {
						"E/Q/X"
					} else if number_width >= 3 {
						"E/Q"
					} else {
						"E"
					},
					width = number_width
				))
			);
			println!(
				"{} (type the index number or the start of the name)",
				HEADER_STYLE.paint("Loadouts:")
			);
		} else {
			println!("{}", HEADER_STYLE.paint("Loadouts:"));
		}
		for (index, loadout) in loadouts_config.loadouts.iter().enumerate() {
			let matches_previous_selection = if let Some(previous) = &previous_selection {
				loadout.name.eq(previous)
			} else {
				false
			};
			println!(
				"\t{} {}",
				INPUT_STYLE.paint(format!("{:>width$}.", index, width = number_width)),
				if matches_previous_selection {
					Paint::new(&loadout.name).bold()
				} else {
					Paint::new(&loadout.name)
				}
			);
		}

		// Get the user's choice
		input_buffer.clear();
		stdin
			.read_line(&mut input_buffer)
			.with_context(|| "failed to get user input successfully")?;
		input_buffer = input_buffer.to_lowercase();
		let user_input = input_buffer.trim_end_matches(is_newline).trim();

		// Process the choice
		if user_input.is_empty() {
			continue;
		}

		match user_input.parse::<usize>() {
			Ok(i) => {
				if i < loadouts_config.loadouts.len() {
					previous_selection = Some(loadouts_config.loadouts[i].name.clone());
					load_loadout(&loadouts_config, loadouts_config.loadouts[i].name.as_str())?;
				} else {
					println!(
						"{}",
						ERROR_STYLE.paint("Unrecognized command. Please try again.")
					);
				}
			}
			Err(_) => match user_input {
				"r" => continue,
				"e" | "q" | "x" => break,
				input => {
					let mut found_loadout_name = None;
					for loadout in &loadouts_config.loadouts {
						let loadout_name_prepared = loadout.name.to_lowercase();
						if loadout_name_prepared.starts_with(input) {
							found_loadout_name = Some(loadout.name.as_str());
							break;
						}
					}
					if let Some(loadout_name) = found_loadout_name {
						previous_selection = Some(loadout_name.to_owned());
						load_loadout(&loadouts_config, loadout_name)?;
						continue;
					}

					println!(
						"{}",
						ERROR_STYLE.paint("Unrecognized command. Please try again.")
					);
					continue;
				}
			},
		}
	}

	Ok(())
}

/// Loads a loadout, managing the symlinks as necessary.
fn load_loadout(config: &LoadoutsConfig, loadout_name: &str) -> Result<()> {
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
		if let Ok(symlink_info) = symlink_metadata(target) {
			let file_type = symlink_info.file_type();
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

	println!("{} {}", RESULT_STYLE.paint("Loaded:"), loadout_name);

	Ok(())
}

/// A convenience function for removing newlines and carriage returns from user
/// input. Technically the carriage return isn't a newline character, but since
/// it always comes with a newline character we need to remove it as well.
fn is_newline(c: char) -> bool {
	c == '\n' || c == '\r'
}
