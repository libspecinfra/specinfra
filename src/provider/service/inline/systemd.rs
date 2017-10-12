pub extern crate dbus;

use std::result::Result;

use self::dbus::{Connection, BusType, Message, Path};
use self::dbus::arg::Variant;

use provider::service::inline::InlineProvider;
use provider::Output;
use provider::error::Error;

#[derive(Clone, Debug)]
pub struct Systemd;

impl InlineProvider for Systemd {
    fn is_running(&self, name: &str) -> Result<Output, Error> {
        let state = try!(self.get_active_state(name));
        Ok(Output::Bool(state == "active"))
    }

    fn box_clone(&self) -> Box<InlineProvider> {
        Box::new((*self).clone())
    }
}

impl Systemd {
    fn get_active_state(&self, name: &str) -> Result<String, Error> {
        let c = try!(Connection::get_private(BusType::System));

        let service: String;
        if name.ends_with(".service") {
            service = name.to_string()
        } else {
            service = name.to_string() + ".service"
        }

        let m = try!(Message::new_method_call("org.freedesktop.systemd1",
                                              "/org/freedesktop/systemd1",
                                              "org.freedesktop.systemd1.Manager",
                                              "GetUnit"))
            .append1(service);

        let r = try!(c.send_with_reply_and_block(m, 2000));
        let object_path: Path = try!(r.read1());

        let m = try!(Message::new_method_call("org.freedesktop.systemd1",
                                              object_path,
                                              "org.freedesktop.DBus.Properties",
                                              "Get"))
            .append2("org.freedesktop.systemd1.Unit", "ActiveState");

        let r = try!(c.send_with_reply_and_block(m, 2000));
        let s: Variant<&str> = try!(r.read1());
        let active_state = s.0;

        Ok(active_state.to_string())
    }
}
