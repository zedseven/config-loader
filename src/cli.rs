//! Implements the program functionality in CLI mode.

// Uses
use std::{
	env::var,
	io::stdin,
	path::{Path, PathBuf},
};

use anyhow::{Context, Error, Result};
use clap::{App, Arg, ArgMatches};
use lazy_static::lazy_static;
use yansi::{Color, Paint, Style};

use crate::{
	app::{create_starter_config, load_config, load_loadout},
	constants::{LOADOUTS_CONFIG_PATH_VAR, PROGRAM_AUTHOURS, PROGRAM_VERSION, PROJECT_URL},
	util::{get_default_config_path, is_newline},
};

// Constants
lazy_static! {
	static ref MESSAGE_STYLE: Style = Style::new(Color::Cyan).wrap();
	static ref HEADER_STYLE: Style = Style::new(Color::Yellow).bold();
	static ref RESULT_STYLE: Style = Style::new(Color::Green).bold();
	static ref ERROR_STYLE: Style = Style::new(Color::Red).bold();
	static ref INPUT_STYLE: Style = Style::default().italic().dimmed();
	static ref VALUE_STYLE: Style = Style::default().underline();
}

/// Start up the tool for CLI operation.
pub fn start() -> Result<()> {
	let matches = parse_cli_arguments();
	let config_file = matches
		.value_of("loadouts")
		.map(PathBuf::from)
		.or_else(|| var(LOADOUTS_CONFIG_PATH_VAR).ok().map(PathBuf::from))
		.or_else(get_default_config_path)
		.ok_or_else(|| Error::msg("unable to get a value for the loadouts config file"))?;
	let colour = matches
		.value_of("colour")
		.expect("clap has betrayed us")
		.to_lowercase();

	if colour.as_str() == "never" || (colour.as_str() == "auto" && !Paint::enable_windows_ascii()) {
		Paint::disable();
	}

	println!(
		"{} \"{}\"",
		RESULT_STYLE.paint(format!(
			"Config Loader v{}, using loadouts config file:",
			env!("CARGO_PKG_VERSION")
		)),
		VALUE_STYLE.paint(config_file.display())
	);

	loadout_loop(config_file.as_path())
}

/// The main loop of the application. On each loop it reads the config, provides
/// the user with options, then awaits the user's decision and acts upon it.
pub fn loadout_loop(config_path: &Path) -> Result<()> {
	let mut previous_selection = None;
	let mut input_buffer = String::new();
	let stdin = stdin();
	loop {
		// Load the config or prompt the user to make a starter one if it doesn't exist
		let loadouts_config = match load_config(config_path) {
			Ok(config) => config,
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
					create_starter_config(config_path)?;
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
					println!(
						"{} {}",
						RESULT_STYLE.paint("Loaded:"),
						&loadouts_config.loadouts[i].name
					);
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
						println!("{} {}", RESULT_STYLE.paint("Loaded:"), loadout_name);
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

/// Defines the CLI arguments and parses user input.
pub fn parse_cli_arguments() -> ArgMatches {
	App::new("Config Loader")
		.version(PROGRAM_VERSION)
		.author(PROGRAM_AUTHOURS)
		.about(format!("{}\n\n{}", PROJECT_URL, env!("CARGO_PKG_DESCRIPTION")).as_str())
		.arg(
			Arg::new("loadouts")
				.short('l')
				.long("loadouts")
				.alias("config")
				.takes_value(true)
				.value_name("PATH")
				.long_help(
					format!(
						"The location of the loadouts config file to use (if not present, it uses \
						 the value of the environment variable \"{}\", and if that's not present \
						 it uses the user home directory)",
						LOADOUTS_CONFIG_PATH_VAR
					)
					.as_str(),
				),
		)
		.arg(
			Arg::new("colour")
				.short('c')
				.long("colour")
				.alias("color")
				.takes_value(true)
				.value_name("WHEN")
				.default_value("auto")
				.validator(|v| match v.to_lowercase().as_str() {
					"auto" | "always" | "never" => Ok(()),
					_ => Err(Error::msg("must be `auto`, `always`, or `never`")),
				})
				.help("Colouring: auto, always, never"),
		)
		.get_matches()
}
