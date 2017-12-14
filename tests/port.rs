#![cfg(feature="test-port")]
extern crate specinfra;

use specinfra::backend::direct::Direct;

#[test]
fn test_port_provider() {
    let b = Direct::new();
    let s = specinfra::new(&b).unwrap();

    let ssh_port = s.port(22);
    assert!(ssh_port.is_listening().unwrap());
}
