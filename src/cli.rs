//! Defines and parses the CLI arguments.

// Uses
use clap::{App, Arg, ArgMatches};

/// Defines the CLI arguments and parses user input.
pub fn parse_cli_arguments() -> ArgMatches {
	App::new("Config Loader")
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(Arg::new("master").short('m').long("master").about(
			"The location of the master config file to use (if not present, it uses the user home \
			 directory)",
		))
		.arg(
			Arg::new("fuzzy")
				.short('f')
				.long("fuzzy")
				.about("Allow fuzzy name searching (find names that start with the search)"),
		)
		.get_matches()
}
