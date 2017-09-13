use backend::Backend;
use provider::error;
use provider::Provider;
use provider::Output;
use libc::uint32_t;

pub struct File<'a> {
    name: &'static str,
    backend: &'a Backend,
    provider: &'a Provider,
}

impl<'a> File<'a> {
    pub fn new(n: &'static str, b: &'a Backend, p: &'a Provider) -> File<'a> {
        File {
            name: n,
            backend: b,
            provider: p,
        }
    }

    pub fn mode(&self) -> Result<u32, error::Error> {
        self.backend
            .handle(self.provider.file.mode(self.name))
            .and_then(Output::to_u32)
    }

    pub fn is_file(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_file(self.name))
            .and_then(Output::to_bool)
    }

    pub fn exist(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.exist(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_directory(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_directory(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_block_device(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_block_device(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_character_device(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_character_device(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_pipe(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_pipe(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_socket(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_socket(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_symlink(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_symlink(self.name))
            .and_then(Output::to_bool)
    }

    pub fn contents(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.file.contents(self.name))
            .and_then(Output::to_string)
    }

    pub fn owner(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.file.owner(self.name))
            .and_then(Output::to_string)
    }

    pub fn group(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.file.group(self.name))
            .and_then(Output::to_string)
    }

    pub fn linked_to(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.file.linked_to(self.name))
            .and_then(Output::to_string)
    }

    pub fn is_readable(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_readable(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_readable_by_owner(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_readable_by_owner(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_readable_by_group(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_readable_by_group(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_readable_by_others(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_readable_by_others(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_readable_by_user(&self, user: &'static str) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_readable_by_user(self.name, user))
            .and_then(Output::to_bool)
    }

    pub fn is_writable(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_writable(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_writable_by_owner(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_writable_by_owner(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_writable_by_group(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_writable_by_group(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_writable_by_others(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_writable_by_others(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_writable_by_user(&self, user: &'static str) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.file.is_writable_by_user(self.name, user))
            .and_then(Output::to_bool)
    }
}

// Wrapper functions for FFI

#[no_mangle]
pub extern "C" fn resource_file_free(ptr: *mut File) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn resource_file_mode(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    f.mode().unwrap()
}
