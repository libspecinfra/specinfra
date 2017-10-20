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

    pub fn is_enabled(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_enabled(name)),
            shell: Box::new(move |b| s.is_enabled(name, b)),
        })
    }

    pub fn enable(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.enable(name)),
            shell: Box::new(move |b| s.enable(name, b)),
        })
    }

    pub fn disable(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.disable(name)),
            shell: Box::new(move |b| s.disable(name, b)),
        })
    }

    pub fn start(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.start(name)),
            shell: Box::new(move |b| s.start(name, b)),
        })
    }

    pub fn stop(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.stop(name)),
            shell: Box::new(move |b| s.stop(name, b)),
        })
    }

    pub fn reload(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.reload(name)),
            shell: Box::new(move |b| s.reload(name, b)),
        })
    }

    pub fn restart(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.restart(name)),
            shell: Box::new(move |b| s.restart(name, b)),
        })
    }
}

pub mod error;
pub mod inline;
pub mod shell;
