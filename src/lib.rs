extern crate uname;
extern crate libc;

use std::ffi::CStr;
use std::error::Error;
use std::fmt;

use libc::c_char;

use backend::Backend;
use platform::platform::Platform;
use resource::file::File;
use provider::Provider;

pub mod backend;
pub mod platform;
pub mod resource;
pub mod provider;

#[derive(Debug)]
pub struct Specinfra<'a> {
    pub backend: &'a Backend,
    pub platform: Box<Platform>,
    pub provider: Box<Provider>,
}

#[derive(Debug)]
pub struct SpecinfraError;

impl Error for SpecinfraError {
    fn description(&self) -> &str {
        "error"
    }
}

impl fmt::Display for SpecinfraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error")
    }
}

pub fn new(b: &Backend) -> Result<Specinfra, Box<Error>> {
    let p = try!(b.detect_platform().ok_or(SpecinfraError));
    let provider = p.get_provider();
    Ok(Specinfra {
        backend: b,
        platform: p,
        provider: provider,
    })
}

impl<'a> Specinfra<'a> {
    pub fn file(&self, name: &'static str) -> File {
        File::new(name, self.backend, &self.provider)
    }
}

// Wrapper functions for FFI

use backend::BackendWrapper;

#[no_mangle]
pub extern "C" fn specinfra_new<'a>(ptr: *const BackendWrapper) -> *const Specinfra<'a> {
    let b = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    Box::into_raw(Box::new(self::new(&*b.backend).unwrap()))
}

#[no_mangle]
pub extern "C" fn specinfra_free(ptr: *mut Specinfra) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn specinfra_file(ptr: *const Specinfra, name: *const c_char) -> *const File {
    let s = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let name = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };

    Box::into_raw(Box::new(s.file(name.to_str().unwrap())))
}
