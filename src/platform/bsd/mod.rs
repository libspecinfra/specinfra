pub mod darwin;

use platform::base_platform::BasePlatform;
use platform::platform::Platform;
use platform::bsd::darwin::Darwin;

#[derive(Clone, Debug)]
pub struct Bsd {
    curr: usize,
    platforms: Vec<Box<Platform>>,
}

impl BasePlatform for Bsd {
    fn new() -> Bsd {
        Bsd {
            curr: 0,
            platforms: vec![Box::new(Darwin::new())],
        }
    }

    fn reset(&mut self) {
        self.curr = 0;
    }
}

impl Iterator for Bsd {
    type Item = Box<Platform>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.platforms.len() {
            let curr = self.curr;
            self.curr += 1;
            Some(self.platforms[curr].clone())
        } else {
            None
        }
    }
}
