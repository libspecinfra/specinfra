use provider::HandleFunc;
use self::shell::ShellProvider;
use self::inline::InlineProvider;

pub struct PackageProvider {
    pub inline: Box<InlineProvider>,
    pub shell: Box<ShellProvider>,
}

impl PackageProvider {
    pub fn is_installed(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_installed(name)),
            shell: Box::new(move |b| s.is_installed(name, b)),
        })
    }
}

pub mod shell;
pub mod inline;
