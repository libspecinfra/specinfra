use std::fmt::Debug;

use provider::error::Error;
use provider::error::HandleFuncNotDefined;
use provider::file::Whom;

use provider::Output;
use backend::Backend;

// See https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6

pub trait ShellProvider: Debug {
    fn mode(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "mode".to_string(),
        };
        Err(From::from(e))
    }

    fn size(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "size".to_string(),
        };
        Err(From::from(e))
    }

    fn is_file(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_file".to_string(),
        };
        Err(From::from(e))
    }

    fn is_directory(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_directory".to_string(),
        };
        Err(From::from(e))
    }

    fn is_block_device(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_block_device".to_string(),
        };
        Err(From::from(e))
    }

    fn is_character_device(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_character_device".to_string(),
        };
        Err(From::from(e))
    }

    fn is_pipe(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_pipe".to_string(),
        };
        Err(From::from(e))
    }

    fn is_socket(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_socket".to_string(),
        };
        Err(From::from(e))
    }

    fn is_symlink(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_symlink".to_string(),
        };
        Err(From::from(e))
    }

    fn exist(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "exist".to_string(),
        };
        Err(From::from(e))
    }

    fn contents(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "contents".to_string(),
        };
        Err(From::from(e))
    }

    fn owner(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "owner".to_string(),
        };
        Err(From::from(e))
    }

    fn group(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "group".to_string(),
        };
        Err(From::from(e))
    }

    fn linked_to(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "linked_to".to_string(),
        };
        Err(From::from(e))
    }

    fn is_readable(&self, &str, Option<&Whom>, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_readable".to_string(),
        };
        Err(From::from(e))
    }

    fn is_writable(&self, &str, Option<&Whom>, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_writable".to_string(),
        };
        Err(From::from(e))
    }

    fn md5sum(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "md5sum".to_string(),
        };
        Err(From::from(e))
    }

    fn sha256sum(&self, &str, &Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "sha256sum".to_string(),
        };
        Err(From::from(e))
    }

    fn box_clone(&self) -> Box<ShellProvider>;
}

impl Clone for Box<ShellProvider> {
    fn clone(&self) -> Box<ShellProvider> {
        self.box_clone()
    }
}

pub mod bsd;
pub mod linux;
pub mod null;
