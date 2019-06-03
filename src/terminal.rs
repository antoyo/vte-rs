use glib::object::IsA;
use Terminal;

pub trait TerminalExtManual: 'static {
	#[cfg(any(feature = "v0_46", feature = "dox"))]
	fn event_check_regex_simple(&self, event: &mut gdk::Event, regexes: &[&Regex], match_flags: u32) -> Option<Vec<GString>>;
}

impl<O: IsA<Terminal>> TerminalExtManual for O {
	/// Checks each regex in regexes if the text in and around the position of the event matches the regular expressions.
    #[cfg(any(feature = "v0_46", feature = "dox"))]
    fn event_check_regex_simple(&self, event: &mut gdk::Event, regexes: &[&Regex], match_flags: u32) -> Option<Vec<GString>> {
        unsafe {
            let mut n_regexes = mem::uninitialized();
            let mut matches = Vec::<GString>::uninitialized();
            let ret = from_glib(vte_sys::vte_terminal_event_check_regex_simple(self.as_ref().to_glib_none().0, event.to_glib_none_mut().0, regexes.to_glib_none().0, &mut n_regexes, match_flags, matches.to_glib_none_mut().0));
            if ret { Some(matches) } else { None }
        }
    }
}
