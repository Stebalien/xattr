#[cfg(any(target_os = "macos", target_os = "linux"))]
mod linux_macos;

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub use self::linux_macos::*;

#[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
mod bsd;

#[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
pub use self::bsd::*;

#[cfg(all(feature = "unsupported",
          not(any(target_os = "freebsd", target_os = "netbsd", target_os = "macos",
                  target_os = "linux"))))]
mod unsupported;

#[cfg(all(feature = "unsupported",
          not(any(target_os = "freebsd", target_os = "netbsd", target_os = "macos",
                  target_os = "linux"))))]
pub use self::unsupported::*;

/// A constant indicating whether or not the target platform is supported.
///
/// To make programmer's lives easier, this library builds on all platforms.
/// However, all function calls on unsupported platforms will return
/// `io::Error`s.
///
/// Note: If you would like compilation to simply fail on unsupported platforms,
/// turn of the `unsupported` feature.
pub const SUPPORTED_PLATFORM: bool = cfg!(any(
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "macos",
    target_os = "linux"
));
