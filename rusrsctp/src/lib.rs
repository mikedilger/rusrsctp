
extern crate errno;
extern crate rusrsctp_sys;

use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::os::raw::c_int;
use std::ptr;
use std::mem;
use std::thread;
use std::time;
use errno::Errno;
use rusrsctp_sys::*;

#[cfg(test)]
mod tests;

mod ip;
pub use self::ip::*;

static SOCK_STREAM: c_int = 1;
static SOCK_SEQPACKET: c_int = 5;

static REFCOUNT: AtomicUsize = AtomicUsize::new(0);
// We set this true AFTER intialization is complete (we bump REFCOUNT before)
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct UsrSctp {}

/// An object representing the SCTP networking system.
impl UsrSctp {
    /// Initialize SCTP.
    /// If port is specified, SCTP will run over UDP (which traverses NAT and is
    /// generally more available over the Internet at large); otherwise SCTP will
    /// run over IP directly.  IANA has assigned 9899 as the SCTP over UDP port,
    /// but you don't have to use that one.
    pub fn new(port: Option<u16>) -> UsrSctp
    {
        // If it was 0, make it 1 and enter this block
        if REFCOUNT.fetch_add(1, Ordering::SeqCst) == 0 {
            // We were the first!  We get to initialize
            unsafe {
                usrsctp_init(port.unwrap_or(0),
                             None, // conn_output not supported (yet)
                             None); // debug_printf not supported (yet)
            }
            INITIALIZED.store(true, Ordering::SeqCst);
        } else {
            // Only proceed once INITIALIZED is true -- other thread could be
            // still working on initialization
            let mut safety: usize = 0;
            while !INITIALIZED.load(Ordering::SeqCst) {
                thread::sleep(time::Duration::from_millis(1));
                safety += 1;
                if safety > 1000 {
                    panic!("Waiting >1s for SCTP to initialize");
                }
            }
        }
        UsrSctp {}
    }
}

impl Drop for UsrSctp {
    fn drop(&mut self) {
        if REFCOUNT.fetch_sub(1, Ordering::SeqCst) == 1 {
            unsafe {
                // FIXME: this can return -1 on error, although I don't know what
                // I should do in that case.
                usrsctp_finish();
            }
        }
    }
}

impl UsrSctp {
    pub fn socket<'a, T: 'a + Ip>(&'a self, one_to_many: bool) -> Result<Socket<'a, T>, Errno> {
        let so = unsafe {
            usrsctp_socket(
                T::pf(),
                if one_to_many { SOCK_SEQPACKET } else { SOCK_STREAM }, // type
                IPPROTO_SCTP as i32,
                None, // callback API (receive_cb) not supported (yet)
                None, // callback API (send_cb) not supported (yet)
                0, // sb_threshold is irrelevant without send_cp
                ptr::null_mut() // ulp_info is irrelevant without receive_cp
            )
        };
        if so.is_null() {
            Err(errno::errno())
        } else {
            Ok(Socket {
                inner: so,
                _ip: PhantomData,
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
            usrsctp_close(self.inner);
        }
    }
}

// fixme: for Ipv6, bind accepts either Ipv4 or Ipv6.  In our coding, we are
// forcing it to Ipv6

impl<'a, T: 'a + Ip> Socket<'a, T> {
    pub fn bind(&mut self, addr: T::Addr, port: u16) -> Result<(), Errno> {
        let mut sa = T::to_sockaddr(addr, port);
        let rval = unsafe {
            use ::std::os::raw::c_void;
            // We cannot transmute, we have to pass the pointer through the void.C world did.
            usrsctp_bind(
                self.inner,
                &mut sa as *mut T::Sockaddr as *mut c_void as *mut sockaddr,
                mem::size_of::<T::Sockaddr>() as u32
            )
        };
        if rval < 0 {
            Err(errno::errno())
        } else {
            Ok(())
        }
    }

    pub fn listen(&mut self, backlog: i32) -> Result<(), Errno> {
        let rval = unsafe {
            usrsctp_listen(
                self.inner,
                backlog
            )
        };
        if rval < 0 {
            Err(errno::errno())
        } else {
            Ok(())
        }
    }

    pub fn accept(&mut self) -> Result<(T::Addr, u16, Socket<'a, T>), Errno> {
        // space for return value
        let mut sa: T::Sockaddr = T::to_sockaddr_wildcard();
        let mut sa_len: u32 = 0;
        let so = unsafe {
            use ::std::os::raw::c_void;
            // We cannot transmute, we have to pass the pointer through the void.C world did.
            usrsctp_accept(
                self.inner,
                &mut sa as *mut T::Sockaddr as *mut c_void as *mut sockaddr,
                &mut sa_len as *mut u32
            )
        };
        if so.is_null() {
            Err(errno::errno())
        } else {
            let (addr, port) = T::from_sockaddr(sa);
            Ok((addr, port, Socket {
                inner: so,
                _ip: PhantomData,
            }))
        }
    }
}
