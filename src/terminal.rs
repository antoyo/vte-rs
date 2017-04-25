use std::path::PathBuf;
use std::ptr;

use ffi;
use gdk;
use gio_ffi;
use glib::translate::*;
use glib_ffi;
use glib_ffi::{gboolean, gpointer};
use libc;
use Terminal;

impl Terminal {
    pub fn set_color_background(&self, background: &gdk::RGBA) {
        unsafe {
            ffi::vte_terminal_set_color_background(self.to_glib_none().0, background);
        }
    }

    pub fn set_color_bold(&self, bold: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_bold(self.to_glib_none().0, option_to_ptr(bold));
        }
    }

    pub fn set_color_cursor(&self, cursor_background: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_cursor(self.to_glib_none().0, option_to_ptr(cursor_background));
        }
    }

    #[cfg(feature = "v0_44")]
    pub fn set_color_cursor_foreground(&self, cursor_foreground: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_cursor_foreground(self.to_glib_none().0, option_to_ptr(cursor_foreground));
        }
    }

    pub fn set_color_foreground(&self, foreground: &gdk::RGBA) {
        unsafe {
            ffi::vte_terminal_set_color_foreground(self.to_glib_none().0, foreground);
        }
    }

    pub fn set_color_highlight(&self, highlight_background: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_highlight(self.to_glib_none().0, option_to_ptr(highlight_background));
        }
    }

    pub fn set_color_highlight_foreground(&self, highlight_foreground: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_highlight_foreground(self.to_glib_none().0, option_to_ptr(highlight_foreground));
        }
    }

    pub fn spawn_async(&self, working_directory: Option<PathBuf>, argv: &[&str], envv: &[&str]) {
        let directory = working_directory.as_ref().map(|path_buf| path_buf.as_path());
        unsafe {
            vte_terminal_spawn_async(self.to_glib_none().0, ffi::VTE_PTY_DEFAULT,
                directory.to_glib_none().0, argv.to_glib_none().0, envv.to_glib_none().0,
                glib_ffi::G_SPAWN_DEFAULT, None, ptr::null_mut(), None, -1, ptr::null_mut(),
                None, ptr::null_mut());
        }
    }
}

fn option_to_ptr<T>(value: Option<&T>) -> *const T {
    match value {
        Some(value) => value as *const _,
        None => ptr::null(),
    }
}

pub type VteTerminalSpawnAsyncCallback = Option<unsafe extern "C" fn(*mut ffi::VteTerminal,
    glib_ffi::GPid, glib_ffi::GError, gpointer)>;

extern "C" {
    pub fn vte_terminal_spawn_async(terminal: *mut ffi::VteTerminal, pty_flags: ffi::VtePtyFlags,
        working_directory: *const libc::c_char, argv: *mut *mut libc::c_char, envv: *mut *mut libc::c_char,
        spawn_flags: glib_ffi::GSpawnFlags, child_setup: glib_ffi::GSpawnChildSetupFunc,
        child_setup_data: gpointer, child_setup_data_destroy: glib_ffi::GDestroyNotify, timeout: libc::c_int,
        cancellable: *mut gio_ffi::GCancellable, callback: VteTerminalSpawnAsyncCallback,
        user_data: gpointer) -> gboolean;
}
