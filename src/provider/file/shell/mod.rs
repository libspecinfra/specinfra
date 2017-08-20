use std::error::Error;
use provider::Output;
use backend::Backend;

// See https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6

pub trait ShellProvider {
    fn mode(&self, &str, &Backend) -> Result<Output, Box<Error>>;
    fn box_clone(&self) -> Box<ShellProvider>;
}

impl Clone for Box<ShellProvider> {
    fn clone(&self) -> Box<ShellProvider> {
        self.box_clone()
    }
}

pub mod bsd;
