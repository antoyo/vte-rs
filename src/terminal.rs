use glib::object::IsA;
use glib::translate::{ToGlib, ToGlibPtr};
use Terminal;

pub trait TerminalExtManual: 'static {
    fn watch_child(&self, child_pid: glib::Pid);
}

impl<O: IsA<Terminal>> TerminalExtManual for O {

    fn watch_child(&self, child_pid: glib::Pid) {
        unsafe {
            vte_sys::vte_terminal_watch_child(self.as_ref().to_glib_none().0, child_pid.to_glib());
        }
    }
}
