pub extern crate ssh2;

use libc::c_char;
use std::ffi::CStr;

use std::result::Result;
use std::str;
use std::net::TcpStream;
use std::io::prelude::*;
use std::path::Path;

use backend;
use backend::error::Error;
use backend::Backend;
use backend::command::Command;
use backend::command::CommandResult;
use provider;
use provider::Output;
use platform::platform::Platform;
use platform::platforms::Platforms;

pub struct SSH {
    session: ssh2::Session,
    _tcp: TcpStream,
}

#[derive(Clone, Copy, Debug)]
pub struct SSHBuilder<'a> {
    host: Option<&'a str>,
    port: Option<usize>,
    user: Option<&'a str>,
    password: Option<&'a str>,
    key_file: Option<&'a str>,
}

impl<'a> SSHBuilder<'a> {
    pub fn new() -> Self {
        SSHBuilder {
            host: None,
            port: None,
            user: None,
            password: None,
            key_file: None,
        }
    }

    pub fn host(mut self, h: &'a str) -> Self {
        self.host = Some(h);
        self
    }

    pub fn port(mut self, p: usize) -> Self {
        self.port = Some(p);
        self
    }

    pub fn user(mut self, u: &'a str) -> Self {
        self.user = Some(u);
        self
    }

    pub fn password(mut self, p: &'a str) -> Self {
        self.password = Some(p);
        self
    }

    pub fn key_file(mut self, k: &'a str) -> Self {
        self.key_file = Some(k);
        self
    }

    pub fn finalize(self) -> Result<SSH, Error> {
        let host = self.host.unwrap();
        let remote_addr = match self.port {
            Some(p) => host.to_string() + ":" + &p.to_string(),
            None => host.to_string() + ":22",
        };

        let tcp = try!(TcpStream::connect(remote_addr));
        let mut session = ssh2::Session::new().unwrap();
        try!(session.handshake(&tcp));

        let user = self.user.unwrap();

        match self.key_file {
            Some(k) => try!(session.userauth_pubkey_file(user, None, Path::new(k), None)),
            None => {
                match self.password {
                    Some(p) => try!(session.userauth_password(user, p)),
                    None => try!(session.userauth_agent(user)),
                }
            }
        }

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

    fn handle(&self,
              handle_func: Box<provider::HandleFunc>)
              -> Result<Output, provider::error::Error> {
        (handle_func.shell)(self)
    }

    fn run_command(&self, c: Command) -> Result<CommandResult, backend::error::Error> {
        let mut chan = try!(self.session.channel_session());
        chan.exec(&c.string).unwrap();

        let mut stdout = String::new();
        chan.read_to_string(&mut stdout).unwrap();

        let mut stderr = String::new();
        chan.stderr().read_to_string(&mut stderr).unwrap();

        let code = try!(chan.exit_status());

        let success = if code == 0 { true } else { false };

        let res = CommandResult {
            stdout: stdout.trim().to_string(),
            stderr: stderr.trim().to_string(),
            code: code,
            success: success,
        };

        Ok(res)
    }
}

// Wrapper functions for FFI

use backend::BackendWrapper;

#[no_mangle]
pub extern "C" fn backend_ssh_builder_new<'a>(host: *const c_char) -> *mut SSHBuilder<'a> {
    let host = unsafe {
        assert!(!host.is_null());
        CStr::from_ptr(host)
    };
    let host_str = host.to_str().unwrap();

    let b = SSHBuilder::new().host(host_str);
    Box::into_raw(Box::new(b))
}

#[no_mangle]
pub extern "C" fn backend_ssh_builder_free(ptr: *mut SSHBuilder) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn backend_ssh_builder_user(ptr: *mut SSHBuilder,
                                           u: *const c_char)
                                           -> *mut SSHBuilder {
    let b = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let user = unsafe {
        assert!(!u.is_null());
        CStr::from_ptr(u)
    };
    let user_str = user.to_str().unwrap();
    Box::into_raw(Box::new(b.user(user_str)))
}

#[no_mangle]
pub extern "C" fn backend_ssh_builder_password(ptr: *mut SSHBuilder,
                                               p: *const c_char)
                                               -> *mut SSHBuilder {
    let b = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let password = unsafe {
        assert!(!p.is_null());
        CStr::from_ptr(p)
    };
    let password_str = password.to_str().unwrap();
    Box::into_raw(Box::new(b.password(password_str)))
}

#[no_mangle]
pub extern "C" fn backend_ssh_builder_key_file(ptr: *mut SSHBuilder,
                                               k: *const c_char)
                                               -> *mut SSHBuilder {
    let b = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let key_file = unsafe {
        assert!(!k.is_null());
        CStr::from_ptr(k)
    };
    let key_file_str = key_file.to_str().unwrap();
    Box::into_raw(Box::new(b.key_file(key_file_str)))
}

#[no_mangle]
pub extern "C" fn backend_ssh_builder_port(ptr: *mut SSHBuilder, p: usize) -> *mut SSHBuilder {
    let b = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    Box::into_raw(Box::new(b.port(p)))
}

#[no_mangle]
pub extern "C" fn backend_ssh_builder_finalize(ptr: *mut SSHBuilder) -> *mut BackendWrapper {
    let b = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let s = BackendWrapper(Box::new(b.finalize().unwrap()));
    Box::into_raw(Box::new(s))
}

#[no_mangle]
pub extern "C" fn backend_ssh_free(ptr: *mut SSH) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
