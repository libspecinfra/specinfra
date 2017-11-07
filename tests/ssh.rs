#![cfg(feature="test-ssh")]
extern crate specinfra;

use specinfra::backend::ssh::SSHBuilder;

#[test]
fn test_with_agent() {
    let b = SSHBuilder::new()
        .host("localhost")
        .user("mizzy")
        .finalize()
        .unwrap();

    let s = specinfra::new(&b).unwrap();
    let f = s.file("/etc/passwd");
    assert_eq!(f.mode().unwrap(), 0o644);
}

#[test]
fn test_with_port() {
    let b = SSHBuilder::new()
        .host("localhost")
        .port(22)
        .user("mizzy")
        .finalize()
        .unwrap();

    let s = specinfra::new(&b).unwrap();
    let f = s.file("/etc/passwd");
    assert_eq!(f.mode().unwrap(), 0o644);
}

#[test]
fn test_with_key_file() {
    let b = SSHBuilder::new()
        .host("localhost")
        .user("mizzy")
        .key_file("/Users/mizzy/.ssh/id_rsa")
        .finalize()
        .unwrap();

    let s = specinfra::new(&b).unwrap();
    let f = s.file("/etc/passwd");
    assert_eq!(f.mode().unwrap(), 0o644);
}

#[test]
fn test_with_password() {
    let b = SSHBuilder::new()
        .host("localhost")
        .user("mizzy")
        .password("******")
        .finalize()
        .unwrap();

    let s = specinfra::new(&b).unwrap();
    let f = s.file("/etc/passwd");
    assert_eq!(f.mode().unwrap(), 0o644);
}
