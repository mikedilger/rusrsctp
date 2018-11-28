
use std::mem;
use rusrsctp_sys::*;

pub trait SctpOption {
    type Value;
    fn c_name(&self) -> i32;
    fn value_ptr<'a>(&'a self) -> &'a Self::Value;
    fn value_ptr_mut<'a>(&'a mut self) -> &'a mut Self::Value;
    fn value_size(&self) -> u32 {
        mem::size_of::<Self::Value>() as u32
    }
}

pub struct RtoInfo(pub sctp_rtoinfo);
impl SctpOption for RtoInfo {
    type Value = sctp_rtoinfo;
    fn c_name(&self) -> i32 { SCTP_RTOINFO as i32 }
    fn value_ptr<'a>(&'a self) -> &'a Self::Value {
        &self.0
    }
    fn value_ptr_mut<'a>(&'a mut self) -> &'a mut Self::Value {
        &mut self.0
    }
}

pub struct RemoteUdpEncapsPort(pub sctp_udpencaps);
impl SctpOption for RemoteUdpEncapsPort {
    type Value = sctp_udpencaps;
    fn c_name(&self) -> i32 { SCTP_REMOTE_UDP_ENCAPS_PORT as i32 }
    fn value_ptr<'a>(&'a self) -> &'a Self::Value {
        &self.0
    }
    fn value_ptr_mut<'a>(&'a mut self) -> &'a mut Self::Value {
        &mut self.0
    }
}
