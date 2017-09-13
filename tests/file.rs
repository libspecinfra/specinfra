extern crate specinfra;

use specinfra::backend;

#[test]
#[cfg(any(target_os="macos", target_os="linux"))]
fn file_resource() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
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
}
