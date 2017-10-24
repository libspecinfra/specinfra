use std::result::Result;

use platform::platform::Platform;
use provider::Output;
use provider::HandleFunc;
use provider;

pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub code: i32,
}

pub trait Backend {
    fn detect_platform(&self) -> Option<Box<Platform>>;
    fn handle(&self, Box<HandleFunc>) -> Result<Output, provider::error::Error>;
    fn run_command(&self, &str) -> Result<CommandResult, error::Error>;
}

// Wrapper struct for FFI
pub struct BackendWrapper {
    pub backend: Box<Backend>,
}

pub mod direct;
pub mod error;

#[cfg(feature="backend-ssh")]
pub mod ssh;

#[cfg(not(feature="backend-ssh"))]
pub mod _ssh;

#[cfg(not(feature="backend-ssh"))]
pub use self::_ssh as ssh;
