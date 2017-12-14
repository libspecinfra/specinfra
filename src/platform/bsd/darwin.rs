use std::result::Result;

use uname;

use backend::Backend;
use platform::platform::Platform;
use platform::error::Error;
use provider::Providers;
use provider::file;
use provider::file::FileProvider;
use provider::service;
use provider::service::ServiceProvider;
use provider::package;
use provider::package::PackageProvider;
use provider::port;
use provider::port::PortProvider;

#[derive(Clone, Debug)]
pub struct Darwin {
    name: String,
    release: String,
}

impl Platform for Darwin {
    fn new() -> Darwin {
        Darwin {
            name: "".to_string(),
            release: "".to_string(),
        }
    }

    fn inline_detector(&self) -> Option<Box<Platform>> {
        let u = match uname::uname() {
            Ok(u) => u,
            Err(_) => return None,
        };

        if u.sysname == "Darwin" {
            let d = Darwin {
                name: u.sysname,
                release: u.release,
            };
            Some(Box::new(d))
        } else {
            None
        }
    }

    fn shell_detector(&self, b: &Backend) -> Option<Box<Platform>> {
        let res = b.run_command("uname -sr".into()).unwrap();
        let mut iter = res.stdout.split_whitespace();
        let sysname = iter.next().unwrap();
        if sysname == "Darwin" {
            let release = iter.next().unwrap();
            let d = Darwin {
                name: sysname.to_string(),
                release: release.to_string(),
            };
            Some(Box::new(d))
        } else {
            None
        }
    }

    fn get_providers(&self) -> Result<Box<Providers>, Error> {
        let file_provider = FileProvider {
            inline: Box::new(file::inline::posix::Posix),
            shell: Box::new(file::shell::bsd::Bsd),
        };

        let service_provider = ServiceProvider {
            inline: Box::new(service::inline::null::Null),
            shell: Box::new(service::shell::null::Null),
        };

        let package_provider = PackageProvider {
            inline: Box::new(package::inline::null::Null),
            shell: Box::new(package::shell::null::Null),
        };

        let port_provider = PortProvider {
            inline: Box::new(port::inline::null::Null),
            shell: Box::new(port::shell::null::Null),
        };

        let p = Providers {
            file: Box::new(file_provider),
            service: Box::new(service_provider),
            package: Box::new(package_provider),
            port: Box::new(port_provider),
        };

        Ok(Box::new(p))
    }
}
