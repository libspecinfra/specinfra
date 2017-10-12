use provider::HandleFunc;
use provider::service::inline::InlineProvider;
use provider::service::shell::ShellProvider;

pub struct ServiceProvider {
    pub inline: Box<InlineProvider>,
    pub shell: Box<ShellProvider>,
}

impl ServiceProvider {
    pub fn new(i: Box<InlineProvider>, s: Box<ShellProvider>) -> ServiceProvider {
        ServiceProvider {
            inline: i,
            shell: s,
        }
    }

    pub fn is_running(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_running(name)),
            shell: Box::new(move |b| s.is_running(name, b)),
        })
    }
}

pub mod error;
pub mod inline;
pub mod shell;
