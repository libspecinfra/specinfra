pub extern crate dbus;

use std::result::Result;

use self::dbus::{Connection, BusType, Message, Path};

use provider::service::inline::InlineProvider;
use provider::Output;
use provider::error::Error;

#[derive(Clone, Debug)]
pub struct Systemd;

impl InlineProvider for Systemd {
    fn is_running(&self, name: &str) -> Result<Output, Error> {

        Ok(Output::Bool(true))
    }

    fn box_clone(&self) -> Box<InlineProvider> {
        Box::new((*self).clone())
    }
}

impl Systemd {
    fn get_active_state(&self, name: &str) -> Result<&str, Error> {
        let c = try!(Connection::get_private(BusType::System));

        let m = try!(Message::new_method_call("org.freedesktop.systemd1",
                                              "/org/freedesktop/systemd1",
                                              "org.freedesktop.systemd1.Manager",
                                              "GetUnit"));

        Ok("active")
    }
}
