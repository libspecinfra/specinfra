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
        let res = b.run_command("uname -sr").unwrap();
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
        let fp = FileProvider {
            inline: Box::new(file::inline::posix::Posix),
            shell: Box::new(file::shell::bsd::Bsd),
        };

        let sp = ServiceProvider {
            inline: Box::new(service::inline::null::Null),
            shell: Box::new(service::shell::null::Null),
        };

        let p = Providers {
            file: Box::new(fp),
            service: Box::new(sp),
        };

        Ok(Box::new(p))
    }
}
