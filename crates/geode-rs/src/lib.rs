pub mod base;
pub mod convention;
pub mod fmod;
pub mod geode_utils;
pub mod inherit;
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
pub use geode_utils::*;
pub use inherit::*;
pub use loader::*;
pub use modify::*;
pub use tulip::*;

pub use geode_macros::{geode_bind, geode_main, inherit, modify};

pub use ctor;

#[macro_export]
macro_rules! spr {
    ($name:expr) => {
        $crate::loader::expand_sprite_name($name)
    };
}

include!(concat!(env!("OUT_DIR"), "/geode_generated/mod.rs"));
