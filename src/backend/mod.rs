pub mod direct;
pub mod ssh;

use std::result::Result;
use std::error::Error;

use platform::platform::Platform;
use provider::HandleFunc;
use provider::Output;

pub trait Backend {
    fn detect_platform(&self) -> Option<Box<Platform>>;
    fn handle(&self, Box<HandleFunc>) -> Result<Output, Box<Error>>;
    fn run_command(&self, &str) -> Result<String, Box<Error>>;
}

// Wrapper struct for FFI
pub struct BackendWrapper {
    pub backend: Box<Backend>,
}
