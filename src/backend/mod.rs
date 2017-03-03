pub mod direct;

use std::fmt::Debug;
use platform::Platform;
use provider::HandleFunc;
use provider::Output;
use std::result::Result;
use std::error::Error;

pub trait Backend: Debug {
    fn new() -> Self where Self: Sized;
    fn detect(&self, Box<Platform>) -> Option<Box<Platform>>;
    fn handle(&self, Box<HandleFunc>) -> Result<Output, Box<Error>>;
}
