use backend::Backend;
use provider;
use std::result::Result;
use provider::Output;
use platform::Platform;
use std::error::Error;

#[derive(Debug)]
pub struct Direct;

impl Backend for Direct {
    fn new() -> Direct {
        Direct
    }

    fn detect(&self, p: Box<Platform>) -> Option<Box<Platform>> {
        p.detect_inline()
    }

    fn handle(&self, handle_func: Box<provider::HandleFunc>) -> Result<Output, Box<Error>> {
        (handle_func.inline)()
    }
}
