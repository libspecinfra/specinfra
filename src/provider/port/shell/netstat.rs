use std::result::Result;

use backend::Backend;
use backend::command::Command;
use provider::error::Error;
use provider::Output;
use provider::port::shell::ShellProvider;

#[derive(Clone, Debug)]
pub struct Netstat;

impl ShellProvider for Netstat {
    fn is_listening(&self, number: usize, b: &Backend) -> Result<Output, Error> {
        let mut c = Command::new("netstat -tunl");
        c.pipe(&format!("grep -- :{}", number));

        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn box_clone(&self) -> Box<ShellProvider> {
        Box::new((*self).clone())
    }
}
