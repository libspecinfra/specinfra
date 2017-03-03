use handler::Handler;
use platform::Platform;

#[derive(Debug)]
pub struct Inline;

impl Handler for Inline {
    fn detect_platform(&self, f: &Fn() -> Option<Box<Platform>>) -> Option<Box<Platform>> {
        f()
    }
}
