use super::InlineProvider;

#[derive(Debug, Clone)]
pub struct Null;

impl InlineProvider for Null {
    fn box_clone(&self) -> Box<InlineProvider> {
        Box::new((*self).clone())
    }
}
