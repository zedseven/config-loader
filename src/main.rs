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
use std::path::PathBuf;

use anyhow::{Error, Result};
use home::home_dir;

use crate::{app::loadout_loop, cli::parse_cli_arguments};

// Constants
const DEFAULT_MASTER_FILE: &str = "config-master.toml";

// Entry Point
fn main() -> Result<()> {
	let matches = parse_cli_arguments();
	let master_file = matches
		.value_of("master")
		.map(PathBuf::from)
		.or_else(get_default_master_file)
		.ok_or_else(|| Error::msg("unable to get a value for the master config file"))?;
	let fuzzy_search = matches.is_present("fuzzy");

	println!("Using master config file: \"{}\"", master_file.display());

	loadout_loop(master_file.as_path(), fuzzy_search)
}

fn get_default_master_file() -> Option<PathBuf> {
	home_dir().map(|dir| dir.join(DEFAULT_MASTER_FILE))
}
