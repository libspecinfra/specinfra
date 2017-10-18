#![cfg(feature="inline-systemd")]
extern crate specinfra;

use specinfra::backend;

#[test]
fn service_resource() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();

    let dbus = s.service("dbus.service");
    assert!(dbus.is_running().unwrap());

    let dbus = s.service("dbus");
    assert!(dbus.is_running().unwrap());

    let sshd = s.service("sshd");
    assert!(sshd.is_enabled().unwrap());

    let nginx = s.service("nginx");
    assert!(nginx.enable().unwrap());
    assert!(nginx.is_enabled().unwrap());
    assert!(nginx.disable().unwrap());
    assert_eq!(nginx.is_enabled().unwrap(), false);

    assert!(nginx.start().unwrap());
    assert!(nginx.is_running().unwrap());
    assert!(nginx.stop().unwrap());
    assert_eq!(nginx.is_running().unwrap(), false);

}
