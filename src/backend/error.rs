extern crate ssh2;

use std::error;
use std::string;
use std::fmt;
use std::io;
use std::env;

#[derive(Debug)]
pub enum Error {
    FromUtf8(string::FromUtf8Error),
    Io(io::Error),
    Ssh(ssh2::Error),
    Env(env::VarError),
    Command(CommandError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::FromUtf8(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::Ssh(ref err) => err.description(),
            Error::Env(ref err) => err.description(),
            Error::Command(ref err) => err.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::FromUtf8(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::Ssh(ref err) => err.fmt(f),
            Error::Env(ref err) => err.fmt(f),
            Error::Command(ref err) => err.fmt(f),
        }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::FromUtf8(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<ssh2::Error> for Error {
    fn from(err: ssh2::Error) -> Error {
        Error::Ssh(err)
    }
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error::Env(err)
    }
}

impl From<CommandError> for Error {
    fn from(err: CommandError) -> Error {
        Error::Command(err)
    }
}

#[derive(Debug)]
pub struct CommandError {
    pub code: i32,
    pub message: String,
}

impl error::Error for CommandError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
