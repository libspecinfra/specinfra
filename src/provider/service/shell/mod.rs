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
        Err(e.into())
    }

    fn is_enabled(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_enabled".to_string(),
        };
        Err(e.into())
    }

    fn enable(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "enable".to_string(),
        };
        Err(e.into())
    }

    fn disable(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "disable".to_string(),
        };
        Err(e.into())
    }

    fn start(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "start".to_string(),
        };
        Err(e.into())
    }

    fn stop(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "stop".to_string(),
        };
        Err(e.into())
    }

    fn reload(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "reload".to_string(),
        };
        Err(e.into())
    }

    fn restart(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "restart".to_string(),
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
pub mod systemd;
pub mod ubuntu_init;
pub mod sysvinit;
