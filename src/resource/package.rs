use backend::Backend;
use provider::error;
use provider::Output;
use provider::package::PackageProvider;

pub struct Package<'a> {
    name: &'static str,
    backend: &'a Backend,
    provider: &'a PackageProvider,
}

impl<'a> Package<'a> {
    pub fn new(n: &'static str, b: &'a Backend, p: &'a PackageProvider) -> Package<'a> {
        Package {
            name: n,
            backend: b,
            provider: p,
        }
    }

    pub fn is_installed(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_installed(self.name))
            .and_then(Output::to_bool)
    }
}
