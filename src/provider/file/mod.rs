use provider::HandleFunc;
use provider::file::inline::InlineProvider;
use provider::file::shell::ShellProvider;

pub struct FileProvider {
    pub inline: Option<Box<InlineProvider>>,
    pub shell: Option<Box<ShellProvider>>,
}

impl FileProvider {
    pub fn new(i: Option<Box<InlineProvider>>, s: Option<Box<ShellProvider>>) -> FileProvider {
        FileProvider {
            inline: i,
            shell: s,
        }
    }

    pub fn mode(&self, name: &'static str) -> Box<HandleFunc> {
        let mut handle_func = HandleFunc {
            inline: None,
            shell: None,
        };

        handle_func.inline = match self.inline {
            Some(ref i) => {
                let c = i.clone();
                Some(Box::new(move || c.mode(name)))
            }
            None => None,
        };

        handle_func.shell = match self.shell {
            Some(ref s) => {
                let c = s.clone();
                Some(Box::new(move |b| c.mode(name, b)))
            }
            None => None,
        };

        Box::new(handle_func)
    }

    pub fn is_file(&self, name: &'static str) -> Box<HandleFunc> {
        let mut handle_func = HandleFunc {
            inline: None,
            shell: None,
        };

        handle_func.inline = match self.inline {
            Some(ref i) => {
                let c = i.clone();
                Some(Box::new(move || c.is_file(name)))
            }
            None => None,
        };

        // handle_func.shell = match self.shell {
        // Some(ref s) => {
        // let c = s.clone();
        // Some(Box::new(move |b| c.is_file(name, b)))
        // }
        // None => None,
        // };
        //

        Box::new(handle_func)
    }
}

pub mod inline;
pub mod shell;
