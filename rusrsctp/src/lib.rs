
extern crate errno;
extern crate rusrsctp_sys;

use std::sync::atomic::{AtomicBool, Ordering};
use std::os::raw::c_int;
use std::ptr;
use errno::Errno;
use rusrsctp_sys::{PF_INET, PF_INET6, IPPROTO_SCTP, socket};
static SOCK_STREAM: c_int = 1;
static SOCK_SEQPACKET: c_int = 5;

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

#[allow(dead_code)]
pub struct Socket {
    inner: *mut socket,
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe {
            rusrsctp_sys::usrsctp_close(self.inner);
        }
    }
}

impl UsrSctp {
    pub fn socket(&self, inet6: bool, one_to_many: bool) -> Result<Socket, Errno> {
        let socket = unsafe {
            rusrsctp_sys::usrsctp_socket(
                if inet6 { PF_INET6 as i32 } else { PF_INET as i32 }, // domain
                if one_to_many { SOCK_SEQPACKET } else { SOCK_STREAM }, // type
                IPPROTO_SCTP as i32,
                None, // callback API (receive_cb) not supported (yet)
                None, // callback API (send_cb) not supported (yet)
                0, // sb_threshold is irrelevant without send_cp
                ptr::null_mut() // ulp_info is irrelevant without receive_cp
            )
        };
        if socket.is_null() {
            Err(errno::errno())
        } else {
            Ok(Socket {
                inner: socket
            })
        }
    }
}
