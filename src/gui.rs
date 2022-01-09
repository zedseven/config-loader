//! Implements the program functionality in GUI mode.

use std::{env::var, path::PathBuf};

// Uses
use anyhow::{Context, Error, Result};
use druid::{
	commands::{QUIT_APP, SHOW_ABOUT, SHOW_PREFERENCES},
	widget::{Flex, Label},
	AppDelegate,
	AppLauncher,
	Command,
	Data,
	DelegateCtx,
	Env,
	Event,
	Handled,
	LocalizedString,
	Menu,
	MenuItem,
	Target,
	Widget,
	WidgetExt,
	WindowDesc,
	WindowId,
	WindowSizePolicy,
};

use crate::{
	app::{load_config, LoadoutsConfig},
	constants::LOADOUTS_CONFIG_PATH_VAR,
	util::get_default_config_path,
};

#[derive(Clone, Debug, Default)]
struct ProgramState {
	loadouts_config: Option<LoadoutsConfig>,
}

impl Data for ProgramState {
	fn same(&self, _: &Self) -> bool {
		// We should only ever have one instance anyways, so it isn't worth implementing
		// the equality operators
		true
	}
}

#[derive(Debug, Default)]
struct ProgramDelegate {
	/// Acts as an indicator of whether the first-time initialization has been
	/// performed.
	main_window_id: Option<WindowId>,
}

impl AppDelegate<ProgramState> for ProgramDelegate {
	fn command(
		&mut self,
		ctx: &mut DelegateCtx,
		target: Target,
		cmd: &Command,
		data: &mut ProgramState,
		env: &Env,
	) -> Handled {
		Handled::No
	}

	fn window_added(
		&mut self,
		window_id: WindowId,
		data: &mut ProgramState,
		_: &Env,
		_: &mut DelegateCtx,
	) {
		if self.main_window_id.is_none() {
			self.main_window_id = Some(window_id);
			// TODO: Handle this properly
			initialize_once_loaded(data).expect("Zacc forgot to handle this properly");
		}
	}
}

/// Start up the tool for GUI operation.
pub fn start() -> Result<()> {
	// Build the window
	let main_window = WindowDesc::new(ui_builder())
		.menu(make_menu)
		.title(LocalizedString::new("program-name"))
		.window_size_policy(WindowSizePolicy::Content)
		.resizable(false);

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

/// Build the UI definition.
fn ui_builder() -> impl Widget<ProgramState> {
	let text = LocalizedString::new("program-name");
	let label = Label::new(text).padding(5.0).center();

	// Container for the whole UI
	Flex::column().with_child(label).with_spacer(100.0)
}

/// Create the main menu bar.
fn make_menu(_: Option<WindowId>, _: &ProgramState, _: &Env) -> Menu<ProgramState> {
	let mut menu_main = Menu::new(LocalizedString::new("program-name"));

	let mut menu_file = Menu::new(LocalizedString::new("menu-file"));
	menu_file = menu_file
		.entry(MenuItem::new(LocalizedString::new("menu-file-refresh-config")).hotkey(None, "r"))
		.separator()
		.entry(
			MenuItem::new(LocalizedString::new("menu-file-about"))
				// TODO: Add this
				.command(SHOW_ABOUT),
		)
		.entry(
			MenuItem::new(LocalizedString::new("menu-file-preferences"))
				// TODO: Add this
				.command(SHOW_PREFERENCES),
		)
		.entry(
			MenuItem::new(LocalizedString::new("menu-file-exit"))
				.command(QUIT_APP)
				.hotkey(None, "e"),
		);

	menu_main = menu_main.entry(menu_file);
	menu_main
}
