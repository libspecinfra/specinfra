use backend::Backend;
use provider::error;
use provider::Provider;
use provider::Output;
use libc::{c_char, uint32_t, int32_t, int64_t};
use std::ffi::CString;
use std::ffi::CStr;
use std;
use std::error::Error;

pub struct File<'a> {
    name: &'static str,
    backend: &'a Backend,
    provider: &'a Provider,
    error: Option<error::Error>,
}

impl<'a> File<'a> {
    pub fn new(n: &'static str, b: &'a Backend, p: &'a Provider) -> File<'a> {
        File {
            name: n,
            backend: b,
            provider: p,
            error: None,
        }
    }

    pub fn mode(&self) -> Result<i32, error::Error> {
        self.backend
            .handle(self.provider.file.mode(self.name))
            .and_then(Output::to_i32)
    }

    pub fn size(&self) -> Result<i64, error::Error> {
        self.backend
            .handle(self.provider.file.size(self.name))
            .and_then(Output::to_i64)
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

    pub fn md5sum(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.file.md5sum(self.name))
            .and_then(Output::to_string)
    }

    pub fn sha256sum(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.file.sha256sum(self.name))
            .and_then(Output::to_string)
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
pub extern "C" fn resource_file_error_description(ptr: *const File) -> *const c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    match f.error {
        Some(ref e) => CString::new(e.description()).unwrap().into_raw(),
        None => std::ptr::null(),
    }
}


#[no_mangle]
pub extern "C" fn resource_file_mode(ptr: *mut File) -> int32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    f.mode().unwrap_or_else(|e| {
        f.error = Some(e);
        return -1;
    })

}

#[no_mangle]
pub extern "C" fn resource_file_is_file(ptr: *mut File) -> int32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_file() {
        Ok(f) => if f { 1 } else { 0 },
        Err(e) => {
            f.error = Some(e);
            return -1;
        }
    }

}

#[no_mangle]
pub extern "C" fn resource_file_exist(ptr: *mut File) -> int32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.exist() {
        Ok(f) => if f { 1 } else { 0 },
        Err(e) => {
            f.error = Some(e);
            return -1;
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_directory(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_directory().unwrap() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn resource_file_is_block_device(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_block_device().unwrap() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn resource_file_is_character_device(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_character_device().unwrap() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_pipe(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_pipe().unwrap() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn resource_file_is_socket(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_socket().unwrap() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn resource_file_is_symlink(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_symlink().unwrap() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn resource_file_contents(ptr: *const File) -> *mut c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let c = f.contents().unwrap();
    CString::new(c).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn resource_file_owner(ptr: *const File) -> *mut c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let c = f.owner().unwrap();
    CString::new(c).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn resource_file_group(ptr: *const File) -> *mut c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let c = f.group().unwrap();
    CString::new(c).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn resource_file_is_readable(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_readable().unwrap() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn resource_file_is_readable_by_owner(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_readable_by_owner().unwrap() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_readable_by_group(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_readable_by_group().unwrap() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_readable_by_others(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_readable_by_others().unwrap() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_readable_by_user(ptr: *const File,
                                                    u: *const c_char)
                                                    -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let c_str = unsafe {
        assert!(!u.is_null());
        CStr::from_ptr(u)
    };

    let user = c_str.to_str().unwrap();
    if f.is_readable_by_user(user).unwrap() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_writable(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_writable().unwrap() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn resource_file_is_writable_by_owner(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_writable_by_owner().unwrap() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_writable_by_group(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_writable_by_group().unwrap() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_writable_by_others(ptr: *const File) -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    if f.is_writable_by_others().unwrap() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_writable_by_user(ptr: *const File,
                                                    u: *const c_char)
                                                    -> uint32_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let c_str = unsafe {
        assert!(!u.is_null());
        CStr::from_ptr(u)
    };

    let user = c_str.to_str().unwrap();
    if f.is_writable_by_user(user).unwrap() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn resource_file_md5sum(ptr: *const File) -> *mut c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let c = f.md5sum().unwrap();
    CString::new(c).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn resource_file_sha256sum(ptr: *const File) -> *mut c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let c = f.sha256sum().unwrap();
    CString::new(c).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn resource_file_size(ptr: *const File) -> int64_t {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    f.size().unwrap()
}

#[no_mangle]
pub extern "C" fn resource_file_linked_to(ptr: *const File) -> *mut c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let c = f.linked_to().unwrap();
    CString::new(c).unwrap().into_raw()
}
