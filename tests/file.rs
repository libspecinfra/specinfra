extern crate specinfra;

use specinfra::backend;
use specinfra::Specinfra;
// use specinfra::provider::file::inline::null::Null;

#[test]
#[cfg(any(target_os="macos", target_os="linux"))]
fn file_resource_with_inline_provider() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    test_file_resource(s);
}

#[test]
fn file_not_exist_with_inline_provider() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    test_file_not_exit(s);
}

// #[test]
// fn file_not_exist_with_shell_provider() {
// let b = backend::direct::Direct::new();
// let mut s = specinfra::new(&b).unwrap();
// s.provider.file.inline = Box::new(Null);
// test_file_not_exit(s);
// }
//

#[test]
#[cfg(target_os="macos")]
fn file_link_on_macos_with_inline_provider() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    test_file_link_on_macos(s);
}

#[test]
#[cfg(target_os="linux")]
fn file_link_with_inline_provder_for_linux() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    test_file_link_on_linux(s);
}

fn test_file_resource(s: Specinfra) {
    let file = s.file("/etc/passwd");

    assert_eq!(file.mode().unwrap(), 0o644);
    assert_eq!(file.is_file().unwrap(), true);
    assert_eq!(file.exist().unwrap(), true);
    assert_eq!(file.is_directory().unwrap(), false);
    assert_eq!(file.is_block_device().unwrap(), false);
    assert_eq!(file.is_character_device().unwrap(), false);
    assert_eq!(file.is_pipe().unwrap(), false);
    assert_eq!(file.is_socket().unwrap(), false);
    assert_eq!(file.is_symlink().unwrap(), false);
    assert_eq!(file.contents().unwrap().contains("root"), true);
    assert_eq!(file.owner().unwrap(), "root");

    let group = file.group().unwrap();
    assert!(group == "root" || group == "wheel");

    assert_eq!(file.is_readable().unwrap(), true);
    assert_eq!(file.is_readable_by_owner().unwrap(), true);
    assert_eq!(file.is_readable_by_group().unwrap(), true);
    assert_eq!(file.is_readable_by_others().unwrap(), true);
    assert_eq!(file.is_readable_by_user("root").unwrap(), true);

    assert_eq!(file.is_writable().unwrap(), true);
    assert_eq!(file.is_writable_by_owner().unwrap(), true);
    assert_eq!(file.is_writable_by_group().unwrap(), false);
    assert_eq!(file.is_writable_by_others().unwrap(), false);
    assert_eq!(file.is_writable_by_user("root").unwrap(), true);

    assert_eq!(file.md5sum().unwrap().len(), 32);
    assert_eq!(file.sha256sum().unwrap().len(), 64);

    assert!(file.size().unwrap() > 0);
}

fn test_file_not_exit(s: Specinfra) {
    let file = s.file("file_does_not_exist");
    assert_eq!(file.exist().unwrap(), false);
}

#[cfg(target_os="macos")]
fn test_file_link_on_macos(s: Specinfra) {
    let file = s.file("/etc");
    assert_eq!(file.linked_to().unwrap(), "private/etc");
}

#[cfg(target_os="linux")]
fn test_file_link_on_linux(s: Specinfra) {
    let file = s.file("/var/lock");
    let link = file.linked_to().unwrap();
    assert!(link == "/run/lock" || link == "../run/lock");
}
