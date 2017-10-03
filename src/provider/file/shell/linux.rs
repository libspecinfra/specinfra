use std::result::Result;

use backend::Backend;
use provider::error::Error;
use provider::Output;
use provider::file::shell::ShellProvider;

#[derive(Clone)]
pub struct Linux;

impl ShellProvider for Linux {
    fn mode(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = format!("stat -c %a {}", name);
        let res = try!(b.run_command(&c));
        let m = try!(i32::from_str_radix(&res, 8));
        Ok(Output::I32(m))
    }

    fn box_clone(&self) -> Box<ShellProvider> {
        Box::new((*self).clone())
    }
}
