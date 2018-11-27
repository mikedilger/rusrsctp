
use std::net::{Ipv4Addr, Ipv6Addr};
use rusrsctp_sys::{sockaddr_in, sockaddr_in6, PF_INET, PF_INET6,
                   AF_INET, AF_INET6, in_addr, in6_addr};

fn u16be(v: u16) -> u16 {
    if cfg!(target_endian = "little") {
        ((v & 0x00FF) << 8) | ((v & 0xFF00) >> 8)
    } else {
        v
    }
}

fn u32be(v: u32) -> u32 {
    if cfg!(target_endian = "little") {
        ((v & 0x000000FF) << 24)
            | ((v & 0x0000FF00) << 8)
            | ((v & 0x00FF0000) >> 8)
            | ((v & 0xFF000000) >> 24)
    } else {
        v
    }
}

pub trait Ip {
    type Addr;
    type Sockaddr;
    fn pf() -> i32;
    fn to_sockaddr(addr: Self::Addr, port: u16) -> Self::Sockaddr;
    fn to_sockaddr_wildcard() -> Self::Sockaddr;
    fn from_sockaddr(sockaddr: Self::Sockaddr) -> (Self::Addr, u16);
}
pub struct Ipv4;
impl Ip for Ipv4 {
    type Addr = Ipv4Addr;
    type Sockaddr = sockaddr_in;
    fn pf() -> i32 { PF_INET as i32 }
    fn to_sockaddr(addr: Self::Addr, port: u16) -> Self::Sockaddr {
        sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: u16be(port),
            sin_addr: in_addr {
                s_addr: u32be(addr.into()),
            },
            sin_zero: [0,0,0,0,0,0,0,0],
        }
    }
    fn to_sockaddr_wildcard() -> Self::Sockaddr {
        Self::to_sockaddr(Ipv4Addr::UNSPECIFIED, 0)
    }
    fn from_sockaddr(sockaddr: Self::Sockaddr) -> (Self::Addr, u16) {
        (sockaddr.sin_addr.s_addr.into(), sockaddr.sin_port)
    }
}
pub struct Ipv6;
impl Ip for Ipv6 {
    type Addr = Ipv6Addr;
    type Sockaddr = sockaddr_in6;
    fn pf() -> i32 { PF_INET6 as i32 }
    fn to_sockaddr(addr: Self::Addr, port: u16) -> Self::Sockaddr {
        let mut segments = addr.segments();
        for i in 0..8 {
            segments[i] = u16be(segments[i]); // convert to big endian
        }
        sockaddr_in6 {
            sin6_family: AF_INET6 as u16,
            sin6_port: u16be(port),
            sin6_flowinfo: 0, // RFC2460 requires zero when not supported
            sin6_addr: in6_addr {
                __in6_u: rusrsctp_sys::in6_addr__bindgen_ty_1 {
                    __u6_addr16: segments
                },
            },
            sin6_scope_id: 0 // only used for local scope, and we don't set it here
        }
    }
    fn to_sockaddr_wildcard() -> Self::Sockaddr {
        Self::to_sockaddr(Ipv6Addr::UNSPECIFIED, 0)
    }
    fn from_sockaddr(sockaddr: Self::Sockaddr) -> (Self::Addr, u16) {
        unsafe {
            (sockaddr.sin6_addr.__in6_u.__u6_addr16.into(), sockaddr.sin6_port)
        }
    }
}
