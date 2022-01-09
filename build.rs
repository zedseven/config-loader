// Enforce selected features
#[cfg(not(any(feature = "cli", feature = "gui")))]
compile_error!(
	"Specify whether to build the tool as a `cli` or `gui` application, using: \
	 `--features=\"cli|gui\"`"
);

#[cfg(all(feature = "cli", feature = "gui"))]
compile_error!("The tool cannot be built for both `cli` and `gui` at the same time.");

/// Unused `main` function.
fn main() {}
