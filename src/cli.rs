//! Defines and parses the CLI arguments.

// Uses
use anyhow::Error;
use clap::{App, Arg, ArgMatches};

use crate::LOADOUTS_CONFIG_PATH_VAR;

// Constants
const PROJECT_URL: &str = "https://github.com/zedseven/config-loader";

/// Defines the CLI arguments and parses user input.
pub fn parse_cli_arguments() -> ArgMatches {
	App::new("Config Loader")
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
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
				.validator(|v| match v {
					"auto" | "always" | "never" => Ok(()),
					_ => Err(Error::msg("must be auto, always, or never")),
				})
				.help("Colouring: auto, always, never"),
		)
		.get_matches()
}
