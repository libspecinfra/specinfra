use platform::platform::Platform;
use uname;
use provider;


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

    fn inline_detector(&self) -> Option<(Box<Platform>)> {
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

    fn get_provider(&self) -> Box<provider::Provider> {
        Box::new(provider::Provider { file: Box::new(provider::file::posix::Posix) })
    }
}
