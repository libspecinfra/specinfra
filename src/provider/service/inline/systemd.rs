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

    fn is_enabled(&self, name: &str) -> Result<Output, Error> {
        let state = try!(self.get_unit_file_state(name));
        Ok(Output::Bool(state == "enabled"))
    }

    fn enable(&self, name: &str) -> Result<Output, Error> {
        let s = try!(self.enable_unit_file_state(name));
        Ok(Output::Bool(s))
    }

    fn disable(&self, name: &str) -> Result<Output, Error> {
        let s = try!(self.disable_unit_file_state(name));
        Ok(Output::Bool(s))
    }

    fn box_clone(&self) -> Box<InlineProvider> {
        Box::new((*self).clone())
    }
}

impl Systemd {
    fn enable_unit_file_state(&self, name: &str) -> Result<bool, Error> {
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
                                              "EnableUnitFiles"))
            .append3(vec![service], false, false);

        try!(c.send_with_reply_and_block(m, 2000));

        Ok(true)
    }

    fn disable_unit_file_state(&self, name: &str) -> Result<bool, Error> {
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
                                              "DisableUnitFiles"))
            .append2(vec![service], false);

        try!(c.send_with_reply_and_block(m, 2000));

        Ok(true)
    }

    fn get_active_state(&self, name: &str) -> Result<String, Error> {
        let c = try!(Connection::get_private(BusType::System));

        let object_path = try!(self.get_object_path(name));

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

    fn get_unit_file_state(&self, name: &str) -> Result<String, Error> {
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
                                              "GetUnitFileState"))
            .append1(service);

        let r = try!(c.send_with_reply_and_block(m, 2000));
        let unit_file_state: &str = try!(r.read1());
        Ok(unit_file_state.to_string())
    }

    fn get_object_path(&self, name: &str) -> Result<Path, Error> {
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
        Ok(object_path)
    }
}
