pub trait BasePlatform: Iterator {
    fn new() -> Self where Self: Sized;
    fn reset(&mut self);
}
