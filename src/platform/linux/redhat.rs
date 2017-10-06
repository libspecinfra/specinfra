use std::fs::File;
use std::io::prelude::*;

use backend::Backend;
use platform::platform::Platform;
use provider::Providers;
use provider::file::FileProvider;
use provider::file::inline::posix::Posix;
use provider::file::shell::linux::Linux;

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
        let contents = match b.run_command("cat /etc/redhat-release") {
            Err(_) => return None,
            Ok(f) => f,
        };

        self.detect_by_redhat_release(&contents.stdout)
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

impl RedHat {
    fn detect_by_redhat_release(&self, contents: &str) -> Option<Box<Platform>> {
        let mut line = contents.split(" ");
        let r = RedHat {
            name: line.nth(0).unwrap().trim().to_string(),
            release: line.nth(2).unwrap().trim().to_string(),
        };

        Some(Box::new(r))
    }
}
