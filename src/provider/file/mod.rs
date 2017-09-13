use provider::HandleFunc;
use provider::file::inline::InlineProvider;
use provider::file::shell::ShellProvider;

pub struct FileProvider {
    pub inline: Box<InlineProvider>,
    pub shell: Box<ShellProvider>,
}

impl FileProvider {
    pub fn new(i: Box<InlineProvider>, s: Box<ShellProvider>) -> FileProvider {
        FileProvider {
            inline: i,
            shell: s,
        }
    }

    pub fn mode(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.mode(name)),
            shell: Box::new(move |b| s.mode(name, b)),
        })
    }

    pub fn is_file(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_file(name)),
            shell: Box::new(move |b| s.is_file(name, b)),
        })
    }

    pub fn is_directory(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_directory(name)),
            shell: Box::new(move |b| s.is_directory(name, b)),
        })
    }

    pub fn is_block_device(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_block_device(name)),
            shell: Box::new(move |b| s.is_block_device(name, b)),
        })
    }

    pub fn exist(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.exist(name)),
            shell: Box::new(move |b| s.exist(name, b)),
        })
    }
}

pub mod inline;
pub mod shell;
