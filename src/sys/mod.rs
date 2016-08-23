
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod linux_macos;

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub use self::linux_macos::*;

#[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
mod bsd;

#[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
pub use self::bsd::*;
