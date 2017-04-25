#[macro_use]
extern crate bitflags;
extern crate gdk;
#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gtk;
extern crate libc;
extern crate pango;
extern crate vte_sys as ffi;

macro_rules! assert_initialized_main_thread {
    () => (
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            }
            else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    )
}

macro_rules! skip_assert_initialized {
    () => ()
}

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

use glib::Error;
pub use auto::*;

mod auto;
mod terminal;
