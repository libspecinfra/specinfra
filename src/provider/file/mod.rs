use provider::HandleFunc;
use provider::file::inline::InlineProvider;
use provider::file::shell::ShellProvider;

pub enum Whom {
    Owner,
    Group,
    Others,
    User(String),
}

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

    pub fn is_character_device(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_character_device(name)),
            shell: Box::new(move |b| s.is_character_device(name, b)),
        })
    }

    pub fn is_pipe(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_pipe(name)),
            shell: Box::new(move |b| s.is_pipe(name, b)),
        })
    }

    pub fn is_socket(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_socket(name)),
            shell: Box::new(move |b| s.is_socket(name, b)),
        })
    }

    pub fn is_symlink(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_symlink(name)),
            shell: Box::new(move |b| s.is_symlink(name, b)),
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

    pub fn contents(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.contents(name)),
            shell: Box::new(move |b| s.contents(name, b)),
        })
    }

    pub fn owner(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.owner(name)),
            shell: Box::new(move |b| s.owner(name, b)),
        })
    }

    pub fn group(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.group(name)),
            shell: Box::new(move |b| s.group(name, b)),
        })
    }

    pub fn linked_to(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.linked_to(name)),
            shell: Box::new(move |b| s.linked_to(name, b)),
        })
    }

    pub fn is_readable(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_readable(name, None)),
            shell: Box::new(move |b| s.is_readable(name, None, b)),
        })
    }

    pub fn is_readable_by_owner(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_readable(name, Some(&Whom::Owner))),
            shell: Box::new(move |b| s.is_readable(name, Some(&Whom::Owner), b)),
        })
    }
}

pub mod inline;
pub mod shell;
