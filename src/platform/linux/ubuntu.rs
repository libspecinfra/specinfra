use std::fs::File;
use std::io::prelude::*;

use backend::Backend;
use platform::platform::Platform;
use provider::Providers;
use provider::file::FileProvider;
use provider::file::inline::posix::Posix;
use provider::file::shell::linux::Linux;

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
        let contents = match b.run_command("cat /etc/lsb-release") {
            Err(_) => return None,
            Ok(f) => f,
        };

        self.detect_by_lsb_release(&contents.stdout)
    }

    fn get_providers(&self) -> Box<Providers> {
        let fp = FileProvider {
            inline: Box::new(Posix),
            shell: Box::new(Linux),
        };

        let p = Providers { file: Box::new(fp) };

        Box::new(p)
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
