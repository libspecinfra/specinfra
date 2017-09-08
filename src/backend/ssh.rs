extern crate ssh2;

use libc::c_char;
use std::ffi::CStr;

use std::result::Result;
use std::error::Error;
use std::str;
use std::net::TcpStream;
use std::env;
use std::io::prelude::*;

use backend::Backend;
use provider;
use provider::Output;
use platform::platform::Platform;
use platform::platforms::Platforms;

pub struct SSH {
    session: ssh2::Session,
    _tcp: TcpStream,
}

pub struct SSHBuilder {
    hostname: Option<String>,
}

impl SSHBuilder {
    pub fn new() -> Self {
        SSHBuilder { hostname: None }
    }

    pub fn hostname(mut self, h: &str) -> Self {
        self.hostname = Some(h.to_string());
        self
    }

    pub fn finalize(self) -> Result<SSH, Box<Error>> {
        let hostname = self.hostname.unwrap();
        let remote_addr = hostname + ":22";
        let tcp = try!(TcpStream::connect(remote_addr));
        let mut session = ssh2::Session::new().unwrap();
        try!(session.handshake(&tcp));
        let user = try!(env::var("USER"));
        try!(session.userauth_agent(&user));

        let ssh = SSH {
            session: session,
            _tcp: tcp,
        };
        Ok(ssh)
    }
}

impl Backend for SSH {
    fn detect_platform(&self) -> Option<Box<Platform>> {
        let mut platforms = Platforms::new();
        while let Some(p) = platforms.next() {
            match p.shell_detector(self) {
                Some(m) => return Some(m),
                None => (),
            }
        }
        None
    }

    fn handle(&self, handle_func: Box<provider::HandleFunc>) -> Result<Output, Box<Error>> {
        match handle_func.shell {
            Some(f) => return f(self),
            None => {}
        };

        Err(Box::new(provider::error::Error))
    }

    fn run_command(&self, c: &str) -> Result<String, Box<Error>> {
        let mut chan = try!(self.session.channel_session());
        chan.exec(c).unwrap();
        let mut s = String::new();
        chan.read_to_string(&mut s).unwrap();
        Ok(s.trim().to_string())
    }
}

// Wrapper functions for FFI

use backend::BackendWrapper;

#[no_mangle]
pub extern "C" fn backend_ssh_new(host: *const c_char) -> *mut BackendWrapper {
    let host = unsafe {
        assert!(!host.is_null());
        CStr::from_ptr(host)
    };
    let host_str = host.to_str().unwrap();

    let s = SSHBuilder::new().hostname(host_str).finalize().unwrap();
    let b = BackendWrapper { backend: Box::new(s) };
    Box::into_raw(Box::new(b))
}

#[no_mangle]
pub extern "C" fn backend_ssh_free(ptr: *mut BackendWrapper) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
