//! Implements the program functionality in GUI mode.

// Uses
use std::{
	collections::HashMap,
	env::var,
	fmt,
	fmt::{Debug, Formatter},
	path::PathBuf,
	ptr::null,
};

use anyhow::{Context, Error, Result};
use druid::{
	commands::{CLOSE_WINDOW, QUIT_APP, SHOW_ABOUT, SHOW_PREFERENCES},
	widget::{Button, Flex, Label, Padding},
	AppDelegate,
	AppLauncher,
	Command,
	Data,
	DelegateCtx,
	Env,
	Handled,
	HasRawWindowHandle,
	LocalizedString,
	Menu,
	MenuItem,
	RawWindowHandle,
	Target,
	Widget,
	WidgetExt,
	WindowDesc,
	WindowHandle,
	WindowId,
	WindowLevel,
	WindowSizePolicy,
};
use lazy_static::lazy_static;
#[cfg(windows)]
use winapi::{
	shared::windef::{HICON, HWND__},
	um::{
		libloaderapi::GetModuleHandleW,
		winuser::{
			LoadImageW,
			SendMessageW,
			ICON_BIG,
			ICON_SMALL,
			IDI_APPLICATION,
			IMAGE_ICON,
			LR_DEFAULTSIZE,
			LR_SHARED,
			LR_VGACOLOR,
			WM_SETICON,
		},
	},
};

#[cfg(windows)]
use crate::util::assert_winapi_success;
use crate::{
	app::{load_config, LoadoutsConfig},
	constants::{LOADOUTS_CONFIG_PATH_VAR, PROGRAM_AUTHOURS, PROGRAM_VERSION, PROJECT_URL},
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
			ctx.new_window(
				WindowDesc::new(ui_about())
					.title(LocalizedString::new("about"))
					.window_size_policy(WindowSizePolicy::Content)
					.set_level(WindowLevel::Modal(self.main_window_handle()))
					.set_position((0.0, 0.0))
					.show_titlebar(true)
					.resizable(false),
			);
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
			// TODO: Handle this properly
			initialize_once_loaded(data).expect("Zacc forgot to handle this properly");
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

/// Start up the tool for GUI operation.
pub fn start() -> Result<()> {
	// Build the window
	let main_window = WindowDesc::new(ui_main())
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

// UI Definitions
fn ui_main() -> impl Widget<ProgramState> {
	let text = LocalizedString::new("program-name");
	let label = Label::new(text).padding(200.0).center();

	// Container for the whole UI
	Flex::column().with_child(label).with_spacer(100.0)
}

fn ui_about() -> impl Widget<ProgramState> {
	let label_version = Label::new(
		LocalizedString::new("about-version")
			.with_arg("program_version", |_, _| PROGRAM_VERSION.into()),
	)
	.padding(5.0)
	.center();
	let label_authours = Label::new(
		LocalizedString::new("about-authours")
			.with_arg("program_authours", |_, _| PROGRAM_AUTHOURS.into()),
	)
	.padding(5.0)
	.center();
	let label_url = Label::new(
		LocalizedString::new("about-url").with_arg("project_url", |_, _| PROJECT_URL.into()),
	)
	.padding(5.0)
	.center();
	let button_close = Button::new(LocalizedString::new("button-close"))
		.on_click(|ctx, _, _| ctx.submit_command(CLOSE_WINDOW))
		.padding(5.0)
		.center();

	// Container for the whole UI
	Padding::new(
		10.0,
		Flex::column()
			.with_child(label_version)
			.with_child(label_authours)
			.with_child(label_url)
			.with_child(button_close),
	)
}

/// Create the main menu bar.
fn make_menu(_: Option<WindowId>, _: &ProgramState, _: &Env) -> Menu<ProgramState> {
	let mut menu_main = Menu::new(LocalizedString::new("program-name"));

	let mut menu_file = Menu::new(LocalizedString::new("menu-file"));
	menu_file = menu_file
		.entry(MenuItem::new(LocalizedString::new("menu-file-refresh-config")).hotkey(None, "r"))
		.separator()
		.entry(MenuItem::new(LocalizedString::new("menu-file-about")).command(SHOW_ABOUT))
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

/// Sets the window icon at runtime.
///
/// Once Druid supports this natively, this function can be scrapped.
fn set_window_icon(handle: &WindowHandle) {
	let raw_handle = handle.raw_window_handle();
	#[allow(clippy::single_match)]
	match raw_handle {
		RawWindowHandle::Windows(win_handle) => unsafe {
			#[cfg(windows)]
			{
				// Ugly, but once we load the icon it *should* exist for the lifetime of the
				// program.
				// It doesn't make sense to re-load the icon every time we want to set it for a
				// window.
				lazy_static! {
					static ref PROGRAM_ICON: isize = unsafe {
						// Passing NULL means the executable file is selected
						let h_instance = GetModuleHandleW(null());

						// Don't need MAKEINTRESOURCEW() here because IDI_APPLICATION is already
						// converted
						LoadImageW(
							h_instance,
							IDI_APPLICATION,
							IMAGE_ICON,
							0,
							0,
							LR_SHARED | LR_DEFAULTSIZE | LR_VGACOLOR,
						)
						.cast::<HICON>() as isize
					};
				}
				assert_winapi_success();

				// Shown at the top of the window
				SendMessageW(
					win_handle.hwnd.cast::<HWND__>(),
					WM_SETICON,
					ICON_SMALL as usize,
					*PROGRAM_ICON,
				);
				assert_winapi_success();
				// Shown in the Alt+Tab dialog
				SendMessageW(
					win_handle.hwnd.cast::<HWND__>(),
					WM_SETICON,
					ICON_BIG as usize,
					*PROGRAM_ICON,
				);
				assert_winapi_success();
			}
		},
		_ => {}
	}
}
