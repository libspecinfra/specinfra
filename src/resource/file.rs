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
