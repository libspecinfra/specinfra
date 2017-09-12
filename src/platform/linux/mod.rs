pub mod ubuntu;
use platform::base_platform::BasePlatform;
use platform::platform::Platform;
use platform::linux::ubuntu::Ubuntu;


#[derive(Clone)]
pub struct Linux {
    curr: usize,
    platforms: Vec<Box<Platform>>,
}

impl BasePlatform for Linux {
    fn new() -> Linux {
        Linux {
            curr: 0,
            platforms: vec![Box::new(Ubuntu::new())],
        }
    }

    fn reset(&mut self) {
        self.curr = 0;
    }
}

impl Iterator for Linux {
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
