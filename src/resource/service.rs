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

    pub fn is_running(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_running(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_enabled(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_enabled(self.name))
            .and_then(Output::to_bool)
    }

    pub fn enable(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.enable(self.name))
            .and_then(Output::to_bool)
    }

    pub fn disable(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.disable(self.name))
            .and_then(Output::to_bool)
    }

    pub fn start(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.start(self.name))
            .and_then(Output::to_bool)
    }

    pub fn stop(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.stop(self.name))
            .and_then(Output::to_bool)
    }
}
