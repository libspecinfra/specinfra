use std::fmt::Debug;

pub trait BasePlatform: Iterator + Debug {
    fn new() -> Self where Self: Sized;
    fn reset(&mut self);
}
