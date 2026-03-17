#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc,
    clippy::not_unsafe_ptr_arg_deref
)]

mod raw;

use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::sync::OnceLock;

use crate::CallingConvention;
use crate::stl::{StlOptional, StlSharedPtr, StlSpan, StlString, StlVector};
use crate::tulip::{HandlerMetadata, HookMetadata, TulipConvention};

pub type LoaderResult<T> = Result<T, String>;
pub type ByteSpan = StlSpan<u8>;

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

#[repr(C)]
union ResultPayload<T> {
    ok: ManuallyDrop<T>,
    err: ManuallyDrop<StlString>,
}

#[repr(C)]
struct GeodeResult<T> {
    payload: ResultPayload<T>,
    tag: usize,
}

impl<T> GeodeResult<T> {
    fn is_ok(&self) -> bool {
        (self.tag & 0xff) == 0
    }

    unsafe fn into_rust(self) -> LoaderResult<T> {
        let this = ManuallyDrop::new(self);
        if this.is_ok() {
            Ok(ManuallyDrop::into_inner(std::ptr::read(&this.payload.ok)))
        } else {
            Err(stl_string_to_string(&ManuallyDrop::into_inner(
                std::ptr::read(&this.payload.err),
            )))
        }
    }
}

#[derive(Clone)]
pub struct Hook {
    ptr: *mut c_void,
    _owned: Option<StlSharedPtr<c_void>>,
}

unsafe impl Send for Hook {}
unsafe impl Sync for Hook {}

impl Hook {
    pub fn create(
        address: *mut c_void,
        detour: *mut c_void,
        name: &str,
        convention: CallingConvention,
        priority: i32,
    ) -> LoaderResult<Self> {
        let tulip_conv = TulipConvention::from(convention);
        let conv_ptr = unsafe { raw::create_convention(tulip_conv as i32) }
            .ok_or_else(|| "missing geode::hook::createConvention".to_owned())?;
        if conv_ptr.is_null() {
            return Err("geode::hook::createConvention returned null".to_owned());
        }

        let display_name = StlString::from(name);
        let handler_meta = HandlerMetadata::with_convention(conv_ptr);
        let hook_meta = HookMetadata::new(priority);
        let hook =
            unsafe { raw::hook_create(address, detour, &display_name, &handler_meta, hook_meta) }
                .ok_or_else(|| "missing geode::Hook::create".to_owned())?;

        if hook.is_null() {
            return Err("geode::Hook::create returned null".to_owned());
        }

        let current_mod =
            Mod::get().ok_or_else(|| "current Geode mod is unavailable".to_owned())?;
        let claimed = unsafe { raw::mod_claim_hook(current_mod.ptr, &hook) }
            .ok_or_else(|| "missing geode::Mod::claimHook".to_owned())?;
        unsafe { claimed.into_rust() }?;

        Ok(Self {
            ptr: hook.as_ptr(),
            _owned: Some(hook),
        })
    }

    pub fn owner(&self) -> Option<Mod> {
        let ptr = unsafe { raw::hook_get_owner(self.ptr) }?;
        Some(Mod { ptr })
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { raw::hook_is_enabled(self.ptr) }.unwrap_or(false)
    }

    pub fn enable(&self) -> LoaderResult<()> {
        let result = unsafe { raw::hook_enable(self.ptr) }
            .ok_or_else(|| "missing geode::Hook::enable".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn disable(&self) -> LoaderResult<()> {
        let result = unsafe { raw::hook_disable(self.ptr) }
            .ok_or_else(|| "missing geode::Hook::disable".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn toggle(&self) -> LoaderResult<()> {
        let result = unsafe { raw::hook_toggle(self.ptr) }
            .ok_or_else(|| "missing geode::Hook::toggle".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn toggle_to(&self, enabled: bool) -> LoaderResult<()> {
        let result = unsafe { raw::hook_toggle_to(self.ptr, enabled) }
            .ok_or_else(|| "missing geode::Hook::toggle(bool)".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn auto_enable(&self) -> bool {
        unsafe { raw::hook_get_auto_enable(self.ptr) }.unwrap_or(false)
    }

    pub fn set_auto_enable(&self, enabled: bool) {
        let _ = unsafe { raw::hook_set_auto_enable(self.ptr, enabled) };
    }

    pub fn address(&self) -> usize {
        unsafe { raw::hook_get_address(self.ptr) }.unwrap_or_default()
    }

    pub fn display_name(&self) -> String {
        unsafe { raw::hook_get_display_name(self.ptr) }
            .map(|view| view.as_str().to_owned())
            .unwrap_or_default()
    }

    pub fn metadata(&self) -> Option<HookMetadata> {
        unsafe { raw::hook_get_hook_metadata(self.ptr) }
    }

    pub fn set_metadata(&self, metadata: HookMetadata) {
        let _ = unsafe { raw::hook_set_hook_metadata(self.ptr, &metadata) };
    }

    pub fn priority(&self) -> i32 {
        unsafe { raw::hook_get_priority(self.ptr) }.unwrap_or_default()
    }

    pub fn set_priority(&self, priority: i32) {
        let _ = unsafe { raw::hook_set_priority(self.ptr, priority) };
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

#[derive(Clone)]
pub struct Patch {
    ptr: *mut c_void,
    _owned: Option<StlSharedPtr<c_void>>,
}

unsafe impl Send for Patch {}
unsafe impl Sync for Patch {}

impl Patch {
    pub fn create(address: *mut c_void, bytes: &[u8]) -> LoaderResult<Self> {
        let patch = unsafe { raw::patch_create(address, &ByteSpan::from(bytes)) }
            .ok_or_else(|| "missing geode::Patch::create".to_owned())?;
        if patch.is_null() {
            return Err("geode::Patch::create returned null".to_owned());
        }

        let current_mod =
            Mod::get().ok_or_else(|| "current Geode mod is unavailable".to_owned())?;
        let claimed = unsafe { raw::mod_claim_patch(current_mod.ptr, &patch) }
            .ok_or_else(|| "missing geode::Mod::claimPatch".to_owned())?;
        unsafe { claimed.into_rust() }?;

        Ok(Self {
            ptr: patch.as_ptr(),
            _owned: Some(patch),
        })
    }

    pub fn owner(&self) -> Option<Mod> {
        let ptr = unsafe { raw::patch_get_owner(self.ptr) }?;
        Some(Mod { ptr })
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { raw::patch_is_enabled(self.ptr) }.unwrap_or(false)
    }

    pub fn enable(&self) -> LoaderResult<()> {
        let result = unsafe { raw::patch_enable(self.ptr) }
            .ok_or_else(|| "missing geode::Patch::enable".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn disable(&self) -> LoaderResult<()> {
        let result = unsafe { raw::patch_disable(self.ptr) }
            .ok_or_else(|| "missing geode::Patch::disable".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn toggle(&self) -> LoaderResult<()> {
        let result = unsafe { raw::patch_toggle(self.ptr) }
            .ok_or_else(|| "missing geode::Patch::toggle".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn toggle_to(&self, enabled: bool) -> LoaderResult<()> {
        let result = unsafe { raw::patch_toggle_to(self.ptr, enabled) }
            .ok_or_else(|| "missing geode::Patch::toggle(bool)".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn auto_enable(&self) -> bool {
        unsafe { raw::patch_get_auto_enable(self.ptr) }.unwrap_or(false)
    }

    pub fn set_auto_enable(&self, enabled: bool) {
        let _ = unsafe { raw::patch_set_auto_enable(self.ptr, enabled) };
    }

    pub fn bytes(&self) -> &[u8] {
        unsafe { raw::patch_get_bytes(self.ptr) }
            .and_then(|value| unsafe { value.as_ref() })
            .map(|value| &value[..])
            .unwrap_or(&[])
    }

    pub fn update_bytes(&self, bytes: &[u8]) -> LoaderResult<()> {
        let result = unsafe { raw::patch_update_bytes(self.ptr, &ByteSpan::from(bytes)) }
            .ok_or_else(|| "missing geode::Patch::updateBytes".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn address(&self) -> usize {
        unsafe { raw::patch_get_address(self.ptr) }.unwrap_or_default()
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

#[derive(Clone, Copy)]
pub struct Loader {
    ptr: *mut c_void,
}

unsafe impl Send for Loader {}
unsafe impl Sync for Loader {}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LoadingState {
    #[default]
    None = 0,
    Queue = 1,
    List = 2,
    Graph = 3,
    EarlyMods = 4,
    Mods = 5,
    Problems = 6,
    Done = 7,
}

impl Loader {
    pub fn get() -> Option<Self> {
        let ptr = unsafe { raw::loader_get() }?;
        (!ptr.is_null()).then_some(Self { ptr })
    }

    pub fn loading_state(&self) -> LoadingState {
        unsafe { raw::loader_get_loading_state(self.ptr) }
            .map(|raw| match raw {
                1 => LoadingState::Queue,
                2 => LoadingState::List,
                3 => LoadingState::Graph,
                4 => LoadingState::EarlyMods,
                5 => LoadingState::Mods,
                6 => LoadingState::Problems,
                7 => LoadingState::Done,
                _ => LoadingState::None,
            })
            .unwrap_or_default()
    }

    pub fn is_patchless(&self) -> bool {
        unsafe { raw::loader_is_patchless(self.ptr) }.unwrap_or(false)
    }

    pub fn is_mod_installed(&self, id: &str) -> bool {
        unsafe { raw::loader_is_mod_installed(self.ptr, id.into()) }.unwrap_or(false)
    }

    pub fn get_installed_mod(&self, id: &str) -> Option<Mod> {
        let ptr = unsafe { raw::loader_get_installed_mod(self.ptr, id.into()) }?;
        Some(Mod { ptr })
    }

    pub fn is_mod_loaded(&self, id: &str) -> bool {
        unsafe { raw::loader_is_mod_loaded(self.ptr, id.into()) }.unwrap_or(false)
    }

    pub fn get_loaded_mod(&self, id: &str) -> Option<Mod> {
        let ptr = unsafe { raw::loader_get_loaded_mod(self.ptr, id.into()) }?;
        Some(Mod { ptr })
    }

    pub fn all_mods(&self) -> Vec<Mod> {
        unsafe { raw::loader_get_all_mods(self.ptr) }
            .map(|mods| {
                mods.iter()
                    .copied()
                    .filter(|ptr| !ptr.is_null())
                    .map(|ptr| Mod { ptr })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn launch_argument_names(&self) -> Vec<String> {
        unsafe { raw::loader_get_launch_argument_names(self.ptr) }
            .map(string_vec_to_vec)
            .unwrap_or_default()
    }

    pub fn has_launch_argument(&self, name: &str) -> bool {
        unsafe { raw::loader_has_launch_argument(self.ptr, name.into()) }.unwrap_or(false)
    }

    pub fn get_launch_argument(&self, name: &str) -> Option<String> {
        unsafe { raw::loader_get_launch_argument(self.ptr, name.into()) }
            .and_then(optional_string_to_option)
    }

    pub fn game_version(&self) -> Option<String> {
        unsafe { raw::loader_get_game_version(self.ptr) }.map(|value| stl_string_to_string(&value))
    }
}

#[derive(Clone, Copy)]
pub struct Mod {
    ptr: *mut c_void,
}

unsafe impl Send for Mod {}
unsafe impl Sync for Mod {}

static SHARED_MOD: OnceLock<Mod> = OnceLock::new();

impl Mod {
    pub fn set_shared(ptr: *mut c_void) {
        let _ = SHARED_MOD.set(Self { ptr });
    }

    pub fn get() -> Option<&'static Self> {
        SHARED_MOD.get()
    }

    pub fn id(&self) -> String {
        unsafe { raw::mod_get_id(self.ptr) }
            .map(|value| value.to_string_lossy())
            .unwrap_or_default()
    }

    pub fn name(&self) -> String {
        unsafe { raw::mod_get_name(self.ptr) }
            .map(|value| value.to_string_lossy())
            .unwrap_or_default()
    }

    pub fn is_loaded(&self) -> bool {
        unsafe { raw::mod_is_loaded(self.ptr) }.unwrap_or(false)
    }

    pub fn is_currently_loading(&self) -> bool {
        unsafe { raw::mod_is_currently_loading(self.ptr) }.unwrap_or(false)
    }

    pub fn has_settings(&self) -> bool {
        unsafe { raw::mod_has_settings(self.ptr) }.unwrap_or(false)
    }

    pub fn setting_keys(&self) -> Vec<String> {
        unsafe { raw::mod_get_setting_keys(self.ptr) }
            .map(string_vec_to_vec)
            .unwrap_or_default()
    }

    pub fn has_setting(&self, key: &str) -> bool {
        unsafe { raw::mod_has_setting(self.ptr, key.into()) }.unwrap_or(false)
    }

    pub fn get_setting(&self, key: &str) -> Option<SettingV3> {
        let shared = unsafe { raw::mod_get_setting(self.ptr, key.into()) }?;
        (!shared.is_null()).then_some(SettingV3 {
            ptr: shared.as_ptr(),
            _shared: shared,
        })
    }

    pub fn get_hooks(&self) -> Vec<Hook> {
        unsafe { raw::mod_get_hooks(self.ptr) }
            .map(|hooks| {
                hooks
                    .iter()
                    .copied()
                    .filter(|ptr| !ptr.is_null())
                    .map(|ptr| Hook {
                        ptr: ptr.cast(),
                        _owned: None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_patches(&self) -> Vec<Patch> {
        unsafe { raw::mod_get_patches(self.ptr) }
            .map(|patches| {
                patches
                    .iter()
                    .copied()
                    .filter(|ptr| !ptr.is_null())
                    .map(|ptr| Patch {
                        ptr: ptr.cast(),
                        _owned: None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn enable(&self) -> LoaderResult<()> {
        let result = unsafe { raw::mod_enable(self.ptr) }
            .ok_or_else(|| "missing geode::Mod::enable".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn disable(&self) -> LoaderResult<()> {
        let result = unsafe { raw::mod_disable(self.ptr) }
            .ok_or_else(|| "missing geode::Mod::disable".to_owned())?;
        unsafe { result.into_rust() }
    }

    pub fn launch_argument_name(&self, name: &str) -> Option<String> {
        unsafe { raw::mod_get_launch_argument_name(self.ptr, name.into()) }
            .map(|value| stl_string_to_string(&value))
    }

    pub fn launch_argument_names(&self) -> Vec<String> {
        unsafe { raw::mod_get_launch_argument_names(self.ptr) }
            .map(string_vec_to_vec)
            .unwrap_or_default()
    }

    pub fn has_launch_argument(&self, name: &str) -> bool {
        unsafe { raw::mod_has_launch_argument(self.ptr, name.into()) }.unwrap_or(false)
    }

    pub fn get_launch_argument(&self, name: &str) -> Option<String> {
        unsafe { raw::mod_get_launch_argument(self.ptr, name.into()) }
            .and_then(optional_string_to_option)
    }

    pub fn settings_manager(&self) -> Option<ModSettingsManager> {
        let ptr = unsafe { raw::mod_settings_manager_from(self.ptr) }?;
        (!ptr.is_null()).then_some(ModSettingsManager { ptr })
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    unsafe fn take_next_mod() -> Option<*mut c_void> {
        let loader = Loader::get()?;
        let ptr = raw::loader_take_next_mod(loader.ptr)?;
        (!ptr.is_null()).then_some(ptr)
    }
}

#[derive(Clone)]
pub struct SettingV3 {
    ptr: *mut c_void,
    _shared: StlSharedPtr<c_void>,
}

unsafe impl Send for SettingV3 {}
unsafe impl Sync for SettingV3 {}

impl SettingV3 {
    pub fn key(&self) -> Option<String> {
        unsafe { raw::setting_get_key(self.ptr) }.map(|value| stl_string_to_string(&value))
    }

    pub fn mod_id(&self) -> Option<String> {
        unsafe { raw::setting_get_mod_id(self.ptr) }.map(|value| stl_string_to_string(&value))
    }

    pub fn mod_handle(&self) -> Option<Mod> {
        let ptr = unsafe { raw::setting_get_mod(self.ptr) }?;
        Some(Mod { ptr })
    }

    pub fn name(&self) -> Option<String> {
        unsafe { raw::setting_get_name(self.ptr) }.and_then(optional_string_to_option)
    }

    pub fn display_name(&self) -> Option<String> {
        unsafe { raw::setting_get_display_name(self.ptr) }.map(|value| stl_string_to_string(&value))
    }

    pub fn description(&self) -> Option<String> {
        unsafe { raw::setting_get_description(self.ptr) }.and_then(optional_string_to_option)
    }

    pub fn enable_if(&self) -> Option<String> {
        unsafe { raw::setting_get_enable_if(self.ptr) }.and_then(optional_string_to_option)
    }

    pub fn enable_if_description(&self) -> Option<String> {
        unsafe { raw::setting_get_enable_if_description(self.ptr) }
            .and_then(optional_string_to_option)
    }

    pub fn should_enable(&self) -> bool {
        unsafe { raw::setting_should_enable(self.ptr) }.unwrap_or(false)
    }

    pub fn requires_restart(&self) -> bool {
        unsafe { raw::setting_requires_restart(self.ptr) }.unwrap_or(false)
    }
}

#[derive(Clone, Copy)]
pub struct ModSettingsManager {
    ptr: *mut c_void,
}

unsafe impl Send for ModSettingsManager {}
unsafe impl Sync for ModSettingsManager {}

impl ModSettingsManager {
    pub fn from_mod(mod_handle: &Mod) -> Option<Self> {
        let ptr = unsafe { raw::mod_settings_manager_from(mod_handle.ptr) }?;
        (!ptr.is_null()).then_some(Self { ptr })
    }

    pub fn get(&self, key: &str) -> Option<SettingV3> {
        let shared = unsafe { raw::mod_settings_manager_get(self.ptr, key.into()) }?;
        (!shared.is_null()).then_some(SettingV3 {
            ptr: shared.as_ptr(),
            _shared: shared,
        })
    }

    pub fn restart_required(&self) -> bool {
        unsafe { raw::mod_settings_manager_restart_required(self.ptr) }.unwrap_or(false)
    }

    pub fn add_dependant(&self, dependant: &Mod) {
        let _ = unsafe { raw::mod_settings_manager_add_dependant(self.ptr, dependant.ptr) };
    }
}

fn stl_string_to_string(value: &StlString) -> String {
    value.to_string()
}

fn optional_string_to_option(value: StlOptional<StlString>) -> Option<String> {
    Option::<StlString>::from(value).map(|value| stl_string_to_string(&value))
}

fn string_vec_to_vec(value: StlVector<StlString>) -> Vec<String> {
    value.iter().map(stl_string_to_string).collect()
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
