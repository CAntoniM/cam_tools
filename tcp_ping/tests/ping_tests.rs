
use std::{net::TcpListener, time::{Duration, SystemTime}};
use tcpping::ping;

/// This is a simple test case for the ping where it
/// will open and listen to a tcp socket and will then 
/// call ping and will then see to make sure that we 
/// recived a connection open.
fn ping_test(listen_addr: &String, connect_addr: &String, timeout: Duration) -> bool {
    let listener: TcpListener;
    match TcpListener::bind(listen_addr) {
        Ok(socket) => {
            listener = socket;
        }
        Err(_) => {
            return false;
        }
    };

    match ping(connect_addr, timeout) {
        Ok(_) => {},
        Err (_)=> {
            return  false;
        },
    };
    match listener.set_nonblocking(true) {
        Ok(_) => {}
        Err(_) => {
            return false;
        }
    };
    let start = SystemTime::now();
    while SystemTime::now().duration_since(start).unwrap() < timeout {
        match listener.accept() {
            Ok(_) => {
                return true;
            }
            Err(_) => {}
        }
    };
    false
}

///This is a simple test to ensure that the most basic functionality works
///Using a loopback address
#[test]
pub fn ping_loopback() {
    let listener_addr  = String::from("127.0.0.1:14000");
    let ping_addr = String::from("127.0.0.1:14000");
    assert!(ping_test(&listener_addr,&ping_addr, Duration::from_secs(1)));
}

///This is a simple negative test to ensrue that when an invalid port number is
///given the application will fail
#[test]
pub fn ping_fail_loopback() {
    let listener_addr  = String::from("127.0.0.1:15001");
    let ping_addr = String::from("127.0.0.1:15000");
    assert!(!ping_test(&listener_addr,&ping_addr, Duration::from_secs(1)));
}

///This is test to ensure that pinging values contained in the systems
///etc hosts file which is generally where localhost is defined.
#[test]
pub fn ping_host() {
    let listener_addr  = String::from("localhost:16000");
    let ping_addr = String::from("localhost:16000");
    assert!(ping_test(&listener_addr,&ping_addr, Duration::from_secs(1)));
}

///This is a test to ensure that if we give some malformed hostname it will fail
///Orignally this test used loaclhost name however the rust networking api wil
///return an invalid pointer causing a seg fault (funnily enough I have never seg fauled cpp like this before)
#[test]
pub fn ping_fail_host() {
    let listener_addr  = String::from("localhost:17000");
    let ping_addr = String::from("ost:17000");
    assert!(!ping_test(&listener_addr,&ping_addr, Duration::from_secs(1)));
}
///This test is to ensure that basic dns lookups work
#[test]
pub fn ping_hostname() {
    let ping_addr = String::from("google.com:80");
    assert!(match ping(&ping_addr, Duration::from_secs(1)) {
        Ok(_) => true,
        Err (_)=> false,
    });
}

///This is a simple test to check that when using an incorrect dns lookup
///that the application will fail 
#[test]
pub fn ping_fail_hostname() {
    let ping_addr = String::from("sdoersd.jes:80");
    assert!(match ping(&ping_addr, Duration::from_secs(1)) {
        Ok(_) => false,
        Err (_)=> true,
    });
}