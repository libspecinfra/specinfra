use provider::service::inline::InlineProvider;
use provider::service::shell::ShellProvider;

pub struct ServiceProvider {
    pub inline: Box<InlineProvider>,
    pub shell: Box<ShellProvider>,
}

pub mod inline;
pub mod shell;
