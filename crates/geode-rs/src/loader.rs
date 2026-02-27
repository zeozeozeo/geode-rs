#![allow(unsafe_op_in_unsafe_fn, clippy::missing_safety_doc)]

use std::ffi::c_void;
use std::sync::OnceLock;

use crate::CallingConvention;
use crate::stl::{StlSharedPtr, StlString};
use crate::tulip::{HandlerMetadata, HookMetadata, TulipConvention};
use geode_macros::geode_bind;

geode_bind! {
    pub unsafe fn loader_get() -> *mut c_void {
        win:       "?get@Loader@geode@@SAPEAV12@XZ",
        mac_intel: "_ZN5geode6Loader3getEv",
        mac_arm:   "_ZN5geode6Loader3getEv",
        ios:       "_ZN5geode6Loader3getEv",
        android32: "_ZN5geode6Loader3getEv",
        android64: "_ZN5geode6Loader3getEv",
    }

    pub unsafe fn loader_take_next_mod(loader: *mut c_void) -> *mut c_void {
        win:       "?takeNextMod@Loader@geode@@IEAAPEAVMod@2@XZ",
        mac_intel: "_ZN5geode6Loader11takeNextModEv",
        mac_arm:   "_ZN5geode6Loader11takeNextModEv",
        ios:       "_ZN5geode6Loader11takeNextModEv",
        android32: "_ZN5geode6Loader11takeNextModEv",
        android64: "_ZN5geode6Loader11takeNextModEv",
    }

    pub unsafe fn mod_get_id(mod_ptr: *mut c_void) -> *const i8 {
        win:       "?getID@Mod@geode@@QEBA?AV?$BasicZStringView@D@2@XZ",
        mac_intel: "_ZNK5geode3Mod5getIDEv",
        mac_arm:   "_ZNK5geode3Mod5getIDEv",
        ios:       "_ZNK5geode3Mod5getIDEv",
        android32: "_ZNK5geode3Mod5getIDEv",
        android64: "_ZNK5geode3Mod5getIDEv",
    }

    pub unsafe fn mod_claim_hook(
        mod_ptr:   *mut c_void,
        hook_sptr: *const StlSharedPtr<c_void>
    ) -> method_sret [u8; 128] {
        win:       "?claimHook@Mod@geode@@QEAA?AV?$Result@PEAVHook@2@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@V?$shared_ptr@VHook@geode@@@5@@Z",
        mac_intel: "_ZN5geode3Mod9claimHookENSt3__112basic_stringIcNS1_11char_traitsIcEENS1_9allocatorIcEEEE",
        mac_arm:   "_ZN5geode3Mod9claimHookENSt3__110shared_ptrINS_4HookEEE",
        ios:       "_ZN5geode3Mod9claimHookENSt3__110shared_ptrINS_4HookEEE",
        android32: "_ZN5geode3Mod9claimHookENSt6__ndk110shared_ptrINS_4HookEEE",
        android64: "_ZN5geode3Mod9claimHookENSt6__ndk110shared_ptrINS_4HookEEE",
    }

    pub unsafe fn create_convention(conv: i32) -> sret StlSharedPtr<c_void> {
        win:       "?createConvention@hook@geode@@YA?AV?$shared_ptr@VCallingConvention@hook@tulip@@@std@@W4TulipConvention@1tulip@@@Z",
        mac_intel: "_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE",
        mac_arm:   "_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE",
        ios:       "_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE",
        android32: "_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE",
        android64: "_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE",
    }

    pub unsafe fn hook_create(
        address:      *mut c_void,
        detour:       *mut c_void,
        name:         *const StlString,
        handler_meta: *const HandlerMetadata,
        hook_meta:    HookMetadata
    ) -> sret StlSharedPtr<c_void> {
        win:       "?create@Hook@geode@@SA?AV?$shared_ptr@VHook@geode@@@std@@PEAX0V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@4@VHandlerMetadata@hook@tulip@@VHookMetadata@78@@Z",
        mac_intel: "_ZN5geode4Hook6createEPvS1_NSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE",
        mac_arm:   "_ZN5geode4Hook6createEPvS1_NSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE",
        ios:       "_ZN5geode4Hook6createEPvS1_NSt3__112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE",
        android32: "_ZN5geode4Hook6createEPvS1_NSt6__ndk112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE",
        android64: "_ZN5geode4Hook6createEPvS1_NSt6__ndk112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE",
    }

    pub unsafe fn hook_enable(hook_ptr: *mut c_void) -> method_sret [u8; 64] {
        win:       "?enable@Hook@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ",
        mac_intel: "_ZN5geode4Hook6enableEv",
        mac_arm:   "_ZN5geode4Hook6enableEv",
        ios:       "_ZN5geode4Hook6enableEv",
        android32: "_ZN5geode4Hook6enableEv",
        android64: "_ZN5geode4Hook6enableEv",
    }
}

#[cfg(target_os = "android")]
pub fn android_log(msg: &[u8]) {
    unsafe extern "C" {
        fn __android_log_print(prio: i32, tag: *const u8, fmt: *const u8, ...) -> i32;
    }
    unsafe { __android_log_print(3, b"geode-rs\0".as_ptr(), msg.as_ptr()) };
}

#[cfg(not(target_os = "android"))]
#[allow(dead_code)]
pub fn android_log(_msg: &[u8]) {}

pub struct Hook {
    ptr: *mut c_void,
}

unsafe impl Send for Hook {}
unsafe impl Sync for Hook {}

impl Hook {
    pub unsafe fn create(
        address: *mut c_void,
        detour: *mut c_void,
        name: &str,
        convention: CallingConvention,
        priority: i32,
    ) -> Option<Self> {
        let tulip_conv = TulipConvention::from(convention);

        let conv_ptr = match create_convention(tulip_conv as i32) {
            Some(p) => p,
            None => {
                #[cfg(not(target_os = "android"))]
                eprintln!("[geode-rs] Hook::create: create_convention symbol not found");
                android_log(b"Hook::create: create_convention symbol not found\0");
                return None;
            }
        };
        if conv_ptr.is_null() {
            #[cfg(not(target_os = "android"))]
            eprintln!("[geode-rs] Hook::create: createConvention returned null ptr");
            android_log(b"Hook::create: createConvention returned null ptr\0");
            return None;
        }

        let display_name = StlString::from(name);
        let handler_meta = HandlerMetadata::with_convention(conv_ptr);
        let hook_meta = HookMetadata::new(priority);

        let hook_sptr = match hook_create(address, detour, &display_name, &handler_meta, hook_meta)
        {
            Some(p) => p,
            None => {
                #[cfg(not(target_os = "android"))]
                eprintln!("[geode-rs] Hook::create: hook_create symbol not found");
                android_log(b"Hook::create: hook_create symbol not found\0");
                return None;
            }
        };

        if hook_sptr.is_null() {
            #[cfg(not(target_os = "android"))]
            eprintln!("[geode-rs] Hook::create: Hook::create returned null shared_ptr");
            android_log(b"Hook::create: hook_create returned null shared_ptr\0");
            return None;
        }

        let raw = hook_sptr.as_ptr();

        #[cfg(target_os = "android")]
        {
            let mod_ptr = Mod::get().map(|m| m.ptr()).unwrap_or(std::ptr::null_mut());
            if mod_ptr.is_null() {
                android_log(b"Hook::create: Mod ptr null, cannot claimHook\0");
                return None;
            }
            mod_claim_hook(mod_ptr, &hook_sptr)?;
        }

        std::mem::forget(hook_sptr);
        Some(Self { ptr: raw })
    }

    pub unsafe fn enable(&self) -> bool {
        hook_enable(self.ptr).is_some()
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.ptr
    }
}

pub struct Mod {
    ptr: *mut c_void,
}

unsafe impl Send for Mod {}
unsafe impl Sync for Mod {}

static SHARED_MOD: OnceLock<Mod> = OnceLock::new();

impl Mod {
    pub fn set_shared(ptr: *mut c_void) {
        let _ = SHARED_MOD.set(Mod { ptr });
    }

    pub fn get() -> Option<&'static Mod> {
        SHARED_MOD.get()
    }

    pub fn ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn get_id(&self) -> &'static str {
        static MOD_ID: OnceLock<String> = OnceLock::new();
        MOD_ID
            .get_or_init(|| unsafe {
                mod_get_id(self.ptr)
                    .and_then(|ptr| {
                        if ptr.is_null() {
                            None
                        } else {
                            Some(
                                std::ffi::CStr::from_ptr(ptr as *const std::ffi::c_char)
                                    .to_string_lossy()
                                    .into_owned(),
                            )
                        }
                    })
                    .unwrap_or_default()
            })
            .as_str()
    }

    pub unsafe fn take_next_mod() -> Option<*mut c_void> {
        let loader_ptr = loader_get()?;
        if loader_ptr.is_null() {
            return None;
        }
        let ptr = loader_take_next_mod(loader_ptr)?;
        if ptr.is_null() { None } else { Some(ptr) }
    }
}

pub mod internal {
    use super::*;

    pub fn init_mod() {
        unsafe {
            if let Some(ptr) = Mod::take_next_mod() {
                Mod::set_shared(ptr);
            }
        }
    }
}
