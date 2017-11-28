use std::fmt::Debug;

use backend::Backend;
use provider::error::Error;
use provider::error::HandleFuncNotDefined;
use provider::Output;

pub trait ShellProvider: Debug {
    fn is_installed(&self, &str, Option<&str>, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_installed".to_string(),
        };
        Err(e.into())
    }

    fn version(&self, &str, Option<&str>, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "version".to_string(),
        };
        Err(e.into())
    }

    fn remove(&self, &str, Option<&str>, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "remove".to_string(),
        };
        Err(e.into())
    }

    fn install(&self, &str, Option<&str>, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "install".to_string(),
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
pub mod yum;
