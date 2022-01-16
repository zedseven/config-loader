//! UI Definitions

// Uses
use druid::{
	commands::{CLOSE_WINDOW, QUIT_APP, SHOW_ABOUT, SHOW_PREFERENCES},
	widget::{Button, Flex, Label, Padding},
	Env,
	LocalizedString,
	Menu,
	MenuItem,
	Widget,
	WidgetExt,
	WindowDesc,
	WindowHandle,
	WindowId,
	WindowLevel,
	WindowSizePolicy,
};

use super::ProgramState;
use crate::constants::{PROGRAM_AUTHOURS, PROGRAM_VERSION, PROJECT_URL};

// Main Window
/// Builds the main window.
pub(super) fn make_main_window() -> WindowDesc<ProgramState> {
	WindowDesc::new(ui_main())
		.menu(make_main_menu)
		.title(LocalizedString::new("program-name"))
		.window_size_policy(WindowSizePolicy::Content)
		.resizable(false)
}

/// Builds the main window interface.
fn ui_main() -> impl Widget<ProgramState> {
	let text = LocalizedString::new("program-name");
	let label = Label::new(text).padding(200.0).center();

	// Container for the whole UI
	Flex::column().with_child(label).with_spacer(100.0)
}

/// Builds the main menu bar.
fn make_main_menu(_: Option<WindowId>, _: &ProgramState, _: &Env) -> Menu<ProgramState> {
	let mut menu_main = Menu::new(LocalizedString::new("program-name"));

	let mut menu_file = Menu::new(LocalizedString::new("menu-file"));
	menu_file = menu_file
		.entry(MenuItem::new(LocalizedString::new("menu-file-refresh-config")).hotkey(None, "r"))
		.separator()
		.entry(MenuItem::new(LocalizedString::new("menu-file-about")).command(SHOW_ABOUT))
		.entry(
			MenuItem::new(LocalizedString::new("menu-file-preferences")).command(SHOW_PREFERENCES),
		)
		.entry(
			MenuItem::new(LocalizedString::new("menu-file-exit"))
				.command(QUIT_APP)
				.hotkey(None, "e"),
		);

	menu_main = menu_main.entry(menu_file);
	menu_main
}

// About Window
/// Builds the About window.
pub(super) fn make_about_window(main_handle: WindowHandle) -> WindowDesc<ProgramState> {
	WindowDesc::new(ui_about())
		.title(LocalizedString::new("about"))
		.window_size_policy(WindowSizePolicy::Content)
		.set_level(WindowLevel::Modal(main_handle))
		.set_position((0.0, 0.0))
		.show_titlebar(true)
		.resizable(false)
}

/// Builds the About window interface.
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

// Preferences Window
/// Builds the Preferences window.
pub(super) fn make_preferences_window(main_handle: WindowHandle) -> WindowDesc<ProgramState> {
	WindowDesc::new(ui_preferences())
		.title(LocalizedString::new("preferences"))
		.window_size_policy(WindowSizePolicy::Content)
		.set_level(WindowLevel::Modal(main_handle))
		.set_position((0.0, 0.0))
		.show_titlebar(true)
		.resizable(false)
}

/// Builds the Preferences window interface.
fn ui_preferences() -> impl Widget<ProgramState> {
	// Container for the whole UI
	Padding::new(10.0, Flex::column())
}
