// Uses
#[cfg(not(windows))]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_file;
use std::{
	collections::HashMap,
	fs::{read_to_string as read_file_to_string, remove_file, symlink_metadata},
	io::stdin,
	path::Path,
};

use anyhow::{Context, Error, Result};
use serde::{Deserialize, Serialize};
use toml::from_str as from_toml_str;

// Type Definitions
type FileTarget = String;

/// The master config.
#[derive(Debug, Serialize, Deserialize)]
struct MasterConfig {
	targets: HashMap<FileTarget, String>,
	loadouts: Vec<Loadout>,
}

/// A loadout, defining destination files for each target.
#[derive(Debug, Serialize, Deserialize)]
struct Loadout {
	name: String,
	files: HashMap<FileTarget, String>,
}

pub fn loadout_loop(master_file: &Path, fuzzy_search: bool) -> Result<()> {
	let mut input_buffer = String::new();
	let stdin = stdin();
	loop {
		let file_contents = read_file_to_string(master_file).with_context(|| {
			format!(
				"unable to read master config file \"{}\"",
				master_file.display()
			)
		})?;
		let master_config = from_toml_str::<MasterConfig>(file_contents.as_str())
			.with_context(|| "unable to deserialize the master config file")?;

		let number_width = master_config.loadouts.len().log10() as usize + 1;
		println!("Actions:");
		println!(
			"\t{:>width$}. Refresh master config",
			"r",
			width = number_width
		);
		println!("\t{:>width$}. Exit", "x", width = number_width);
		println!("Loadouts:");
		for (index, loadout) in master_config.loadouts.iter().enumerate() {
			println!(
				"\t{:>width$}. {}",
				index,
				loadout.name,
				width = number_width
			);
		}

		input_buffer.clear();
		stdin
			.read_line(&mut input_buffer)
			.with_context(|| "failed to get user input successfully")?;
		input_buffer = input_buffer.to_lowercase();
		let user_input = input_buffer.trim_end_matches(is_newline).trim();

		if user_input.is_empty() {
			continue;
		}

		match user_input.parse::<usize>() {
			Ok(i) => {
				load_loadout(&master_config.targets, &master_config.loadouts[i])?;
			}
			Err(_) => match user_input {
				"r" => continue,
				"x" | "e" => break,
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

fn load_loadout(targets: &HashMap<FileTarget, String>, loadout: &Loadout) -> Result<()> {
	for (target_name, file) in &loadout.files {
		let target = targets.get(target_name).ok_or_else(|| {
			Error::msg(format!(
				"target with identifier \"{}\" could not be found",
				target_name
			))
		})?;

		// Remove the existing target file
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
