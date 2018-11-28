
use std::net::{Ipv4Addr, Ipv6Addr};
use super::*;

#[test]
fn setup_and_teardown4() {
    {
        let sctp = UsrSctp::new(Some(9899), true);
        {
            let _socket = sctp.socket::<Ipv4>(false).unwrap();
        } // socket drops here
    } // sctp drops here
    assert!(true)
}

#[test]
fn setup_and_teardown6() {
    {
        let sctp = UsrSctp::new(Some(9899), true);
        {
            let _socket = sctp.socket::<Ipv6>(false).unwrap();
        } // socket drops here
    } // sctp drops here
    assert!(true)
}

/* We can't actually test "should_fail_to_compile"
#[test]
#[should_fail_to_compile(expected = "error[E0597]: `sctp` does not live long enough")]
fn test_socket_outlive_usrsctp() {
    let socket = {
        let sctp = UsrSctp::new(Some(9899), true);
        sctp.socket::<Ipv4>(false).unwrap()
    };
    assert!(true)
}
 */

#[test]
fn bind4() {
    let sctp = UsrSctp::new(Some(9899), true);
    {
        let mut socket = sctp.socket::<Ipv4>(false).unwrap();
        socket.bind(Ipv4Addr::new(0, 0, 0, 0), 0).unwrap(); // wildcard addr and port
    }
}

#[test]
fn bind6() {
    let sctp = UsrSctp::new(Some(9899), true);
    {
        let mut socket = sctp.socket::<Ipv6>(false).unwrap();
        socket.bind(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 0).unwrap(); // wildcard addr and port
    }
}

#[test]
fn listen4() {
    let sctp = UsrSctp::new(Some(9899), true);
    {
        let mut socket = sctp.socket::<Ipv4>(true).unwrap();
        socket.bind(Ipv4Addr::new(0, 0, 0, 0), 0).unwrap(); // wildcard addr and port
        socket.listen(8).unwrap();
    }

}

#[test]
fn listen6() {
    let sctp = UsrSctp::new(Some(9899), true);
    {
        let mut socket = sctp.socket::<Ipv6>(true).unwrap();
        socket.bind(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 0).unwrap(); // wildcard addr and port
        socket.listen(8).unwrap();
    }
}

#[test]
fn accept4() {
    let sctp = UsrSctp::new(Some(9899), true);
    {
        let mut socket = sctp.socket::<Ipv4>(false).unwrap();
        socket.set_non_blocking(true).unwrap();
        socket.bind(Ipv4Addr::new(0, 0, 0, 0), 0).unwrap(); // wildcard addr and port
        socket.listen(8).unwrap();
        match socket.accept() {
            Ok(_) => (), // unlikely, but not a failure
            Err(e) => {
                let ei: i32 = e.into();
                assert_eq!(ei as u32, EWOULDBLOCK);
            }
        };
    }
}

#[test]
fn accept6() {
    let sctp = UsrSctp::new(Some(9899), true);
    {
        let mut socket = sctp.socket::<Ipv6>(false).unwrap();
        socket.set_non_blocking(true).unwrap();
        socket.bind(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 0).unwrap(); // wildcard addr and port
        socket.listen(8).unwrap();
        match socket.accept() {
            Ok(_) => (), // unlikely, but not a failure
            Err(e) => {
                let ei: i32 = e.into();
                assert_eq!(ei as u32, EWOULDBLOCK);
            }
        };
    }
}

#[test]
fn connect4() {
    let sctp = UsrSctp::new(Some(9899), true);
    {
        let mut socket = sctp.socket::<Ipv4>(false).unwrap();
        socket.set_non_blocking(true).unwrap();
        match socket.connect(Ipv4Addr::new(127, 0, 0, 1), 10000) {
            Ok(_) => (), // unlikely, but not a failure
            Err(e) => {
                let ei: i32 = e.into();
                assert_eq!(ei as u32, EINPROGRESS);
            }
        }
    }
}

#[test]
fn connect6() {
    let sctp = UsrSctp::new(Some(9899), true);
    {
        let mut socket = sctp.socket::<Ipv6>(false).unwrap();
        socket.set_non_blocking(true).unwrap();
        match socket.connect(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 10000) {
            Ok(_) => (), // unlikely, but not a failure
            Err(e) => {
                let ei: i32 = e.into();
                assert_eq!(ei as u32, EINPROGRESS);
            }
        }
    }
}

#[test]
fn shutdown() {
    let sctp = UsrSctp::new(Some(9899), true);
    {
        let mut socket = sctp.socket::<Ipv6>(false).unwrap();
        socket.set_non_blocking(true).unwrap();
        match socket.connect(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 10000) {
            Ok(_) => (), // unlikely, but not a failure
            Err(e) => {
                let ei: i32 = e.into();
                assert_eq!(ei as u32, EINPROGRESS);
            }
        }
        socket.shutdown(Shutdown::RdWr).unwrap();
    }
}

#[test]
fn non_blocking() {
    let sctp = UsrSctp::new(Some(9899), true);
    {
        let mut socket = sctp.socket::<Ipv4>(true).unwrap();
        socket.set_non_blocking(true).unwrap();
        assert_eq!(socket.get_non_blocking().unwrap(), true);
        socket.set_non_blocking(false).unwrap();
        assert_eq!(socket.get_non_blocking().unwrap(), false);
    }
}
