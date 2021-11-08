//! Defines and parses the CLI arguments.

// Uses
use anyhow::Error;
use clap::{App, Arg, ArgMatches};

use crate::MASTER_CONFIG_VAR;

// Constants
const PROJECT_URL: &str = "https://github.com/zedseven/config-loader";

/// Defines the CLI arguments and parses user input.
pub fn parse_cli_arguments() -> ArgMatches {
	App::new("Config Loader")
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(format!("{}\n\n{}", PROJECT_URL, env!("CARGO_PKG_DESCRIPTION")).as_str())
		.arg(
			Arg::new("master")
				.short('m')
				.long("master")
				.takes_value(true)
				.value_name("PATH")
				.about(
					format!(
						"The location of the master config file to use (if not present, it uses \
						 the value of the environment variable \"{}\", and if that's not present \
						 it uses the user home directory)",
						MASTER_CONFIG_VAR
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
				.about("Colouring: auto, always, never"),
		)
		.get_matches()
}
