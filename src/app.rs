//! The main body of the application.

// Uses
#[cfg(not(windows))]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_file;
use std::{
	collections::HashMap,
	fs::{
		read_to_string as read_file_to_string,
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

// Constants
const STARTER_CONFIG_CONTENTS: &str = include_str!("../static/starter-config-master.toml");

// Type Definitions
type FileTarget = String;

/// The master config.
#[derive(Debug, Deserialize)]
struct MasterConfig {
	targets: HashMap<FileTarget, String>,
	loadouts: Vec<Loadout>,
}

/// A loadout, defining destination files for each target.
#[derive(Debug, Deserialize)]
struct Loadout {
	name: String,
	files: HashMap<FileTarget, String>,
}

/// The main loop of the application. On each loop it reads the config, provides
/// the user with options, then awaits the user's decision and acts upon it.
pub fn loadout_loop(master_file: &Path, fuzzy_search: bool) -> Result<()> {
	let mut input_buffer = String::new();
	let stdin = stdin();
	loop {
		// Read the config contents or prompt the user to make a starter one if it
		// doesn't exist
		let file_contents = match read_file_to_string(master_file) {
			Ok(contents) => contents,
			Err(error) => {
				println!(
					"Unable to read the master config file. Would you like a starter one to be \
					 created? (y/n)"
				);

				input_buffer.clear();
				stdin
					.read_line(&mut input_buffer)
					.with_context(|| "failed to get user input successfully")?;
				input_buffer = input_buffer.to_lowercase();
				let user_input = input_buffer.trim_start();

				if user_input.starts_with('y') {
					write_string_to_file(master_file, STARTER_CONFIG_CONTENTS).with_context(
						|| {
							format!(
								"unable to write the starter config file \"{}\"",
								master_file.display()
							)
						},
					)?;
					println!(
						"A starter config file has been created at \"{}\". You will have to edit \
						 it to add your loadouts before you can use this tool.",
						master_file.display()
					);
					continue;
				}

				return Err(error).with_context(|| {
					format!(
						"unable to read master config file \"{}\" and the user declined to make a \
						 starter copy",
						master_file.display()
					)
				});
			}
		};

		// Deserialize the config
		let master_config = from_toml_str::<MasterConfig>(file_contents.as_str())
			.with_context(|| "unable to deserialize the master config file")?;

		// Calculate the width to pad entries to so they remain lined up
		let number_width = master_config.loadouts.len().log10() as usize + 1;

		// Give the user their options
		println!("Actions:");
		println!("\t{:>width$}. Refresh config", "r", width = number_width);
		println!(
			"\t{:>width$}. Exit",
			if number_width >= 3 { "q/x" } else { "q" },
			width = number_width
		);
		println!(
			"Loadouts: (type the index number or the {})",
			if fuzzy_search {
				"start of the name"
			} else {
				"full name"
			}
		);
		for (index, loadout) in master_config.loadouts.iter().enumerate() {
			println!(
				"\t{:>width$}. {}",
				index,
				loadout.name,
				width = number_width
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
				load_loadout(&master_config.targets, &master_config.loadouts[i])?;
			}
			Err(_) => match user_input {
				"r" => continue,
				"q" | "x" => break,
				input => {
					let mut found_loadout = None;
					for loadout in &master_config.loadouts {
						let loadout_name_prepared = loadout.name.to_lowercase();
						if fuzzy_search {
							if loadout_name_prepared.starts_with(input) {
								found_loadout = Some(loadout);
								break;
							}
						} else if loadout_name_prepared.eq(input) {
							found_loadout = Some(loadout);
							break;
						}
					}
					if let Some(loadout) = found_loadout {
						load_loadout(&master_config.targets, loadout)?;
						continue;
					}

					println!("Unrecognized command. Please try again.");
					continue;
				}
			},
		}
	}

	Ok(())
}

/// Loads a loadout, managing the symlinks as necessary.
fn load_loadout(targets: &HashMap<FileTarget, String>, loadout: &Loadout) -> Result<()> {
	for (target_name, file) in &loadout.files {
		// Get the target path based on the identifier provided
		let target = targets.get(target_name).ok_or_else(|| {
			Error::msg(format!(
				"target with identifier \"{}\" could not be found",
				target_name
			))
		})?;

		// Remove the existing target symlink unless it's an actual file
		// We don't want to accidentally delete a user's real file
		if let Ok(symlink_info) = symlink_metadata(target) {
			let file_type = symlink_info.file_type();
			if file_type.is_symlink() {
				remove_file(target).with_context(|| {
					Error::msg(format!("unable to remove the file \"{}\"", target))
				})?;
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
			symlink_file(file, target).with_context(|| {
				Error::msg(format!(
					"unable to create a symbolic link at \"{}\" pointing to \"{}\"",
					target, file
				))
			})?;
		}
		#[cfg(not(windows))]
		{
			symlink(file, target).with_context(|| {
				Error::msg(format!(
					"unable to create a symbolic link at \"{}\" pointing to \"{}\"",
					target, file
				))
			})?;
		}
	}

	println!("Loaded loadout {}.", loadout.name);

	Ok(())
}

/// A convenience function for removing newlines and carriage returns from user
/// input. Technically the carriage return isn't a newline character, but since
/// it always comes with a newline character we need to remove it as well.
fn is_newline(c: char) -> bool {
	c == '\n' || c == '\r'
}
