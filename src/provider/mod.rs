pub mod file;

use std::result::Result;
use std::error::Error;
use std::fmt;

use backend::Backend;

#[derive(Debug)]
pub struct Provider {
    pub file: Box<file::File>,
}

pub struct HandleFunc {
    pub inline: Option<Box<Fn() -> Result<Output, Box<Error>>>>,
    pub shell: Option<Box<Fn(&Backend) -> Result<Output, Box<Error>>>>,
}

pub enum Output {
    U32(u32),
    Text(String),
}

#[derive(Debug)]
pub struct OutputError;

impl Error for OutputError {
    fn description(&self) -> &str {
        "Output error"
    }
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Output error")
    }
}

impl Output {
    pub fn to_u32(o: Output) -> Result<u32, Box<Error>> {
        match o {
            Output::U32(u) => Ok(u),
            _ => Err(From::from(OutputError)),
        }
    }
}
