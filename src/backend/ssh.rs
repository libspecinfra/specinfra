extern crate ssh2;

use std::result::Result;
use std::error::Error;
use std::str;
use std::net::TcpStream;
use std::env;
use std::io::prelude::*;

use SpecinfraError;
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

        Err(Box::new(SpecinfraError))
    }

    fn run_command(&self, c: &str) -> Result<String, Box<Error>> {
        let mut chan = try!(self.session.channel_session());
        chan.exec(c).unwrap();
        let mut s = String::new();
        chan.read_to_string(&mut s).unwrap();
        Ok(s.trim().to_string())
    }
}
