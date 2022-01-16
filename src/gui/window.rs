//! Window-related functions.

// Uses
use std::ptr::null;

use druid::{HasRawWindowHandle, RawWindowHandle, WindowHandle};
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

/// Sets the window icon at runtime.
///
/// Once Druid supports this natively, this function can be scrapped.
pub(super) fn set_window_icon(handle: &WindowHandle) {
	let raw_handle = handle.raw_window_handle();
	#[allow(clippy::single_match)]
	match raw_handle {
		#[cfg(windows)]
		RawWindowHandle::Windows(win_handle) => unsafe {
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
		},
		_ => {}
	}
}
