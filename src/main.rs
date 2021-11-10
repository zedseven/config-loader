//! A tool for quickly switching between different file configurations.

// Linting rules
#![warn(
	clippy::complexity,
	clippy::correctness,
	clippy::perf,
	clippy::style,
	clippy::suspicious,
	clippy::pedantic,
	clippy::filetype_is_file,
	clippy::str_to_string
)]
#![allow(
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::doc_markdown,
	clippy::module_name_repetitions,
	clippy::similar_names,
	clippy::too_many_lines,
	clippy::unnecessary_wraps,
	dead_code,
	unused_macros
)]
#![feature(int_log)]

// Modules
mod app;
mod cli;

// Uses
use std::{env::var, path::PathBuf};

use anyhow::{Error, Result};
use home::home_dir;
use lazy_static::lazy_static;
use yansi::{Color, Paint, Style};

use crate::{app::loadout_loop, cli::parse_cli_arguments};

// Constants
const DEFAULT_MASTER_FILE: &str = "config-master.toml";
const MASTER_CONFIG_VAR: &str = "CONFIG_LOADER_CONFIG";
lazy_static! {
	static ref MESSAGE_STYLE: Style = Style::new(Color::Cyan).wrap();
	static ref HEADER_STYLE: Style = Style::new(Color::Yellow).bold();
	static ref RESULT_STYLE: Style = Style::new(Color::Green).bold();
	static ref INPUT_STYLE: Style = Style::default().italic().dimmed();
	static ref VALUE_STYLE: Style = Style::default().underline();
}

/// Entry Point.
fn main() -> Result<()> {
	let matches = parse_cli_arguments();
	let master_file = matches
		.value_of("master")
		.map(PathBuf::from)
		.or_else(|| var(MASTER_CONFIG_VAR).ok().map(PathBuf::from))
		.or_else(get_default_master_file)
		.ok_or_else(|| Error::msg("unable to get a value for the master config file"))?;
	let colour = matches.value_of("colour").expect("clap has betrayed us");

	if colour == "never" || (colour == "auto" && !Paint::enable_windows_ascii()) {
		Paint::disable();
	}

	println!(
		"{} \"{}\"",
		RESULT_STYLE.paint("Using master config file:"),
		VALUE_STYLE.paint(master_file.display())
	);

	loadout_loop(master_file.as_path())
}

/// Fetches the path for the default master file.
fn get_default_master_file() -> Option<PathBuf> {
	home_dir().map(|dir| dir.join(DEFAULT_MASTER_FILE))
}
