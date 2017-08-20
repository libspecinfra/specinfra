use std::result::Result;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::error::Error;

use provider::Output;
use provider::file::inline::InlineProvider;

#[derive(Clone)]
pub struct Posix;

impl InlineProvider for Posix {
    fn mode(&self, name: &str) -> Result<Output, Box<Error>> {
        let res = try!(fs::metadata(name).map(|m| Output::U32(m.permissions().mode())));
        Ok(res)
    }

    fn box_clone(&self) -> Box<InlineProvider> {
        Box::new((*self).clone())
    }
}
