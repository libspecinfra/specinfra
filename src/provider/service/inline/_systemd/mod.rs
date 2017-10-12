use provider::service::inline::InlineProvider;

#[derive(Clone, Debug)]
pub struct Systemd;

impl InlineProvider for Systemd {
    fn box_clone(&self) -> Box<InlineProvider> {
        Box::new((*self).clone())
    }
}

pub mod dbus;
