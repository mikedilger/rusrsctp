
use std::net::{Ipv4Addr, Ipv6Addr};
use rusrsctp_sys::{sockaddr_in, sockaddr_in6, PF_INET, PF_INET6};

pub trait Ip {
    type Addr;
    type Sockaddr;
    fn pf() -> i32;
    fn sockaddr(addr: Self::Addr, port: u16) -> Self::Sockaddr;
}
pub struct Ipv4;
impl Ip for Ipv4 {
    type Addr = Ipv4Addr;
    type Sockaddr = sockaddr_in;
    fn pf() -> i32 { PF_INET as i32 }
    fn sockaddr(_addr: Self::Addr, _port: u16) -> Self::Sockaddr {
        unimplemented!()
    }
}
pub struct Ipv6;
impl Ip for Ipv6 {
    type Addr = Ipv6Addr;
    type Sockaddr = sockaddr_in6;
    fn pf() -> i32 { PF_INET6 as i32 }
    fn sockaddr(_addr: Self::Addr, _port: u16) -> Self::Sockaddr {
        unimplemented!()
    }
}
