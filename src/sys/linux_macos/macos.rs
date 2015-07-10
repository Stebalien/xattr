use ::libc::{c_char, ssize_t, size_t, c_int, c_void, uint32_t};

#[inline(always)]
pub unsafe fn fremovexattr(fd: c_int, name: *const c_char) -> c_int {
    extern "C" {
        fn fremovexattr(fd: c_int, name: *const c_char, options: c_int) -> c_int;
    }
    fremovexattr(fd, name, options)
}

#[inline(always)]
pub unsafe fn fsetxattr(fd: c_int, name: *const c_char, value: *const c_void, size: size_t) -> ssize_t {
    extern "C" {
        fn fsetxattr(fd: c_int, name: *const c_char, value: *const c_void, size: size_t, position: uint32_t, options: c_int) -> ssize_t;
    }
    fsetxattr(fd, name, value, size, 0, 0)
}

#[inline(always)]
pub unsafe fn fgetxattr(fd: c_int, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t {
    extern "C" {
        fn fgetxattr(fd: c_int, name: *const c_char, value: *mut c_void, position: uint32_t, size: size_t, options: c_int) -> ssize_t;
    }
    fgetxattr(fd, name, value, size, 0, 0)
}

#[inline(always)]
pub unsafe fn flistxattr(fd: c_int, buf: *mut c_char, size: size_t) -> ssize_t {
    extern "C" {
        fn flistxattr(fd: c_int, buf: *mut c_char, size: size_t, options: c_int) -> ssize_t;
    }
    flistxattr(fd, buf, size, 0)
}

#[inline(always)]
pub unsafe fn lremovexattr(fd: c_int, name: *const c_char) -> c_int {
    extern "C" {
        fn lremovexattr(fd: c_int, name: *const c_char, options: c_int) -> c_int;
    }
    lremovexattr(fd, name, options)
}

#[inline(always)]
pub unsafe fn lsetxattr(path: *const c_char, name: *const c_char, value: *const c_void, size: size_t) -> ssize_t {
    extern "C" {
        fn lsetxattr(path: *const c_char, name: *const c_char, value: *const c_void, size: size_t, position: uint32_t, options: c_int) -> ssize_t;
    }
    lsetxattr(fd, name, value, size, 0, 0)
}

#[inline(always)]
pub unsafe fn lgetxattr(path: *const c_char, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t {
    extern "C" {
        fn lgetxattr(path: *const c_char, name: *const c_char, value: *mut c_void, position: uint32_t, size: size_t, options: c_int) -> ssize_t;
    }
    lgetxattr(fd, name, value, size, 0, 0)
}

#[inline(always)]
pub unsafe fn llistxattr(path: *const c_char, buf: *mut c_char, size: size_t) -> ssize_t {
    extern "C" {
        fn llistxattr(path: *const c_char, buf: *mut c_char, size: size_t, options: c_int) -> ssize_t;
    }
    llistxattr(fd, buf, size, 0)
}
