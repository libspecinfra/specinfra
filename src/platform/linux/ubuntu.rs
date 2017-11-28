use std::fs::File;
use std::io::prelude::*;
use std::result::Result;

use backend::Backend;
use platform::platform::Platform;
use platform::error::Error;
use provider::Providers;
use provider::file::FileProvider;
use provider::service::ServiceProvider;
use provider::file;
use provider::service;
use provider::package;
use provider::package::PackageProvider;

#[derive(Clone, Debug)]
pub struct Ubuntu {
    name: String,
    release: String,
}

impl Platform for Ubuntu {
    fn new() -> Ubuntu {
        Ubuntu {
            name: "".to_string(),
            release: "".to_string(),
        }
    }

    fn inline_detector(&self) -> Option<Box<Platform>> {
        let mut file = match File::open("/etc/lsb-release") {
            Err(_) => return None,
            Ok(f) => f,
        };


        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        self.detect_by_lsb_release(&contents)
    }

    fn shell_detector(&self, b: &Backend) -> Option<Box<Platform>> {
        let contents = match b.run_command("cat /etc/lsb-release".into()) {
            Err(_) => return None,
            Ok(f) => f,
        };

        self.detect_by_lsb_release(&contents.stdout)
    }

    fn get_providers(&self) -> Result<Box<Providers>, Error> {
        let fp = FileProvider {
            inline: Box::new(file::inline::posix::Posix),
            shell: Box::new(file::shell::linux::Linux),
        };

        let r = try!(self.release.parse::<f32>());

        let sp = match r {
            n if n >= 16.0 => {
                ServiceProvider {
                    inline: Box::new(service::inline::systemd::Systemd),
                    shell: Box::new(service::shell::systemd::Systemd),
                }
            }
            _ => {
                ServiceProvider {
                    inline: Box::new(service::inline::null::Null),
                    shell: Box::new(service::shell::ubuntu_init::UbuntuInit),
                }
            }
        };

        let pp = PackageProvider {
            inline: Box::new(package::inline::null::Null),
            shell: Box::new(package::shell::apt::Apt),
        };

        let p = Providers {
            file: Box::new(fp),
            service: Box::new(sp),
            package: Box::new(pp),
        };

        Ok(Box::new(p))
    }
}

impl Ubuntu {
    fn detect_by_lsb_release(&self, contents: &str) -> Option<Box<Platform>> {
        let mut lines = contents.lines();
        let line = lines.next().unwrap();
        if line.starts_with("DISTRIB_ID") {
            let id = line.split("=").nth(1).unwrap().trim();
            if id == "Ubuntu" {
                let line = lines.next().unwrap();
                let release = line.split("=").nth(1).unwrap();
                let u = Ubuntu {
                    name: id.to_string(),
                    release: release.to_string(),
                };

                return Some(Box::new(u));
            }
        }
        None
    }
}
