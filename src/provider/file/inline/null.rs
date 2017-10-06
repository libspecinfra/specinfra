use provider::file::inline::InlineProvider;

#[derive(Clone, Debug)]
pub struct Null;

impl InlineProvider for Null {
    fn box_clone(&self) -> Box<InlineProvider> {
        Box::new((*self).clone())
    }
}
