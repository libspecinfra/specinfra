#![cfg(feature="test-service")]
extern crate specinfra;

use specinfra::backend;
use specinfra::Specinfra;
use specinfra::provider::service::inline::null::Null;

#[test]
fn service_resource_with_inline_provider() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    test_service_resource(s);
}

#[test]
fn service_resource_with_shell_provider() {
    let b = backend::direct::Direct::new();
    let mut s = specinfra::new(&b).unwrap();
    s.providers.service.inline = Box::new(Null);
    test_service_resource(s);
}

fn test_service_resource(s: Specinfra) {
    let dbus = s.service("dbus.service");
    assert!(dbus.is_running().unwrap());

    let dbus = s.service("dbus");
    assert!(dbus.is_running().unwrap());

    let sshd = s.service("sshd");
    assert!(sshd.is_enabled().unwrap());

    let nginx = s.service("nginx");
    assert!(nginx.disable().unwrap());
    assert_eq!(nginx.is_enabled().unwrap(), false);

    assert!(nginx.enable().unwrap());
    assert!(nginx.is_enabled().unwrap());

    assert!(nginx.start().unwrap());
    assert!(nginx.is_running().unwrap());

    // assert!(nginx.reload().unwrap());
    // assert!(nginx.is_running().unwrap());

    // assert!(nginx.restart().unwrap());
    // assert!(nginx.is_running().unwrap());

    assert!(nginx.stop().unwrap());
    assert_eq!(nginx.is_running().unwrap(), false);
}
