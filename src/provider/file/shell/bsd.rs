use std::result::Result;

use backend::Backend;
use provider::error::Error;
use provider::Output;
use provider::file::shell::ShellProvider;
use provider::file::shell::unix::Unix;

#[derive(Clone, Debug)]
pub struct Bsd;

impl ShellProvider for Bsd {
    fn mode(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = format!("stat -f%Lp {}", name);
        let res = try!(b.run_command(&c));
        let m = try!(i32::from_str_radix(&res.stdout, 8));
        Ok(Output::I32(m))
    }

    fn exist(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        Unix.exist(name, b)
    }

    fn is_file(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        Unix.is_file(name, b)
    }

    fn is_directory(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        Unix.is_directory(name, b)
    }

    fn is_block_device(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        Unix.is_block_device(name, b)
    }

    fn is_character_device(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        Unix.is_character_device(name, b)
    }

    fn is_pipe(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        Unix.is_pipe(name, b)
    }

    fn is_socket(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        Unix.is_socket(name, b)
    }

    fn is_symlink(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        Unix.is_symlink(name, b)
    }

    fn contents(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        Unix.contents(name, b)
    }

    fn box_clone(&self) -> Box<ShellProvider> {
        Box::new((*self).clone())
    }

    fn owner(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = format!("stat -f%Su {}", name);
        let res = try!(b.run_command(&c));
        Ok(Output::Text(res.stdout))
    }

    fn group(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = format!("stat -f%Sg {}", name);
        let res = try!(b.run_command(&c));
        Ok(Output::Text(res.stdout))
    }
}
