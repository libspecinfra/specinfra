use platform::platform::Platform;
use platform::base_platform::BasePlatform;
use platform::bsd;

pub struct Platforms {
    curr: usize,
    base_platforms: Vec<Box<BasePlatform<Item = Box<Platform>>>>,
}

impl Platforms {
    pub fn new() -> Platforms {
        let mut p: Vec<Box<BasePlatform<Item = Box<Platform>>>> = Vec::new();
        p.push(Box::new(bsd::Bsd::new()));
        Platforms {
            curr: 0,
            base_platforms: p,
        }
    }
}

impl Iterator for Platforms {
    type Item = Box<Platform>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.base_platforms.len() {
            match self.base_platforms[self.curr].next() {
                Some(x) => Some(x),
                None => {
                    self.curr += 1;
                    self.next()
                }
            }
        } else {
            None
        }
    }
}

impl Platforms {
    fn reset(&mut self) {
        self.curr = 0;
        for p in &mut self.base_platforms {
            p.reset();
        }
    }
}
