
use super::*;

#[test]
fn setup_and_teardown4() {
    {
        let sctp = UsrSctp::new(Some(9899)).unwrap();
        {
            let _socket = sctp.socket::<Ipv4>(false).unwrap();
        } // socket drops here
    } // sctp drops here
    assert!(true)
}

#[test]
fn setup_and_teardown6() {
    {
        let sctp = UsrSctp::new(Some(9899)).unwrap();
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
        let sctp = UsrSctp::new(Some(9899)).unwrap();
        sctp.socket::<Ipv4>(false).unwrap()
    };
    assert!(true)
}
 */
