use platform;
use provider;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Platform(platform::error::Error),
    Provider(provider::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Platform(ref err) => err.fmt(f),
            Error::Provider(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Platform(ref err) => err.description(),
            Error::Provider(ref err) => err.description(),
        }
    }
}

impl From<platform::error::Error> for Error {
    fn from(err: platform::error::Error) -> Error {
        Error::Platform(err)
    }
}

impl From<provider::error::Error> for Error {
    fn from(err: provider::error::Error) -> Error {
        Error::Provider(err)
    }
}
