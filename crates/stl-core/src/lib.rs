#[allow(non_camel_case_types)]
pub mod types {
    pub type char = i8;
    pub type size_t = usize;
    pub type ptrdiff_t = isize;
}

pub mod containers;

pub mod gnustl;
pub mod libcxx;
pub mod msvc;

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "android"))]
pub use libcxx::*;
#[cfg(windows)]
pub use msvc::*;

pub use containers::{map, set, unordered_map, unordered_set};
pub use libc;
