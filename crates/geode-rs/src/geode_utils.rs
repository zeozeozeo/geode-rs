// TODO: write better bindings for this

#![allow(unsafe_op_in_unsafe_fn)]

use std::sync::atomic::AtomicUsize;

use crate::base::{SymbolScope, resolve_symbol};

#[cfg(target_os = "windows")]
pub fn geode_display_factor() -> f32 {
    static SLOT: AtomicUsize = AtomicUsize::new(0);
    let addr = resolve_symbol(
        SymbolScope::Geode,
        b"?getDisplayFactor@utils@geode@@YAMXZ\0",
        &SLOT,
    );
    if addr == 0 {
        return 1.0;
    }

    unsafe {
        let func: unsafe extern "C" fn() -> f32 = std::mem::transmute(addr);
        func()
    }
}

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "android"))]
pub fn geode_display_factor() -> f32 {
    static SLOT: AtomicUsize = AtomicUsize::new(0);
    let addr = resolve_symbol(
        SymbolScope::Geode,
        b"_ZN5geode5utils16getDisplayFactorEv\0",
        &SLOT,
    );
    if addr == 0 {
        return 1.0;
    }

    unsafe {
        let func: unsafe extern "C" fn() -> f32 = std::mem::transmute(addr);
        func()
    }
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
)))]
pub fn geode_display_factor() -> f32 {
    1.0
}

#[cfg(target_os = "windows")]
pub fn geode_mouse_position() -> crate::cocos::CCPoint {
    static SLOT: AtomicUsize = AtomicUsize::new(0);
    let addr = resolve_symbol(
        SymbolScope::Geode,
        b"?getMousePos@cocos@geode@@YA?AVCCPoint@cocos2d@@XZ\0",
        &SLOT,
    );
    if addr == 0 {
        return crate::cocos::CCPoint { x: 0.0, y: 0.0 };
    }

    unsafe {
        let mut out = std::mem::MaybeUninit::<crate::cocos::CCPoint>::uninit();
        let func: unsafe extern "system" fn(*mut crate::cocos::CCPoint) = std::mem::transmute(addr);
        func(out.as_mut_ptr());
        out.assume_init()
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn geode_mouse_position() -> crate::cocos::CCPoint {
    static SLOT: AtomicUsize = AtomicUsize::new(0);
    let addr = resolve_symbol(
        SymbolScope::Geode,
        b"_ZN5geode5cocos11getMousePosEv\0",
        &SLOT,
    );
    if addr == 0 {
        return crate::cocos::CCPoint { x: 0.0, y: 0.0 };
    }

    unsafe {
        let func: unsafe extern "C" fn() -> crate::cocos::CCPoint = std::mem::transmute(addr);
        func()
    }
}

#[cfg(target_os = "android")]
pub fn geode_mouse_position() -> crate::cocos::CCPoint {
    crate::cocos::CCPoint { x: 0.0, y: 0.0 }
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
)))]
pub fn geode_mouse_position() -> crate::cocos::CCPoint {
    crate::cocos::CCPoint { x: 0.0, y: 0.0 }
}
