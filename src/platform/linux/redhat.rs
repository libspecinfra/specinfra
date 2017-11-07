use std::fs::File;
use std::io::prelude::*;
use std::result::Result;

use version_compare::Version;

use backend::Backend;
use platform::platform::Platform;
use platform::error::Error;
use provider::Providers;
use provider::file;
use provider::file::FileProvider;
use provider::service;
use provider::service::ServiceProvider;

#[derive(Clone, Debug)]
pub struct RedHat {
    name: String,
    release: String,
}

impl Platform for RedHat {
    fn new() -> RedHat {
        RedHat {
            name: "".to_string(),
            release: "".to_string(),
        }
    }

    fn inline_detector(&self) -> Option<Box<Platform>> {
        let mut file = match File::open("/etc/redhat-release") {
            Ok(f) => f,
            Err(_) => return None,
        };

        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        self.detect_by_redhat_release(&contents)
    }

    fn shell_detector(&self, b: &Backend) -> Option<Box<Platform>> {
        let contents = match b.run_command("cat /etc/redhat-release".into()) {
            Err(_) => return None,
            Ok(f) => f,
        };

        self.detect_by_redhat_release(&contents.stdout)
    }

    fn get_providers(&self) -> Result<Box<Providers>, Error> {
        let fp = FileProvider {
            inline: Box::new(file::inline::posix::Posix),
            shell: Box::new(file::shell::linux::Linux),
        };

        let r = Version::from(&self.release).unwrap();
        let r7 = Version::from("7").unwrap();

        let sp = match r {
            ref n if n >= &r7 => {
                ServiceProvider {
                    inline: Box::new(service::inline::systemd::Systemd),
                    shell: Box::new(service::shell::systemd::Systemd),
                }
            }
            _ => {
                ServiceProvider {
                    inline: Box::new(service::inline::null::Null),
                    shell: Box::new(service::shell::sysvinit::SysVInit),
                }
            }
        };

        let p = Providers {
            file: Box::new(fp),
            service: Box::new(sp),
        };

        Ok(Box::new(p))
    }
}

impl RedHat {
    fn detect_by_redhat_release(&self, contents: &str) -> Option<Box<Platform>> {
        let mut line = contents.split(" ");
        let name = line.nth(0).unwrap().trim().to_string();
        let mut release = line.nth(1).unwrap().trim().to_string();
        if release == "release" {
            release = line.nth(0).unwrap().trim().to_string();
        }

        let r = RedHat {
            name: name,
            release: release,
        };

        Some(Box::new(r))
    }
}
