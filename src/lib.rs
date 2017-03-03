#[macro_use]
extern crate downcast_rs;
extern crate uname;

use std::error::Error;
use std::fmt;

use backend::Backend;
use platform::detector::Detector;
use platform::Platform;
use resource::file::File;
use provider::Provider;

pub mod backend;
pub mod platform;
pub mod handler;
pub mod resource;
pub mod provider;

pub struct Specinfra<'a> {
    pub backend: &'a Backend,
    pub platform: Box<Platform>,
    pub provider: Box<Provider>,
}

#[derive(Debug)]
pub struct SpecinfraError;

impl Error for SpecinfraError {
    fn description(&self) -> &str {
        "error"
    }
}

impl fmt::Display for SpecinfraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error")
    }
}

pub fn new(b: &Backend) -> Result<Specinfra, Box<Error>> {
    let mut d = Detector::new();
    let p = try!(d.detect(b).ok_or(SpecinfraError));
    let provider = p.get_provider();
    Ok(Specinfra {
        backend: b,
        platform: p,
        provider: provider,
    })
}

impl<'b> Specinfra<'b> {
    pub fn file(&self, name: &'static str) -> File {
        File::new(name, self.backend, &self.provider)
    }
}
