#![cfg(feature="test-package")]
extern crate specinfra;

use specinfra::backend::direct::Direct;

#[test]
fn test_package_provider() {
    let b = Direct::new();
    let s = specinfra::new(&b).unwrap();

    let nginx = s.package("nginx", None);
    assert!(nginx.install().unwrap());
    assert!(nginx.is_installed().unwrap());
    assert_eq!(nginx.version().unwrap(), "1.12.2-1.el7_4.ngx");

    let nginx_with_version = s.package("nginx", Some("1.12.2-1.el7_4"));
    assert!(nginx_with_version.is_installed().unwrap());
    assert_eq!(nginx_with_version.version().unwrap(), "1.12.2-1.el7_4");

    assert!(nginx.remove().unwrap());
}
