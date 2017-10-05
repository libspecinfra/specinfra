use provider::Output;
use provider::error::Error;
use provider::error::HandleFuncNotDefined;
use provider::file::Whom;

// See https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6

pub trait InlineProvider {
    fn mode(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn size(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn is_file(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn is_directory(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn is_block_device(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn is_character_device(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn is_pipe(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn is_socket(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn is_symlink(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn exist(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn contents(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn owner(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn group(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn linked_to(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn is_readable(&self, &str, Option<&Whom>) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn is_writable(&self, &str, Option<&Whom>) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn md5sum(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn sha256sum(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn selinux_label(&self, &str) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn box_clone(&self) -> Box<InlineProvider>;
}

impl Clone for Box<InlineProvider> {
    fn clone(&self) -> Box<InlineProvider> {
        self.box_clone()
    }
}

pub mod posix;
pub mod null;
