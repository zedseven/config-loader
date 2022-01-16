// Enforce selected features
#[cfg(not(any(feature = "cli", feature = "gui")))]
compile_error!(
	"Specify whether to build the tool as a `cli` or `gui` application, using: \
	 `--features=\"cli|gui\"`"
);

#[cfg(all(feature = "cli", feature = "gui"))]
compile_error!("The tool cannot be built for both `cli` and `gui` at the same time.");

// Uses
use std::io::Result;

#[cfg(windows)]
use winres::WindowsResource;

// Constants
const BUILD_ASSETS_DIR: &str = "static";
#[cfg(windows)]
const NEUTRAL_LCID: u16 = 0x0000;

/// Build script that prepares the application.
fn main() -> Result<()> {
	// OS-Specific Executable Packaging
	executable_packaging()?;

	Ok(())
}

/// Sets up executable manifests, icons, etc. OS-Specific.
fn executable_packaging() -> Result<()> {
	#[cfg(windows)]
	{
		WindowsResource::new()
			.set_icon(format!("{}/{}", BUILD_ASSETS_DIR, "icon.ico").as_str())
			.set_manifest_file(format!("{}/{}", BUILD_ASSETS_DIR, "windows_manifest.xml").as_str())
			.set_language(NEUTRAL_LCID)
			.compile()?;
	}

	Ok(())
}
