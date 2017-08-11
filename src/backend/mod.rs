pub mod direct;

use std::fmt::Debug;
use std::result::Result;
use std::error::Error;

use platform::platform::Platform;
use provider::HandleFunc;
use provider::Output;

pub trait Backend: Debug {
    fn new() -> Self where Self: Sized;
    fn detect_platform(&self) -> Option<Box<Platform>>;
    fn handle(&self, Box<HandleFunc>) -> Result<Output, Box<Error>>;
    fn run_command(&self, &str) -> Result<String, Box<Error>>;
}

// Wrapper struct for FFI
#[derive(Debug)]
pub struct BackendWrapper {
    pub backend: Box<Backend>,
}
