extern crate xattr;
extern crate tempfile;

use xattr::FileExt;

use tempfile::NamedTempFile;

#[test]
#[cfg(target_os = "linux")]
fn test_fd() {
    // Only works on "real" filesystems.
    let tmp = NamedTempFile::new_in("/var/tmp").unwrap();
    assert!(tmp.get_xattr("user.test").is_err());
    assert_eq!(tmp.list_xattr().unwrap().next(), None);

    tmp.set_xattr("user.test", b"my test").unwrap();
    assert_eq!(tmp.get_xattr("user.test").unwrap(), b"my test");
    assert_eq!(&tmp.list_xattr().unwrap().next().unwrap(), "user.test");

    tmp.remove_xattr("user.test").unwrap();
    assert!(tmp.get_xattr("user.test").is_err());
    assert_eq!(tmp.list_xattr().unwrap().next(), None);
}

#[test]
#[cfg(target_os = "linux")]
fn test_path() {
    // Only works on "real" filesystems.
    let tmp = NamedTempFile::new_in("/var/tmp").unwrap();
    assert!(xattr::get(tmp.path(), "user.test").is_err());
    assert_eq!(xattr::list(tmp.path(), ).unwrap().next(), None);

    xattr::set(tmp.path(), "user.test", b"my test").unwrap();
    assert_eq!(xattr::get(tmp.path(), "user.test").unwrap(), b"my test");
    assert_eq!(&xattr::list(tmp.path(), ).unwrap().next().unwrap(), "user.test");

    xattr::remove(tmp.path(), "user.test").unwrap();
    assert!(xattr::get(tmp.path(), "user.test").is_err());
    assert_eq!(xattr::list(tmp.path(), ).unwrap().next(), None);
}
