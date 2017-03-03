use std::fmt::Debug;

pub trait Platforms: Iterator + Debug {
    fn new() -> Self where Self: Sized;
    fn reset(&mut self);
}
