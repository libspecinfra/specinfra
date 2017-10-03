use std::fs::File;
use std::io::prelude::*;

use backend::Backend;
use platform::platform::Platform;
use provider::Provider;
use provider::file::FileProvider;
use provider::file::inline::posix::Posix;
use provider::file::shell::linux::Linux;

#[derive(Clone)]
pub struct Ubuntu {
    name: String,
    release: String,
}

fn detect_by_lsb_releae(content: &str) -> Option<Box<Platform>> {
    let mut lines = content.lines();
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

impl Platform for Ubuntu {
    fn new() -> Ubuntu {
        Ubuntu {
            name: "".to_string(),
            release: "".to_string(),
        }
    }

    fn inline_detector(&self) -> Option<Box<Platform>> {
        let mut file = File::open("/etc/lsb-release").unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        detect_by_lsb_releae(&content)
    }

    fn shell_detector(&self, b: &Backend) -> Option<Box<Platform>> {
        let content = b.run_command("cat /etc/lsb-release").unwrap();
        detect_by_lsb_releae(&content)
    }

    fn get_provider(&self) -> Box<Provider> {
        let fp = FileProvider {
            inline: Box::new(Posix),
            shell: Box::new(Linux),
        };

        let p = Provider { file: Box::new(fp) };

        Box::new(p)
    }
}
