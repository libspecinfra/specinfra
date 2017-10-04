use platform::base_platform::BasePlatform;
use platform::platform::Platform;
use platform::linux::redhat::RedHat;
use platform::linux::ubuntu::Ubuntu;


#[derive(Clone)]
pub struct Linux {
    curr: usize,
    platforms: Vec<Box<Platform>>,
}

impl BasePlatform for Linux {
    fn new() -> Linux {
        let mut p: Vec<Box<Platform>> = Vec::new();
        p.push(Box::new(Ubuntu::new()));
        p.push(Box::new(RedHat::new()));

        Linux {
            curr: 0,
            platforms: p,
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

pub mod redhat;
pub mod ubuntu;
