use std::fmt::Debug;

use backend::Backend;
use provider::error::Error;
use provider::error::HandleFuncNotDefined;
use provider::Output;

pub trait ShellProvider: Debug {
    fn is_listening(&self, usize, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_listening".to_string(),
        };
        Err(e.into())
    }

    fn box_clone(&self) -> Box<ShellProvider>;
}

impl Clone for Box<ShellProvider> {
    fn clone(&self) -> Box<ShellProvider> {
        self.box_clone()
    }
}

pub mod null;
pub mod netstat;
