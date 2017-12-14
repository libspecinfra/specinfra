use provider::HandleFunc;
use provider::port::inline::InlineProvider;
use provider::port::shell::ShellProvider;

pub struct PortProvider {
    pub inline: Box<InlineProvider>,
    pub shell: Box<ShellProvider>,
}

impl PortProvider {
    pub fn is_listening(&self, number: usize) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_listening(number)),
            shell: Box::new(move |b| s.is_listening(number, b)),
        })

    }
}

pub mod inline;
pub mod shell;
