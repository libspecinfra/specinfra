extern crate specinfra;

use specinfra::backend;

#[test]
#[cfg(target_os="macos")]
fn it_works() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    let file = s.file("/etc/passwd");

    assert_eq!(0o644, file.mode().unwrap())
}
