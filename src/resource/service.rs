use libc::{c_char, int32_t};
use std;
use std::ffi::CString;
use std::error::Error;

use backend::Backend;
use provider::error;
use provider::Output;
use provider::service::ServiceProvider;

pub struct Service<'a> {
    name: &'static str,
    backend: &'a Backend,
    provider: &'a ServiceProvider,
    error: Option<error::Error>,
}

impl<'a> Service<'a> {
    pub fn new(n: &'static str, b: &'a Backend, p: &'a ServiceProvider) -> Service<'a> {
        Service {
            name: n,
            backend: b,
            provider: p,
            error: None,
        }
    }

    pub fn is_running(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_running(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_enabled(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_enabled(self.name))
            .and_then(Output::to_bool)
    }

    pub fn enable(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.enable(self.name))
            .and_then(Output::to_bool)
    }

    pub fn disable(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.disable(self.name))
            .and_then(Output::to_bool)
    }

    pub fn start(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.start(self.name))
            .and_then(Output::to_bool)
    }

    pub fn stop(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.stop(self.name))
            .and_then(Output::to_bool)
    }

    pub fn reload(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.reload(self.name))
            .and_then(Output::to_bool)
    }

    pub fn restart(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.restart(self.name))
            .and_then(Output::to_bool)
    }
}

// Wrapper functions for FFI

#[no_mangle]
pub extern "C" fn resource_service_free(ptr: *mut Service) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn resource_service_error_description(ptr: *const Service) -> *const c_char {
    let s = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    match s.error {
        Some(ref e) => CString::new(e.description()).unwrap().into_raw(),
        None => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn resource_service_is_running(ptr: *mut Service) -> int32_t {
    let s = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match s.is_running() {
        Ok(f) => if f { 1 } else { 0 },
        Err(e) => {
            s.error = Some(e);
            return -1;
        }
    }
}
