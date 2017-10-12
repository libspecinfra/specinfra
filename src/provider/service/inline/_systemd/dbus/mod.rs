use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Error;

impl error::Error for Error {
    fn description(&self) -> &str {
        "dummy error"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "dummy error")
    }
}

pub mod arg;
