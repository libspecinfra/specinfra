use std::fmt;
use std::error;

#[derive(Debug)]
pub struct TypeMismatchError;

impl error::Error for TypeMismatchError {
    fn description(&self) -> &str {
        "dummy error"
    }
}

impl fmt::Display for TypeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "dummy error")
    }
}
