use std::error::Error;
use provider::Output;

// See https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6

pub trait InlineProvider {
    fn mode(&self, &str) -> Result<Output, Box<Error>>;
    fn box_clone(&self) -> Box<InlineProvider>;
}

impl Clone for Box<InlineProvider> {
    fn clone(&self) -> Box<InlineProvider> {
        self.box_clone()
    }
}

pub mod posix;
