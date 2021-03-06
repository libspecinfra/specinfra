use std::fmt::Debug;

use provider::Output;
use provider::error::Error;
use provider::error::HandleFuncNotDefined;

pub trait InlineProvider: Debug {
    fn is_listening(&self, usize) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_listening".to_string(),
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
