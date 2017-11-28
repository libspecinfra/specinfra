use std::fmt::Debug;

use provider::Output;
use provider::error::Error;
use provider::error::HandleFuncNotDefined;

pub trait InlineProvider: Debug {
    fn is_installed(&self, &str, Option<&str>) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_installed".to_string(),
        };
        Err(e.into())
    }

    fn version(&self, &str, Option<&str>) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "version".to_string(),
        };
        Err(e.into())
    }

    fn remove(&self, &str, Option<&str>) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "remove".to_string(),
        };
        Err(e.into())
    }

    fn install(&self, &str, Option<&str>) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "install".to_string(),
        };
        Err(e.into())
    }

    fn box_clone(&self) -> Box<InlineProvider>;
}

impl Clone for Box<InlineProvider> {
    fn clone(&self) -> Box<InlineProvider> {
        self.box_clone()
    }
}

pub mod null;
