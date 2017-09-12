extern crate specinfra;

use specinfra::backend;

#[test]
#[cfg(any(target_os="macos", target_os="linux"))]
fn file_resource_works() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    let file = s.file("/etc/passwd");

    assert_eq!(file.mode().unwrap(), 0o644);

    assert_eq!(file.is_file().unwrap(), true);
}
