use std::error;
use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug)]
pub enum Error {
    Detect(DetectError),
    ParseFloat(ParseFloatError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Detect(ref err) => err.description(),
            Error::ParseFloat(ref err) => err.description(),
        }
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Detect(ref err) => err.fmt(f),
            Error::ParseFloat(ref err) => err.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct DetectError {
    pub message: String,
}

impl error::Error for DetectError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for DetectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<DetectError> for Error {
    fn from(err: DetectError) -> Error {
        Error::Detect(err)
    }
}
