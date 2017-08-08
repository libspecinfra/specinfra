use platform::platform::Platform;
use platform::base_platform::BasePlatform;
use backend::Backend;
use platform::bsd;

pub struct Detector {
    curr: usize,
    base_platforms: Vec<Box<BasePlatform<Item = Box<Platform>>>>,
}

impl Detector {
    pub fn new() -> Detector {
        let mut p: Vec<Box<BasePlatform<Item = Box<Platform>>>> = Vec::new();
        p.push(Box::new(bsd::Bsd::new()));
        Detector {
            curr: 0,
            base_platforms: p,
        }
    }

    pub fn detect(&mut self, b: &Backend) -> Option<Box<Platform>> {
        self.reset();
        while let Some(p) = self.next() {
            match b.detect(p) {
                Some(x) => return Some(x),
                None => (),
            }
        }
        return None;
    }
}

impl Iterator for Detector {
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

impl Detector {
    fn reset(&mut self) {
        self.curr = 0;
        for p in &mut self.base_platforms {
            p.reset();
        }
    }
}
