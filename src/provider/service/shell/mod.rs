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
}
