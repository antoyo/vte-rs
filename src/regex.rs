use std::ptr;

use ffi;
use glib::translate::{ToGlibPtr, from_glib_full, from_glib_none};
use gtk::Error;
use libc;

use Regex;

impl Regex {
    pub fn new_for_match(pattern: &str, pattern_length: libc::c_long, flags: u32) -> Result<Regex, Error> {
        let mut error = ptr::null_mut();
        unsafe {
            let regex = ffi::vte_regex_new_for_match(pattern.to_glib_none().0, pattern_length as _, flags, &mut error);
            if error.is_null() {
                Ok(from_glib_none(regex))
            }
            else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_for_search(pattern: &str, pattern_length: libc::c_long, flags: u32) -> Result<Regex, Error> {
        let mut error = ptr::null_mut();
        unsafe {
            let regex = ffi::vte_regex_new_for_search(pattern.to_glib_none().0, pattern_length as _, flags, &mut error);
            if error.is_null() {
                Ok(from_glib_none(regex))
            }
            else {
                Err(from_glib_full(error))
            }
        }
    }
}
