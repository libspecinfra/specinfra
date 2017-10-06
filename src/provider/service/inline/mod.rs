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
        Err(From::from(e))
    }
}
