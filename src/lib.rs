//! Manage extended attributes.
//!
//! Note: This library *does not* follow symlinks.
extern crate libc;

mod sys;
mod util;
mod error;

use std::os::unix::io::AsRawFd;
use std::ffi::OsStr;
use std::io;
use std::fs::File;
use std::path::Path;

pub use sys::{XAttrs, SUPPORTED_PLATFORM};
pub use error::UnsupportedPlatformError;

/// Get an extended attribute for the specified file.
pub fn get<N, P>(path: P, name: N) -> io::Result<Option<Vec<u8>>>
    where P: AsRef<Path>,
          N: AsRef<OsStr>
{
    util::extract_noattr(sys::get_path(path.as_ref(), name.as_ref()))
}

/// Set an extended attribute on the specified file.
pub fn set<N, P>(path: P, name: N, value: &[u8]) -> io::Result<()>
    where P: AsRef<Path>,
          N: AsRef<OsStr>
{
    sys::set_path(path.as_ref(), name.as_ref(), value)
}

/// Remove an extended attribute from the specified file.
pub fn remove<N, P>(path: P, name: N) -> io::Result<()>
    where P: AsRef<Path>,
          N: AsRef<OsStr>
{
    sys::remove_path(path.as_ref(), name.as_ref())
}

/// List extended attributes attached to the specified file.
///
/// Note: this may not list *all* attributes. Speficially, it definitely won't list any trusted
/// attributes unless you are root and it may not list system attributes.
pub fn list<P>(path: P) -> io::Result<XAttrs>
    where P: AsRef<Path>
{
    sys::list_path(path.as_ref())
}

pub trait FileExt: AsRawFd {
    /// Get an extended attribute for the specified file.
    fn get_xattr<N>(&self, name: N) -> io::Result<Option<Vec<u8>>>
        where N: AsRef<OsStr>
    {
        util::extract_noattr(sys::get_fd(self.as_raw_fd(), name.as_ref()))
    }

    /// Set an extended attribute on the specified file.
    fn set_xattr<N>(&self, name: N, value: &[u8]) -> io::Result<()>
        where N: AsRef<OsStr>
    {
        sys::set_fd(self.as_raw_fd(), name.as_ref(), value)
    }

    /// Remove an extended attribute from the specified file.
    fn remove_xattr<N>(&self, name: N) -> io::Result<()>
        where N: AsRef<OsStr>
    {
        sys::remove_fd(self.as_raw_fd(), name.as_ref())
    }

    /// List extended attributes attached to the specified file.
    ///
    /// Note: this may not list *all* attributes. Speficially, it definitely won't list any trusted
    /// attributes unless you are root and it may not list system attributes.
    fn list_xattr(&self) -> io::Result<XAttrs> {
        sys::list_fd(self.as_raw_fd())
    }
}

impl FileExt for File {}
