#![allow(unsafe_op_in_unsafe_fn, clippy::missing_safety_doc)]

use std::collections::HashMap;
use std::ffi::c_void;
use std::panic::AssertUnwindSafe;
use std::sync::{Mutex, OnceLock};

use crate::CallingConvention;
use crate::loader::Hook;

pub struct ModifyStorage<T> {
    data: OnceLock<Mutex<HashMap<usize, T>>>,
}

impl<T> Default for ModifyStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ModifyStorage<T> {
    pub const fn new() -> Self {
        Self {
            data: OnceLock::new(),
        }
    }

    fn get_data(&self) -> &Mutex<HashMap<usize, T>> {
        self.data.get_or_init(|| Mutex::new(HashMap::new()))
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get(&self, key: usize) -> Option<&mut T> {
        let mut map = self.get_data().lock().unwrap();
        map.get_mut(&key).map(|v| unsafe { &mut *(v as *mut T) })
    }

    pub fn get_or_default<F: FnOnce() -> T>(&self, key: usize, f: F) -> &'static mut T {
        let mut map = self.get_data().lock().unwrap();
        let entry = map.entry(key).or_insert_with(f);
        unsafe { &mut *(entry as *mut T) }
    }

    pub fn remove(&self, key: usize) {
        let mut map = self.get_data().lock().unwrap();
        map.remove(&key);
    }
}

struct PendingHook {
    address: *mut c_void,
    detour: *mut c_void,
    name: String,
    convention: CallingConvention,
}

unsafe impl Send for PendingHook {}

static PENDING_HOOKS: OnceLock<Mutex<Vec<PendingHook>>> = OnceLock::new();

pub unsafe fn register_hook(
    address: usize,
    detour: *mut c_void,
    name: &str,
    convention: CallingConvention,
) {
    if address == 0 {
        #[cfg(not(target_os = "android"))]
        eprintln!("[geode-rs] refusing to register hook {name} at address 0");
        #[cfg(target_os = "android")]
        crate::loader::android_log(b"register_hook: refused address 0\0");
        return;
    }

    let hooks = PENDING_HOOKS.get_or_init(|| Mutex::new(Vec::new()));
    hooks.lock().unwrap().push(PendingHook {
        address: address as *mut c_void,
        detour,
        name: name.to_string(),
        convention,
    });
}

pub fn flush_pending_hooks() {
    let hooks = PENDING_HOOKS.get_or_init(|| Mutex::new(Vec::new()));
    let pending: Vec<PendingHook> = hooks.lock().unwrap().drain(..).collect();

    for hook in pending {
        if hook.address.is_null() {
            #[cfg(not(target_os = "android"))]
            eprintln!(
                "[geode-rs] refusing to create hook {} at address 0",
                hook.name
            );
            #[cfg(target_os = "android")]
            crate::loader::android_log(b"flush_pending_hooks: skipped address 0\0");
            continue;
        }

        match Hook::create(hook.address, hook.detour, &hook.name, hook.convention, 0) {
            Ok(h) => {
                if let Err(err) = h.enable() {
                    #[cfg(not(target_os = "android"))]
                    eprintln!("[geode-rs] failed to enable hook {}: {err}", hook.name);
                    #[cfg(target_os = "android")]
                    crate::loader::android_log_string(&format!(
                        "flush_pending_hooks: enable FAILED for {} at {:p}: {err}",
                        hook.name, hook.address
                    ));
                }
            }
            Err(err) => {
                #[cfg(not(target_os = "android"))]
                eprintln!(
                    "[geode-rs] failed to create hook {} at {:p}: {err}",
                    hook.name, hook.address
                );
                #[cfg(target_os = "android")]
                crate::loader::android_log_string(&format!(
                    "flush_pending_hooks: create FAILED for {} at {:p}: {err}",
                    hook.name, hook.address
                ));
            }
        }
    }
}

pub fn run_hook<R: Default>(name: &str, f: impl FnOnce() -> R) -> R {
    match std::panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(value) => value,
        Err(_) => {
            #[cfg(not(target_os = "android"))]
            eprintln!("[geode-rs] panic in hook {name}; returning default value");
            #[cfg(target_os = "android")]
            crate::loader::android_log_string(&format!(
                "panic in hook {name}; returning default value"
            ));
            R::default()
        }
    }
}
