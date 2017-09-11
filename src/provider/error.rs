use std::error;
use std::fmt;
use std::io;
use std::num;

use nix;

use backend;
use provider::OutputError;

#[derive(Debug)]
pub enum Error {
    HandlerFuncNotFound(HandlerFuncNotFound),
    Nix(nix::Error),
    Io(io::Error),
    String(StringError),
    ParseInt(num::ParseIntError),
    Output(OutputError),
    Backend(backend::error::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HandlerFuncNotFound(ref err) => err.description(),
            Error::Nix(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::String(ref err) => err.description(),
            Error::ParseInt(ref err) => err.description(),
            Error::Output(ref err) => err.description(),
            Error::Backend(ref err) => err.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::HandlerFuncNotFound(ref err) => err.fmt(f),
            Error::Nix(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::String(ref err) => err.fmt(f),
            Error::ParseInt(ref err) => err.fmt(f),
            Error::Output(ref err) => err.fmt(f),
            Error::Backend(ref err) => err.fmt(f),
        }
    }
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Error {
        Error::Nix(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<OutputError> for Error {
    fn from(err: OutputError) -> Error {
        Error::Output(err)
    }
}

impl From<backend::error::Error> for Error {
    fn from(err: backend::error::Error) -> Error {
        Error::Backend(err)
    }
}

#[derive(Debug)]
pub struct HandlerFuncNotFound;

impl error::Error for HandlerFuncNotFound {
    fn description(&self) -> &str {
        "HandlerFunc not found"
    }
}

impl fmt::Display for HandlerFuncNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HandlerFunc not found")
    }
}

impl From<HandlerFuncNotFound> for Error {
    fn from(err: HandlerFuncNotFound) -> Error {
        Error::HandlerFuncNotFound(err)
    }
}

#[derive(Debug)]
pub struct StringError {
    pub string: String,
}

impl error::Error for StringError {
    fn description(&self) -> &str {
        &self.string
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl From<StringError> for Error {
    fn from(err: StringError) -> Error {
        Error::String(err)
    }
}
