//! UI Definitions

// Uses
use druid::{
	commands::{CLOSE_WINDOW, QUIT_APP, SHOW_ABOUT, SHOW_PREFERENCES},
	widget::{
		Axis,
		Button,
		DisabledIf,
		Flex,
		Label,
		List,
		Maybe,
		Padding,
		Scroll,
		Tabs,
		TabsEdge,
		TabsTransition,
	},
	Env,
	LocalizedString,
	Menu,
	MenuItem,
	SysMods,
	TextAlignment,
	Widget,
	WidgetExt,
	WindowDesc,
	WindowHandle,
	WindowId,
	WindowLevel,
	WindowSizePolicy,
};

use super::ProgramState;
use crate::{
	app::{Loadout, LoadoutsConfig},
	constants::{PROGRAM_AUTHOURS, PROGRAM_VERSION, PROJECT_URL},
};

// Main Window
/// Builds the main window.
pub(super) fn make_main_window() -> WindowDesc<ProgramState> {
	WindowDesc::new(ui_main())
		.menu(make_main_menu)
		.title(LocalizedString::new("program-name"))
		//.window_size_policy(WindowSizePolicy::Content)
		.window_size((400.0, 700.0))
		.resizable(false)
}

/// Builds the main window interface.
fn ui_main() -> impl Widget<ProgramState> {
	Tabs::new()
		.with_axis(Axis::Horizontal)
		.with_edge(TabsEdge::Leading)
		.with_transition(TabsTransition::Slide(100_000_000))
		.with_tab(
			LocalizedString::new("tab-configuration"),
			build_config_tab(),
		)
		.with_tab(LocalizedString::new("tab-loadouts"), build_loadouts_tab())
		.with_tab_index(1)
}

fn build_config_tab() -> impl Widget<ProgramState> {
	Label::new(LocalizedString::new("tab-configuration"))
}

fn build_loadouts_tab() -> impl Widget<ProgramState> {
	Scroll::new(
		Flex::column()
			.with_child(Label::new(LocalizedString::new("tab-loadouts")).expand_width()) // TODO: To be expanded later for things like a Refresh button, Loaded status, etc.
			.with_spacer(5.0)
			.with_child(
				Maybe::new(
					|| {
						List::new(|| {
							// TODO: Need to find a way to hide, not just disable (maybe HiddenIf?)
							DisabledIf::new(
								// TODO: Need to find a way to left-align the label - https://xi.zulipchat.com/#narrow/stream/255910-druid-help/topic/Aligning.20text.20within.20a.20Button
								Button::from_label(
									Label::new(|item: &Loadout, _env: &_| item.name.clone())
										.with_text_alignment(TextAlignment::Start),
								)
								.expand_width()
								.padding((0.0, 0.0, 0.0, 5.0)),
								|item: &Loadout, _env: &_| item.hidden,
							)
						})
						.lens(LoadoutsConfig::loadouts)
					},
					|| Label::new("Loadout config not loaded"),
				)
				.lens(ProgramState::loadouts_config),
			),
	)
	.vertical()
}

/// Builds the main menu bar.
fn make_main_menu(_: Option<WindowId>, _: &ProgramState, _: &Env) -> Menu<ProgramState> {
	let mut menu_main = Menu::new(LocalizedString::new("program-name"));

	let mut menu_file = Menu::new(LocalizedString::new("menu-file"));
	menu_file = menu_file
		.entry(
			MenuItem::new(LocalizedString::new("menu-file-refresh-config"))
				.hotkey(SysMods::Cmd, "r"),
		)
		.separator()
		.entry(MenuItem::new(LocalizedString::new("menu-file-about")).command(SHOW_ABOUT))
		.entry(
			MenuItem::new(LocalizedString::new("menu-file-preferences"))
				.command(SHOW_PREFERENCES)
				.hotkey(SysMods::Cmd, "p"),
		)
		.entry(
			MenuItem::new(LocalizedString::new("menu-file-quit"))
				.command(QUIT_APP)
				.hotkey(SysMods::Cmd, "q"),
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
