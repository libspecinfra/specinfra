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
