use backend::Backend;
use provider::error;
use provider::Output;
use provider::service::ServiceProvider;

pub struct Service<'a> {
    name: &'static str,
    backend: &'a Backend,
    provider: &'a ServiceProvider,
    error: Option<error::Error>,
}

impl<'a> Service<'a> {
    pub fn new(n: &'static str, b: &'a Backend, p: &'a ServiceProvider) -> Service<'a> {
        Service {
            name: n,
            backend: b,
            provider: p,
            error: None,
        }
    }
}
