use std::result::Result;
use std::error::Error;

use backend::Backend;
use provider;
use provider::Output;
use platform::platform::Platform;
use platform::platforms::Platforms;

#[derive(Debug)]
pub struct Direct;

impl Backend for Direct {
    fn new() -> Direct {
        Direct
    }

    fn detect_platform(&self) -> Option<Box<Platform>> {
        let mut platforms = Platforms::new();
        while let Some(p) = platforms.next() {
            match p.detect_inline() {
                Some(m) => return Some(m),
                None => (),
            };
        }
        None
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
