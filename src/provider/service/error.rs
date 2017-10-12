use std::error;
use std::fmt;

use provider;
use provider::service::inline::systemd;

#[derive(Debug)]
pub enum Error {
    DBus(systemd::dbus::Error),
    DBusArgTypeMismatch(systemd::dbus::arg::TypeMismatchError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DBus(ref err) => err.description(),
            Error::DBusArgTypeMismatch(ref err) => err.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DBus(ref err) => err.fmt(f),
            Error::DBusArgTypeMismatch(ref err) => err.fmt(f),
        }
    }
}

impl From<systemd::dbus::Error> for Error {
    fn from(err: systemd::dbus::Error) -> Error {
        Error::DBus(err)
    }
}

impl From<systemd::dbus::arg::TypeMismatchError> for Error {
    fn from(err: systemd::dbus::arg::TypeMismatchError) -> Error {
        Error::DBusArgTypeMismatch(err)
    }
}

impl From<systemd::dbus::Error> for provider::error::Error {
    fn from(err: systemd::dbus::Error) -> provider::error::Error {
        From::from(Error::DBus(err))
    }
}

impl From<systemd::dbus::arg::TypeMismatchError> for provider::error::Error {
    fn from(err: systemd::dbus::arg::TypeMismatchError) -> provider::error::Error {
        From::from(Error::DBusArgTypeMismatch(err))
    }
}
