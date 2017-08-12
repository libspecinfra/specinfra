use std::result::Result;
use std::error::Error;
use std::process::Command;
use std::str;

use SpecinfraError;
use backend::Backend;
use provider;
use provider::Output;
use platform::platform::Platform;
use platform::platforms::Platforms;

#[derive(Debug)]
pub struct Direct;

impl Direct {
    fn new() -> Direct {
        Direct
    }
}

impl Backend for Direct {
    fn detect_platform(&self) -> Option<Box<Platform>> {
        let mut platforms = Platforms::new();
        while let Some(p) = platforms.next() {
            match p.inline_detector() {
                Some(m) => return Some(m),
                None => (),
            };

            match p.shell_detector(self) {
                Some(m) => return Some(m),
                None => (),
            }
        }
        None
    }

    fn handle(&self, handle_func: Box<provider::HandleFunc>) -> Result<Output, Box<Error>> {
        match handle_func.inline {
            Some(f) => return f(),
            None => {}
        };

        match handle_func.shell {
            Some(f) => return f(self),
            None => {}
        };

        Err(Box::new(SpecinfraError))
    }

    fn run_command(&self, c: &str) -> Result<String, Box<Error>> {
        let out = try!(Command::new("sh").args(&["-c", c]).output());
        let res = try!(String::from_utf8(out.stdout));
        Ok(res.trim().to_string())
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
