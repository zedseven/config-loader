//! Implements the program functionality in GUI mode.

// Modules
mod ui;
mod window;

// Uses
use std::{
	collections::HashMap,
	env::var,
	fmt,
	fmt::{Debug, Formatter},
	path::PathBuf,
};

use anyhow::{Context, Error, Result};
use druid::{
	commands::{SHOW_ABOUT, SHOW_PREFERENCES},
	AppDelegate,
	AppLauncher,
	Command,
	Data,
	DelegateCtx,
	Env,
	Handled,
	Lens,
	Target,
	WindowHandle,
	WindowId,
};

use crate::{
	app::{load_config, LoadoutsConfig},
	constants::LOADOUTS_CONFIG_PATH_VAR,
	gui::{
		ui::{make_about_window, make_main_window, make_preferences_window},
		window::set_window_icon,
	},
	util::get_default_config_path,
};

// Types
#[derive(Clone, Data, Debug, Default, Lens)]
struct ProgramState {
	loadouts_config: Option<LoadoutsConfig>,
}

// Startup
/// Start up the tool for GUI operation.
pub fn start() -> Result<()> {
	// Build the window
	let main_window = make_main_window();

	// Run the program
	AppLauncher::with_window(main_window)
		.delegate(ProgramDelegate::default())
		.log_to_console()
		.localization_resources(vec!["main.ftl".to_owned()], "runtime/i18n".to_owned())
		.launch(ProgramState::default())
		.with_context(|| "encountered a fatal error")
}

/// Run initialization code that requires the main window to be loaded.
fn initialize_once_loaded(data: &mut ProgramState) -> Result<()> {
	dbg!(&data);

	// Get the loadout config path
	let config_file = var(LOADOUTS_CONFIG_PATH_VAR)
		.ok()
		.map(PathBuf::from)
		.or_else(get_default_config_path)
		.ok_or_else(|| Error::msg("unable to get a value for the loadouts config file"))?;

	// Load the config
	data.loadouts_config =
		Some(load_config(&config_file).with_context(|| "unable to load the loadouts config")?);

	dbg!(&data);

	Ok(())
}

// Druid Delegate
#[derive(Default)]
struct ProgramDelegate {
	/// Acts as an indicator of whether the first-time initialization has been
	/// performed.
	main_window_id: Option<WindowId>,
	window_handles: HashMap<WindowId, WindowHandle>,
}

impl ProgramDelegate {
	/// Gets the [`WindowHandle`] for the main window.
	fn main_window_handle(&self) -> WindowHandle {
		self.window_handles
			.get(
				&self
					.main_window_id
					.expect("somehow didn't have the main window ID"),
			)
			.expect("somehow didn't have a window handle saved for the main window ID")
			.clone()
	}
}

impl AppDelegate<ProgramState> for ProgramDelegate {
	fn command(
		&mut self,
		ctx: &mut DelegateCtx,
		_: Target,
		cmd: &Command,
		_: &mut ProgramState,
		_: &Env,
	) -> Handled {
		if cmd.is(SHOW_ABOUT) {
			ctx.new_window(make_about_window(self.main_window_handle()));
			Handled::Yes
		} else if cmd.is(SHOW_PREFERENCES) {
			ctx.new_window(make_preferences_window(self.main_window_handle()));
			Handled::Yes
		} else {
			Handled::No
		}
	}

	fn window_added(
		&mut self,
		window_id: WindowId,
		handle: WindowHandle,
		data: &mut ProgramState,
		_: &Env,
		_: &mut DelegateCtx,
	) {
		// Store the window ID and handle for later use
		self.window_handles.insert(window_id, handle.clone());

		// Do program startup stuff and store the main window ID
		if self.main_window_id.is_none() {
			self.main_window_id = Some(window_id);
			// TODO: Handle this better (pass it up the chain somehow?)
			initialize_once_loaded(data).expect("something went wrong when doing initialization");
		}

		// Set the window icon
		set_window_icon(&handle);
	}

	fn window_removed(
		&mut self,
		window_id: WindowId,
		_: &mut ProgramState,
		_: &Env,
		_: &mut DelegateCtx,
	) {
		self.window_handles.remove(&window_id);
	}
}

impl Debug for ProgramDelegate {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("ProgramDelegate")
			.field("main_window_id", &self.main_window_id)
			.finish()
	}
}
