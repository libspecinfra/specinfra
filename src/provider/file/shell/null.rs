use provider::file::shell::ShellProvider;

#[derive(Clone)]
pub struct Null;

impl ShellProvider for Null {
    fn box_clone(&self) -> Box<ShellProvider> {
        Box::new((*self).clone())
    }
}
