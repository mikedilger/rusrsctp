
extern crate errno;
extern crate rusrsctp_sys;

use std::sync::atomic::{AtomicBool, Ordering};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct UsrSctp {}

/// An object representing the SCTP networking system.
impl UsrSctp {
    /// Initialize SCTP.  You can only have one of these; Subsequent calls to this
    /// function will return None as long as you have an SCTP object still alive.
    pub fn new(port: Option<u16>) -> Option<UsrSctp>
    {
        // If it was false, make it true and enter this block
        if !INITIALIZED.compare_and_swap(false, true, Ordering::SeqCst) {
            // Initialize usrsctp
            unsafe {
                rusrsctp_sys::usrsctp_init(port.unwrap_or(0),
                                           None, // conn_output not supported (yet)
                                           None); // debug_printf not supported (yet)
            }

            Some(UsrSctp {})
        }
        else {
            None
        }
    }
}

impl Drop for UsrSctp {
    fn drop(&mut self) {
        unsafe {
            // FIXME: this can return -1 on error, although I don't know what
            // I should do in that case.
            rusrsctp_sys::usrsctp_finish();
        }
        INITIALIZED.store(false, Ordering::SeqCst);
    }
}
