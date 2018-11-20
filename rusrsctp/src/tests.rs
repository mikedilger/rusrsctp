
use super::*;

#[test]
fn setup_and_teardown() {
    {
        let sctp = UsrSctp::new(Some(9899)).unwrap();
        {
            let _socket = sctp.socket(false, false).unwrap();
        } // socket drops here
    } // sctp drops here
    assert!(true)
}