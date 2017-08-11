pub mod inline;
pub mod shell;

use provider::file::File;
use provider::HandleFunc;

#[derive(Clone, Debug)]
pub struct Posix;

impl File for Posix {
    fn mode(&self, name: &'static str) -> Box<HandleFunc> {
        let handle_func = HandleFunc {
            inline: Some(Box::new(move || inline::mode(name))),
            shell: Some(Box::new(move |b| shell::mode(name, b))),
        };
        Box::new(handle_func)
    }
}
