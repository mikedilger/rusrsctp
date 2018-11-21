
extern crate errno;
extern crate rusrsctp_sys;

use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::os::raw::c_int;
use std::ptr;
use errno::Errno;
use rusrsctp_sys::{IPPROTO_SCTP, socket};

#[cfg(test)]
mod tests;

mod ip;
pub use self::ip::*;

static SOCK_STREAM: c_int = 1;
static SOCK_SEQPACKET: c_int = 5;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct UsrSctp {}

/// An object representing the SCTP networking system.
impl UsrSctp {
    /// Initialize SCTP.  You can only have one of these; Subsequent calls to this
    /// function will return None as long as you have an SCTP object still alive.
    /// If port is specified, SCTP will run over UDP (which traverses NAT and is
    /// generally more available over the Internet at large); otherwise SCTP will
    /// run over IP directly.  IANA has assigned 9899 as the SCTP over UDP port,
    /// but you don't have to use that one.
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

impl UsrSctp {
    pub fn socket<'a, T: 'a + Ip>(&'a self, one_to_many: bool) -> Result<Socket<'a, T>, Errno> {
        let socket = unsafe {
            rusrsctp_sys::usrsctp_socket(
                T::pf(),
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
                inner: socket,
                _ip: PhantomData
            })
        }
    }
}

#[allow(dead_code)]
pub struct Socket<'a, T: 'a + Ip> {
    inner: *mut socket,
    // Type parameterize a Socket with Ip (v4 or v6), while also using a reference
    // with the lifetime of UsrSctp so that socket objects cannot outlive UsrSctp.
    _ip: PhantomData<&'a T>,
}

impl<'a, T: 'a + Ip> Drop for Socket<'a, T> {
    fn drop(&mut self) {
        unsafe {
            rusrsctp_sys::usrsctp_close(self.inner);
        }
    }
}
