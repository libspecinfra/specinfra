use std::result::Result;
use std::process::Command;
use std::str;

use backend;
use backend::Backend;
use backend::CommandResult;
use provider::error::Error;
use provider::HandleFunc;
use provider::Output;
use platform::platform::Platform;
use platform::platforms::Platforms;

pub struct Direct;

impl Direct {
    pub fn new() -> Direct {
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

    fn handle(&self, handle_func: Box<HandleFunc>) -> Result<Output, Error> {
        match (handle_func.inline)() {
            Ok(r) => return Ok(r),
            Err(Error::HandleFuncNotDefined(_)) => (),
            Err(e) => return Err(e),
        };

        (handle_func.shell)(self)
    }

    fn run_command(&self, c: &str) -> Result<CommandResult, backend::error::Error> {
        let out = try!(Command::new("sh").args(&["-c", c]).output());

        if !out.status.success() {
            let e = backend::error::CommandError {
                code: out.status.code().unwrap(),
                message: try!(String::from_utf8(out.stderr)),
            };
            return Err(From::from(e));
        }

        let stdout = try!(String::from_utf8(out.stdout));
        let stderr = try!(String::from_utf8(out.stderr));
        let res = CommandResult {
            stdout: stdout.trim().to_string(),
            stderr: stderr.trim().to_string(),
            code: out.status.code().unwrap(),
            success: out.status.success(),
        };

        Ok(res)
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
