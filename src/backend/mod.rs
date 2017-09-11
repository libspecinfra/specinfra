pub mod direct;
pub mod ssh;
pub mod error;

use std::result::Result;

use platform::platform::Platform;
use provider::Output;
use provider::HandleFunc;
use provider;

pub trait Backend {
    fn detect_platform(&self) -> Option<Box<Platform>>;
    fn handle(&self, Box<HandleFunc>) -> Result<Output, provider::error::Error>;
    fn run_command(&self, &str) -> Result<String, error::Error>;
}

// Wrapper struct for FFI
pub struct BackendWrapper {
    pub backend: Box<Backend>,
}
