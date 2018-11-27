//! `rusrsctp` =  Rust (Userland (Stream Control Transmission Protocol))
//!
//! For high level information, refer to the README.md file.
//!
//! This library provides rust bindings to `usrsctp`, a userspace SCTP library written
//! in C which operates either at the IP layer or over UDP.
//!
//! Example server:
//! ```
//! # extern crate rusrsctp;
//! # use rusrsctp::*;
//! # use std::net::Ipv6Addr;
//! # fn main() {
//! /// Start SCTP over the IANA-assigned tunnelling port
//! let sctp = UsrSctp::new(Some(9899));
//!
//! // Create an IPv6 socket in one-to-one mode
//! let mut socket = sctp.socket::<Ipv6>(false).unwrap();
//!
//! // Bind to wildcard address, port 5000
//! socket.bind(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 5000).unwrap();
//!
//! // Listen with a backlog of 8
//! socket.listen(8).unwrap();
//!
//! // Accept a connection (you probably want to loop and handle in another thread, or
//! // use a state machine (mio, tokio, etc)).
//! # socket.set_non_blocking(true).unwrap();
//! if let Ok((_remote_addr, _remote_port, client_socket)) =  socket.accept() {
//!   // do client_socket.sendv() and client_socket.recvv() operations...
//!
//!   // client_socket will close on drop
//! }
//!
//! // socket will close on drop
//! # ; }
//! ```
//!
//! Example client:
//! ```
//! # extern crate rusrsctp;
//! # use rusrsctp::*;
//! # use std::net::Ipv6Addr;
//! # fn main() {
//! /// Start SCTP over the IANA-assigned tunnelling port
//! let sctp = UsrSctp::new(Some(9899));
//!
//! // Create an IPv6 socket in one-to-one mode
//! let mut socket = sctp.socket::<Ipv6>(false).unwrap();
//!
//! // Connect to a server (use a real IP address, and prepare to wait)
//! # socket.set_non_blocking(true).unwrap();
//! // socket.connect(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 5000).unwrap();
//!
//! // do socket.sendv() and socket.recvv() operations...
//!
//! // socket will close on drop
//! # ; }
//! ```


extern crate errno;
extern crate rusrsctp_sys;
#[macro_use]
extern crate bitflags;

use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::thread;
use std::time;
use std::mem;
use errno::Errno;
use rusrsctp_sys::*;

#[cfg(test)]
mod tests;

mod ip;
pub use self::ip::*;

mod types;
pub use self::types::*;

static SOCK_STREAM: c_int = 1;
static SOCK_SEQPACKET: c_int = 5;

static REFCOUNT: AtomicUsize = AtomicUsize::new(0);
// We set this true AFTER intialization is complete (we bump REFCOUNT before)
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct UsrSctp {}

/// An object representing the SCTP networking system.
impl UsrSctp {
    /// Initialize SCTP.
    /// If port is specified, SCTP will run over UDP (which traverses NAT
    /// and is generally more available over the Internet at large); otherwise SCTP
    /// will run over IP directly.  IANA has assigned 9899 as the SCTP over UDP port,
    /// but you don't have to use that one.  Communications within SCTP use their
    /// own notion of ports independent of this UDP layer port.
    /// If another thread (or the current one) already started SCTP, `port` will
    /// be ignored and the already setup SCTP will be used.
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

impl<'a, T: 'a + Ip> Socket<'a, T> {
    pub fn bind(&mut self, addr: T::Addr, port: u16) -> Result<(), Errno> {
        let mut sa = T::to_sockaddr(addr, port);
        let rval = unsafe {
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

    pub fn connect(&mut self, addr: T::Addr, port: u16) -> Result<(), Errno> {
        let mut sa = T::to_sockaddr(addr, port);
        let rval = unsafe {
            // We cannot transmute, we have to pass the pointer through the void.C world did.
            usrsctp_connect(
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

    pub fn shutdown(&mut self, shutdown: Shutdown) -> Result<(), Errno> {
        let how = match shutdown {
            Shutdown::Rd => SHUT_RD,
            Shutdown::Wr => SHUT_WR,
            Shutdown::RdWr => SHUT_RDWR,
        };
        let rval = unsafe {
            usrsctp_shutdown(
                self.inner,
                how as i32
            )
        };
        if rval < 0 {
            Err(errno::errno())
        } else {
            Ok(())
        }
    }

    pub fn set_non_blocking(&mut self, onoff: bool) -> Result<(), Errno> {
        let rval = unsafe {
            usrsctp_set_non_blocking(
                self.inner,
                if onoff { 1 } else { 0 }
            )
        };
        if rval < 0 {
            Err(errno::errno())
        } else {
            Ok(())
        }
    }

    pub fn get_non_blocking(&mut self) -> Result<bool, Errno> {
        let rval = unsafe {
            usrsctp_get_non_blocking(
                self.inner)
        };
        if rval < 0 {
            Err(errno::errno())
        } else if rval > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Send data.
    /// NOTE: usrsctp limits addr to zero or one.  SCTP itself allows multiple
    /// addresses.  So we are limited by `usrsctp` on that point.
    pub fn sendv(&mut self,
                 data: &[u8],
                 addr: Option<(T::Addr, u16)>,
                 snd_info: Option<SndInfo>,
                 pr_info: Option<PrInfo>,
                 auth_info: Option<AuthInfo>,
                 flags: MsgFlags) -> Result<usize, Errno>
    {
        let sa = addr.map(|(addr,port)| {
            T::to_sockaddr(addr, port)
        });
        let addrcnt: i32 = if sa.is_some() { 1 } else { 0 };

        let infotype = if auth_info.is_some() || (snd_info.is_some() && pr_info.is_some()) {
            SCTP_SENDV_SPA
        } else if snd_info.is_some() {
            SCTP_SENDV_SNDINFO
        } else if pr_info.is_some() {
            SCTP_SENDV_PRINFO
        } else {
            SCTP_SENDV_NOINFO
        };

        // Build the sctp_sendv_spa structure
        // Even if we don't use full spa, we will use internal fields
        let mut spa_flags: u32 = 0;
        if snd_info.is_some() { spa_flags |= SCTP_SEND_SNDINFO_VALID; }
        if pr_info.is_some() { spa_flags |= SCTP_SEND_PRINFO_VALID; }
        if auth_info.is_some() { spa_flags |= SCTP_SEND_AUTHINFO_VALID; }
        let mut spa = sctp_sendv_spa {
            sendv_flags: spa_flags,
            sendv_sndinfo: snd_info.unwrap_or_default()
                .into_sctp_sndinfo(),
            sendv_prinfo: pr_info.unwrap_or_default()
                .into_sctp_prinfo(),
            sendv_authinfo: auth_info.unwrap_or(sctp_authinfo {
                    auth_keynumber: 0
                })
        };

        let (infoptr, infolen) = match infotype {
            SCTP_SENDV_SPA => (&mut spa as *mut sctp_sendv_spa as *mut c_void,
                               mem::size_of::<sctp_sendv_spa>()),
            SCTP_SENDV_SNDINFO => (&mut spa.sendv_sndinfo as *mut sctp_sndinfo as *mut c_void,
                                   mem::size_of::<sctp_sndinfo>()),
            SCTP_SENDV_PRINFO => (&mut spa.sendv_prinfo as *mut sctp_prinfo as *mut c_void,
                                  mem::size_of::<sctp_prinfo>()),
            SCTP_SENDV_NOINFO => (ptr::null_mut(), 0),
            _ => unreachable!()
        };

        let rval = unsafe {
            usrsctp_sendv(
                self.inner,
                data.as_ptr() as *const c_void,
                data.len(), // * mem::size_of<u8>() which is 1
                match sa {
                    Some(mut s) => &mut s as *mut T::Sockaddr as *mut c_void as *mut sockaddr,
                    None => ptr::null_mut(),
                },
                addrcnt,
                infoptr,
                infolen as u32,
                infotype,
                flags.bits() as i32)
        };
        if rval < 0 {
            Err(errno::errno())
        } else {
            Ok(rval as usize)
        }
    }
}
