pub extern crate ssh2;

use libc::{c_char,c_uint};
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

#[repr(C)]
pub struct SSHInterface {
    hostname: *const c_char,
    user: *const c_char,
    password: *const c_char,
    key_file: *const c_char,
    port: *const c_uint,
}

pub struct SSHBuilder {
    hostname: String,
    user: String,
    password: String,
    key_file: String,
    port: u32,
}

impl SSHBuilder {
    pub fn new() -> Self {
        SSHBuilder {
            hostname: "".to_string(),
            user: "".to_string(),
            password: "".to_string(),
            key_file: "".to_string(),
            port: 22
        }
    }

    pub fn set_target(mut self, s: *const SSHInterface) -> Self {
        unsafe {
            let t = &*s;
            self.hostname = CStr::from_ptr(&*t.hostname).to_string_lossy().into_owned();
            self.user = CStr::from_ptr(&*t.user).to_string_lossy().into_owned();
            self.password = CStr::from_ptr(&*t.password).to_string_lossy().into_owned();
            self.key_file = CStr::from_ptr(&*t.key_file).to_string_lossy().into_owned();
        }
        self
    }

    pub fn finalize(self) -> Result<SSH, Error> {
        let hostname = self.hostname;
        let port = self.port.to_string();
        let remote_addr = hostname + ":" + &port;
        let user = self.user;
        let password = self.password;
        let key_file = self.key_file;
        let tcp = try!(TcpStream::connect(remote_addr));
        let mut session = ssh2::Session::new().unwrap();
        try!(session.handshake(&tcp));

        if &password != "" {
            try!(session.userauth_password(&user, &password));
        } else if &key_file != "" {
            try!(session.userauth_pubkey_file(&user, None, Path::new(&key_file), None));
        } else {
            try!(session.userauth_agent(&user));
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
pub extern "C" fn backend_ssh_new(s: *const SSHInterface) -> *mut BackendWrapper {
    let s = SSHBuilder::new().set_target(s).finalize().unwrap();
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
