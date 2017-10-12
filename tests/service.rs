#![cfg(feature="inline-systemd")]
extern crate specinfra;

use specinfra::backend;

#[test]
fn service_resource() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    let dbus = s.service("dbus.service");
    assert!(dbus.is_running().unwrap());
}
