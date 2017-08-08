use std::result::Result;
use std::error::Error;

use backend::Backend;
use provider;
use provider::Output;
use platform::platform::Platform;

#[derive(Debug)]
pub struct Direct;

impl Backend for Direct {
    fn new() -> Direct {
        Direct
    }

    fn detect(&self, p: Box<Platform>) -> Option<Box<Platform>> {
        p.detect_inline()
    }

    fn handle(&self, handle_func: Box<provider::HandleFunc>) -> Result<Output, Box<Error>> {
        (handle_func.inline)()
    }
}

// Wrapper functions for FFI

use backend::BackendWrapper;

#[no_mangle]
pub extern "C" fn backend_direct_new() -> *mut BackendWrapper {
    let b = BackendWrapper { backend: Box::new(Direct::new()) };
    Box::into_raw(Box::new(b))
}

#[no_mangle]
pub extern "C" fn backend_direct_free(ptr: *mut BackendWrapper) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
