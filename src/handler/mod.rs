use std::fmt::Debug;
use platform::platform::Platform;

pub trait Handler: Debug {
    fn detect_platform(&self, f: &Fn() -> Option<Box<Platform>>) -> Option<Box<Platform>>;
}

pub mod inline;
