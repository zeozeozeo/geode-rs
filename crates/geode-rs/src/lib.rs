pub mod base;
pub mod convention;
pub mod loader;
pub mod modify;
pub mod stl;
pub mod tulip;

#[cfg(target_os = "android")]
#[link(name = "log")] // __android_log_print
unsafe extern "C" {}
#[cfg(target_os = "android")]
#[link(name = "dl")] // dlopen/dlsym
unsafe extern "C" {}

pub use base::*;
pub use convention::*;
pub use loader::*;
pub use modify::*;
pub use tulip::*;

pub use geode_macros::{geode_main, modify};

pub use ctor;

include!(concat!(env!("OUT_DIR"), "/geode_generated/mod.rs"));
