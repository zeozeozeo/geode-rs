#![allow(unsafe_op_in_unsafe_fn, clippy::missing_safety_doc)]

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Mutex, OnceLock};

use crate::CallingConvention;
use crate::base;
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
    let base = base::get();
    let actual_address = base + address;

    let hooks = PENDING_HOOKS.get_or_init(|| Mutex::new(Vec::new()));
    hooks.lock().unwrap().push(PendingHook {
        address: actual_address as *mut c_void,
        detour,
        name: name.to_string(),
        convention,
    });
}

pub fn flush_pending_hooks() {
    let hooks = PENDING_HOOKS.get_or_init(|| Mutex::new(Vec::new()));
    let pending: Vec<PendingHook> = hooks.lock().unwrap().drain(..).collect();

    for hook in pending {
        unsafe {
            if let Some(h) = Hook::create(hook.address, hook.detour, &hook.name, hook.convention, 0)
            {
                h.enable();
            } else {
                #[cfg(not(target_os = "android"))]
                eprintln!(
                    "[geode-rs] failed to create hook for address: {:p}",
                    hook.address
                );
                #[cfg(target_os = "android")]
                crate::loader::android_log(b"flush_pending_hooks: hook FAILED\0");
            }
        }
    }
}
