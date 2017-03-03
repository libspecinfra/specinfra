pub mod inline;

use provider::file::File;
use provider::HandleFunc;

#[derive(Clone, Debug)]
pub struct Posix;

impl File for Posix {
    fn mode(&self, name: &'static str) -> Box<HandleFunc> {
        Box::new(HandleFunc { inline: Box::new(move || inline::mode(name)) })
    }
}
