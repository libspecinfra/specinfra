use backend::Backend;
use provider::error;
use provider::Output;
use provider::port::PortProvider;

pub struct Port<'a> {
    number: usize,
    backend: &'a Backend,
    provider: &'a PortProvider,
}

impl<'a> Port<'a> {
    pub fn new(n: usize, b: &'a Backend, p: &'a PortProvider) -> Port<'a> {
        Port {
            number: n,
            backend: b,
            provider: p,
        }
    }

    pub fn is_listening(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_listening(self.number))
            .and_then(Output::to_bool)
    }
}
