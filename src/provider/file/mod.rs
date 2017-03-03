use std::fmt::Debug;

use provider::HandleFunc;

pub trait File: Debug {
    fn mode(&self, &'static str) -> Box<HandleFunc>;
}

pub mod posix;
