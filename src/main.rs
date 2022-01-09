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
// To make the program not open the console when opened standalone in GUI mode
#![cfg_attr(all(windows, feature = "cli"), windows_subsystem = "console")]
#![cfg_attr(all(windows, feature = "gui"), windows_subsystem = "windows")]

// Modules
mod app;
#[cfg(feature = "cli")]
mod cli;
mod constants;
#[cfg(feature = "gui")]
mod gui;
mod util;

// Uses
use anyhow::Result;

#[cfg(feature = "cli")]
use crate::cli::start;
#[cfg(feature = "gui")]
use crate::gui::start;

/// Entry Point.
fn main() -> Result<()> {
	start()
}
