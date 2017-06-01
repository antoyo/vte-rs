#[cfg(feature="v0_48")]
use std::path::PathBuf;
use std::ptr;

use ffi;
use gdk;
use glib::translate::*;
#[cfg(feature="v0_48")]
use glib_ffi;
use Terminal;

macro_rules! option_to_ptr {
    ($e:expr) => {
        match $e {
            Some(value) => value.to_glib_none().0,
            None => ptr::null()
        }
    };
}

impl Terminal {
    pub fn set_color_background(&self, background: &gdk::RGBA) {
        unsafe {
            ffi::vte_terminal_set_color_background(self.to_glib_none().0, background.to_glib_none().0);
        }
    }

    pub fn set_color_bold(&self, bold: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_bold(self.to_glib_none().0, option_to_ptr!(bold));
        }
    }

    pub fn set_color_cursor(&self, cursor_background: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_cursor(self.to_glib_none().0, option_to_ptr!(cursor_background));
        }
    }

    #[cfg(feature = "v0_44")]
    pub fn set_color_cursor_foreground(&self, cursor_foreground: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_cursor_foreground(self.to_glib_none().0, option_to_ptr!(cursor_foreground));
        }
    }

    pub fn set_color_foreground(&self, foreground: &gdk::RGBA) {
        unsafe {
            ffi::vte_terminal_set_color_foreground(self.to_glib_none().0, foreground.to_glib_none().0);
        }
    }

    pub fn set_color_highlight(&self, highlight_background: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_highlight(self.to_glib_none().0, option_to_ptr!(highlight_background));
        }
    }

    pub fn set_color_highlight_foreground(&self, highlight_foreground: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_highlight_foreground(self.to_glib_none().0, option_to_ptr!(highlight_foreground));
        }
    }

    #[cfg(feature="v0_48")]
    pub fn spawn_async(&self, working_directory: Option<PathBuf>, argv: &[&str], envv: &[&str]) {
        let directory = working_directory.as_ref().map(|path_buf| path_buf.as_path());
        unsafe {
            ffi::vte_terminal_spawn_async(self.to_glib_none().0, ffi::VTE_PTY_DEFAULT,
                directory.to_glib_none().0, argv.to_glib_none().0, envv.to_glib_none().0,
                glib_ffi::G_SPAWN_DEFAULT, None, ptr::null_mut(), None, -1, ptr::null_mut(),
                None, ptr::null_mut());
        }
    }
}
