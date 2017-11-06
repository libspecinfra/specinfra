extern crate uname;
extern crate libc;
extern crate users;
extern crate md5;
extern crate sha2;
extern crate nix;
extern crate regex;

use std::ffi::CStr;

use libc::c_char;

use backend::Backend;
use platform::platform::Platform;
use platform::error::Error;
use platform::error::DetectError;
use resource::file::File;
use resource::service::Service;
use provider::Providers;

pub struct Specinfra<'a> {
    pub backend: &'a Backend,
    pub platform: Box<Platform>,
    pub providers: Box<Providers>,
}

pub fn new(b: &Backend) -> Result<Specinfra, Error> {
    let p = try!(b.detect_platform()
        .ok_or(DetectError { message: "Failed to detect platform".to_string() }));
    let providers = try!(p.get_providers());
    Ok(Specinfra {
        backend: b,
        platform: p,
        providers: providers,
    })
}

impl<'a> Specinfra<'a> {
    pub fn file(&self, name: &'static str) -> File {
        File::new(name, self.backend, &self.providers.file)
    }

    pub fn service(&self, name: &'static str) -> Service {
        Service::new(name, self.backend, &self.providers.service)
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

#[no_mangle]
pub extern "C" fn specinfra_service(ptr: *const Specinfra, name: *const c_char) -> *const Service {
    let s = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let name = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };

    Box::into_raw(Box::new(s.service(name.to_str().unwrap())))
}

pub mod backend;
pub mod platform;
pub mod resource;
pub mod provider;
pub mod error;
