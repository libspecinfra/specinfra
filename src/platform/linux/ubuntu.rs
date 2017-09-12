use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use backend::Backend;
use platform::platform::Platform;
use provider::Provider;
use provider::file::FileProvider;
use provider::file::inline::posix::Posix;

#[derive(Clone)]
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
        let file = File::open("/etc/lsb-release").unwrap();
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        if line.starts_with("DISTRIB_ID") {
            let id = line.split("=").nth(1).unwrap().trim();
            if id == "Ubuntu" {
                let mut line = String::new();
                let _ = reader.read_line(&mut line);
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

    fn shell_detector(&self, _: &Backend) -> Option<Box<Platform>> {
        None
    }

    fn get_provider(&self) -> Box<Provider> {
        let fp = FileProvider {
            inline: Some(Box::new(Posix)),
            shell: None,
        };

        let p = Provider { file: Box::new(fp) };

        Box::new(p)
    }
}
