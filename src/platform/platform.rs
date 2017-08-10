use std::fmt::Debug;

use provider;
use backend::Backend;

// See https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-trait-object
pub trait Platform: Debug + PlatformClone {
    fn new() -> Self where Self: Sized;

    fn inline_detector(&self) -> Option<Box<Platform>>;

    fn shell_detector(&self, &Backend) -> Option<Box<Platform>>;

    fn get_provider(&self) -> Box<provider::Provider>;
}

pub trait PlatformClone {
    fn clone_box(&self) -> Box<Platform>;
}

impl<T> PlatformClone for T
    where T: 'static + Platform + Clone
{
    fn clone_box(&self) -> Box<Platform> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Platform> {
    fn clone(&self) -> Box<Platform> {
        self.clone_box()
    }
}
