extern crate xattr;
extern crate tempfile;

use xattr::FileExt;

use tempfile::NamedTempFile;

#[test]
#[cfg(target_os = "linux")]
fn test() {
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
