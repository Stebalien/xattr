//! Dummy FFI definitions needed for testing on non-Linux, non-OSX platforms
use libc::{c_char, c_int, c_void, size_t, ssize_t};

extern {
    pub fn flistxattr(fd: c_int, buf: *mut c_char, size: size_t) -> ssize_t;
    pub fn fgetxattr(fd: c_int,
                     name: *const c_char,
                     value: *mut c_void,
                     size: size_t) -> ssize_t;
    pub fn fremovexattr(fd: c_int, name: *const c_char) -> c_int;

    pub fn llistxattr(path: *const c_char,
                      buf: *mut c_char,
                      size: size_t) -> ssize_t;
    pub fn lgetxattr(
        path: *const c_char,
        name: *const c_char,
        value: *mut c_void,
        size: size_t,
    ) -> ssize_t;
    pub fn lremovexattr(path: *const c_char, name: *const c_char) -> c_int;
}

pub unsafe fn fsetxattr(
    _fd: c_int,
    _name: *const c_char,
    _value: *const c_void,
    _size: size_t,
) -> ssize_t {
    unimplemented!()
}

pub unsafe fn lsetxattr(
    _path: *const c_char,
    _name: *const c_char,
    _value: *const c_void,
    _size: size_t,
) -> ssize_t {
    unimplemented!()
}
