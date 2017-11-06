use std::fmt::Debug;

use provider::Output;
use provider::error::Error;
use provider::error::HandleFuncNotDefined;

pub trait InlineProvider: Debug {
    fn is_running(&self, &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_running".to_string(),
        };
        Err(e.into())
    }

    fn is_enabled(&self, &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_enabled".to_string(),
        };
        Err(e.into())
    }

    fn enable(&self, &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "enable".to_string(),
        };
        Err(e.into())
    }

    fn disable(&self, &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "disable".to_string(),
        };
        Err(e.into())
    }

    fn start(&self, &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "start".to_string(),
        };
        Err(e.into())
    }

    fn stop(&self, &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "stop".to_string(),
        };
        Err(e.into())
    }

    fn reload(&self, &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "reload".to_string(),
        };
        Err(e.into())
    }

    fn restart(&self, &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "restart".to_string(),
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

#[cfg(all(feature="inline-systemd", target_os="linux"))]
pub mod systemd;

// Dummy module for not using systemd feature
#[cfg(not(all(feature="inline-systemd", target_os="linux")))]
pub mod _systemd;

#[cfg(not(all(feature="inline-systemd", target_os="linux")))]
pub use self::_systemd as systemd;
