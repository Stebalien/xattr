extern crate xattr;
extern crate tempfile;

use std::collections::BTreeSet;
use std::ffi::OsStr;
use xattr::FileExt;

use tempfile::{tempfile_in, NamedTempFile};

#[test]
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn test_fd() {
    use std::os::unix::ffi::OsStrExt;
    // Only works on "real" filesystems.
    let tmp = tempfile_in("/var/tmp").unwrap();
    assert!(tmp.get_xattr("user.test").is_err());
    assert_eq!(tmp.list_xattr()
               .unwrap()
               .filter(|x| x.as_bytes().starts_with(b"user."))
               .count(), 0);

    tmp.set_xattr("user.test", b"my test").unwrap();
    assert_eq!(tmp.get_xattr("user.test").unwrap(), b"my test");
    assert_eq!(tmp.list_xattr()
               .unwrap()
               .filter(|x| x.as_bytes().starts_with(b"user."))
               .collect::<Vec<_>>(), vec![OsStr::new("user.test")]);

    tmp.remove_xattr("user.test").unwrap();
    assert!(tmp.get_xattr("user.test").is_err());
    assert_eq!(tmp.list_xattr()
               .unwrap()
               .filter(|x| x.as_bytes().starts_with(b"user."))
               .count(), 0);
}

#[test]
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn test_path() {
    use std::os::unix::ffi::OsStrExt;
    // Only works on "real" filesystems.
    let tmp = NamedTempFile::new_in("/var/tmp").unwrap();
    assert!(xattr::get(tmp.path(), "user.test").is_err());
    assert_eq!(xattr::list(tmp.path())
               .unwrap()
               .filter(|x| x.as_bytes().starts_with(b"user."))
               .count(), 0);

    xattr::set(tmp.path(), "user.test", b"my test").unwrap();
    assert_eq!(xattr::get(tmp.path(), "user.test").unwrap(), b"my test");
    assert_eq!(xattr::list(tmp.path())
               .unwrap()
               .filter(|x| x.as_bytes().starts_with(b"user."))
               .collect::<Vec<_>>(), vec![OsStr::new("user.test")]);

    xattr::remove(tmp.path(), "user.test").unwrap();
    assert!(xattr::get(tmp.path(), "user.test").is_err());
    assert_eq!(xattr::list(tmp.path())
               .unwrap()
               .filter(|x| x.as_bytes().starts_with(b"user."))
               .count(), 0);
}

#[test]
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn test_multi() {
    use std::os::unix::ffi::OsStrExt;
    // Only works on "real" filesystems.
    let tmp = tempfile_in("/var/tmp").unwrap();
    let mut items: BTreeSet<_> = [
        OsStr::new("user.test1"),
        OsStr::new("user.test2"),
        OsStr::new("user.test3")
    ].iter().cloned().collect();

    for it in &items {
        tmp.set_xattr(it, b"value").unwrap();
    }
    for it in tmp.list_xattr().unwrap()
        .filter(|x| x.as_bytes().starts_with(&*b"user.")) {
        assert!(items.remove(&*it));
    }
    assert!(items.is_empty());
}
