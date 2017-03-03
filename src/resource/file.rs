use backend::Backend;
use provider::Provider;
use provider::Output;
use std::error::Error;

#[derive(Debug)]
pub struct File<'a> {
    name: &'static str,
    backend: &'a Backend,
    provider: &'a Provider,
}

impl<'a> File<'a> {
    pub fn new(n: &'static str, b: &'a Backend, p: &'a Provider) -> File<'a> {
        File {
            name: n,
            backend: b,
            provider: p,
        }
    }

    pub fn mode(&self) -> Result<u32, Box<Error>> {
        self.backend
            .handle(self.provider.file.mode(self.name))
            .and_then(Output::to_u32)
    }
}
