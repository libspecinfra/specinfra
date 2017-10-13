use std::fmt::Debug;

use backend::Backend;
use provider::error::Error;
use provider::error::HandleFuncNotDefined;
use provider::Output;

pub trait ShellProvider: Debug {
    fn is_running(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_running".to_string(),
        };
        Err(From::from(e))
    }

    fn is_enabled(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_enabled".to_string(),
        };
        Err(From::from(e))
    }

    fn enable(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "enable".to_string(),
        };
        Err(From::from(e))
    }

    fn disable(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "disable".to_string(),
        };
        Err(From::from(e))
    }

    fn box_clone(&self) -> Box<ShellProvider>;
}

impl Clone for Box<ShellProvider> {
    fn clone(&self) -> Box<ShellProvider> {
        self.box_clone()
    }
}

pub mod null;
