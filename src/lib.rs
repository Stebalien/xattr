extern crate libc;

mod sys;
mod util;

use std::os::unix::io::AsRawFd;
use std::ffi::OsStr;
use std::io;
use std::fs::File;
use std::path::Path;

pub use sys::XAttrs;

pub fn get<N, P>(path: P, name: N) -> io::Result<Vec<u8>> 
    where P: AsRef<Path>,
          N: AsRef<OsStr>
{
    sys::get_path(path.as_ref(), name.as_ref())
}

pub fn set<N, P>(path: P, name: N, value: &[u8]) -> io::Result<()> 
    where P: AsRef<Path>,
          N: AsRef<OsStr>
{
    sys::set_path(path.as_ref(), name.as_ref(), value)
}

pub fn remove<N, P>(path: P, name: N) -> io::Result<()> 
    where P: AsRef<Path>,
          N: AsRef<OsStr>
{
    sys::remove_path(path.as_ref(), name.as_ref())
}

pub fn list<P>(path: P) -> io::Result<XAttrs>
    where P: AsRef<Path>
{
    sys::list_path(path.as_ref())
}

pub unsafe trait FileExt: AsRawFd {
    fn get_xattr<N>(&self, name: N) -> io::Result<Vec<u8>> 
        where N: AsRef<OsStr>
    {
        sys::get_fd(self.as_raw_fd(), name.as_ref())
    }

    fn set_xattr<N>(&self, name: N, value: &[u8]) -> io::Result<()> 
        where N: AsRef<OsStr>
    {
        sys::set_fd(self.as_raw_fd(), name.as_ref(), value)
    }

    fn remove_xattr<N>(&self, name: N) -> io::Result<()> 
        where N: AsRef<OsStr>
    {
        sys::remove_fd(self.as_raw_fd(), name.as_ref())
    }

    fn list_xattr(&self) -> io::Result<XAttrs> {
        sys::list_fd(self.as_raw_fd())
    }
}

unsafe impl FileExt for File { }
