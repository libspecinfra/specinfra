use std::fmt::Debug;
use downcast_rs::Downcast;
use platform::Platform;

pub trait Handler: Debug + Downcast {
    fn detect_platform(&self, f: &Fn() -> Option<Box<Platform>>) -> Option<Box<Platform>>;
}

impl_downcast!(Handler);

pub mod inline;
