
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod linux_macos;

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub use self::linux_macos::*;

#[cfg(target_os = "freebsd")]
mod freebsd;

#[cfg(target_os = "freebsd")]
pub use self::freebsd::*;
