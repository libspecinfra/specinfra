// This is dummy module for not using feature systemd

pub mod dbus {
    use std::error;
    use std::fmt;

    #[derive(Debug)]
    pub struct Error;

    impl error::Error for Error {
        fn description(&self) -> &str {
            "DBus error"
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "DBus error")
        }
    }
}
