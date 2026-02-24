#![allow(unsafe_op_in_unsafe_fn, clippy::missing_safety_doc)]

use std::ffi::c_void;
use std::sync::OnceLock;

use crate::CallingConvention;
use crate::stl::{StlSharedPtr, StlString};
use crate::tulip::{HandlerMetadata, HookMetadata, TulipConvention};

#[cfg(target_os = "windows")]
mod geode_ffi {
    use super::*;
    use crate::base::get_proc_address;
    use std::ffi::CStr;

    fn geode_base() -> usize {
        crate::base::get_geode()
    }

    fn get_symbol(name: &[u8]) -> Option<usize> {
        let base = geode_base();
        unsafe { get_proc_address(base, name) }
    }

    macro_rules! define_geode_func {
        ($name:ident, $mangled:literal, $sig:ty) => {
            pub fn $name() -> Option<$sig> {
                static ADDR: OnceLock<Option<usize>> = OnceLock::new();
                let addr = ADDR.get_or_init(|| get_symbol($mangled));
                addr.map(|a| unsafe { std::mem::transmute::<usize, $sig>(a) })
            }
        };
    }

    define_geode_func!(
        loader_get,
        b"?get@Loader@geode@@SAPEAV12@XZ",
        unsafe extern "system" fn() -> *mut c_void
    );

    define_geode_func!(
        take_next_mod,
        b"?takeNextMod@Loader@geode@@IEAAPEAVMod@2@XZ",
        unsafe extern "system" fn(*mut c_void) -> *mut c_void
    );

    define_geode_func!(
        create_convention,
        b"?createConvention@hook@geode@@YA?AV?$shared_ptr@VCallingConvention@hook@tulip@@@std@@W4TulipConvention@1tulip@@@Z",
        unsafe extern "system" fn(*mut StlSharedPtr<c_void>, i32)
    );

    define_geode_func!(
        mod_get_id,
        b"?getID@Mod@geode@@QEBA?AV?$BasicZStringView@D@2@XZ",
        unsafe extern "system" fn(*mut c_void) -> *const i8
    );

    pub fn init_loader() -> Option<&'static GeodeLoader> {
        static LOADER: OnceLock<Option<GeodeLoader>> = OnceLock::new();
        LOADER
            .get_or_init(|| {
                let loader_get_fn = loader_get();
                let take_next = take_next_mod();
                let create_conv = create_convention();
                let mod_get_id_fn = mod_get_id();

                match (loader_get_fn, take_next, create_conv, mod_get_id_fn) {
                    (Some(lg), Some(tn), Some(cc), Some(mgi)) => Some(GeodeLoader {
                        loader_get: lg,
                        take_next_mod: tn,
                        create_convention: cc,
                        mod_get_id: mgi,
                    }),
                    _ => None,
                }
            })
            .as_ref()
    }

    pub struct GeodeLoader {
        pub loader_get: unsafe extern "system" fn() -> *mut c_void,
        pub take_next_mod: unsafe extern "system" fn(*mut c_void) -> *mut c_void,
        pub create_convention: unsafe extern "system" fn(*mut StlSharedPtr<c_void>, i32),
        pub mod_get_id: unsafe extern "system" fn(*mut c_void) -> *const i8,
    }

    impl GeodeLoader {
        pub unsafe fn get_mod_id(&self, mod_ptr: *mut c_void) -> Option<String> {
            let ptr = (self.mod_get_id)(mod_ptr);
            if ptr.is_null() {
                return None;
            }
            let cstr = CStr::from_ptr(ptr);
            Some(cstr.to_string_lossy().into_owned())
        }
    }

    static HOOK_CREATE_ADDR: OnceLock<Option<usize>> = OnceLock::new();
    static HOOK_ENABLE_ADDR: OnceLock<Option<usize>> = OnceLock::new();

    fn hook_create_addr() -> Option<usize> {
        *HOOK_CREATE_ADDR.get_or_init(|| {
            get_symbol(b"?create@Hook@geode@@SA?AV?$shared_ptr@VHook@geode@@@std@@PEAX0V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@4@VHandlerMetadata@hook@tulip@@VHookMetadata@78@@Z")
        })
    }

    fn hook_enable_addr() -> Option<usize> {
        *HOOK_ENABLE_ADDR.get_or_init(|| {
            get_symbol(b"?enable@Hook@geode@@QEAA?AV?$Result@XV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@XZ")
        })
    }

    type HookCreateFn = unsafe extern "system" fn(
        *mut StlSharedPtr<c_void>,
        *mut c_void,
        *mut c_void,
        *const StlString,
        *const HandlerMetadata,
        HookMetadata,
    );

    pub unsafe fn call_hook_create(
        address: *mut c_void,
        detour: *mut c_void,
        name: *const StlString,
        handler: *const HandlerMetadata,
        hook: *const HookMetadata,
    ) -> Option<StlSharedPtr<c_void>> {
        let func_addr = hook_create_addr()?;
        let func: HookCreateFn = std::mem::transmute(func_addr);

        let mut result = StlSharedPtr::empty();
        let handler_copy = std::ptr::read(handler);
        let name_copy = std::ptr::read(name);

        func(
            &mut result,
            address,
            detour,
            &name_copy,
            &handler_copy,
            *hook,
        );

        if result.is_null() { None } else { Some(result) }
    }

    type HookEnableFn = unsafe extern "system" fn(*mut c_void, *mut u64);

    pub unsafe fn call_hook_enable(hook_ptr: *mut c_void) -> bool {
        let func_addr = match hook_enable_addr() {
            Some(a) => a,
            None => return false,
        };

        let func: HookEnableFn = std::mem::transmute(func_addr);
        let mut result_buf = [0u64; 8];

        func(hook_ptr, result_buf.as_mut_ptr());

        true
    }
}

#[cfg(not(target_os = "windows"))]
mod geode_ffi {
    use super::*;

    pub fn init_loader() -> Option<&'static GeodeLoader> {
        None
    }

    pub struct GeodeLoader;

    impl GeodeLoader {
        pub unsafe fn get_mod_id(&self, _mod_ptr: *mut c_void) -> Option<String> {
            None
        }
    }

    pub unsafe fn call_hook_create(
        _address: *mut c_void,
        _detour: *mut c_void,
        _name: *const StlString,
        _handler: *const HandlerMetadata,
        _hook: *const HookMetadata,
    ) -> Option<StlSharedPtr<c_void>> {
        None
    }

    pub unsafe fn call_hook_enable(_hook_ptr: *mut c_void) -> bool {
        false
    }
}

pub use geode_ffi::*;

pub struct Hook {
    ptr: StlSharedPtr<c_void>,
}

impl Hook {
    pub unsafe fn create(
        address: *mut c_void,
        detour: *mut c_void,
        name: &str,
        convention: CallingConvention,
        priority: i32,
    ) -> Option<Self> {
        #[cfg(target_os = "windows")]
        {
            let loader = init_loader()?;

            let tulip_conv = TulipConvention::from(convention);
            let mut conv_ptr = StlSharedPtr::empty();
            (loader.create_convention)(&mut conv_ptr, tulip_conv as i32);

            if conv_ptr.is_null() {
                return None;
            }

            let display_name = StlString::new_from_str(name);
            let handler_meta = HandlerMetadata::with_convention(conv_ptr);
            let hook_meta = HookMetadata::new(priority);

            let result =
                call_hook_create(address, detour, &display_name, &handler_meta, &hook_meta)?;

            Some(Self { ptr: result })
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = (address, detour, name, convention, priority);
            None
        }
    }

    pub unsafe fn enable(&self) -> bool {
        call_hook_enable(self.ptr.as_ptr() as *mut c_void)
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.ptr.as_ptr()
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
            .get_or_init(|| {
                #[cfg(target_os = "windows")]
                {
                    unsafe {
                        init_loader()
                            .and_then(|l| l.get_mod_id(self.ptr))
                            .unwrap_or_default()
                    }
                }
                #[cfg(not(target_os = "windows"))]
                {
                    String::new()
                }
            })
            .as_str()
    }

    pub unsafe fn take_next_mod() -> Option<*mut c_void> {
        #[cfg(target_os = "windows")]
        {
            let loader = init_loader()?;
            let loader_ptr = (loader.loader_get)();
            if loader_ptr.is_null() {
                return None;
            }
            let ptr = (loader.take_next_mod)(loader_ptr);
            if ptr.is_null() { None } else { Some(ptr) }
        }
        #[cfg(not(target_os = "windows"))]
        {
            None
        }
    }
}

pub mod internal {
    use super::*;

    pub fn init_mod() {
        #[cfg(target_os = "windows")]
        unsafe {
            if let Some(ptr) = Mod::take_next_mod() {
                Mod::set_shared(ptr);
            }
        }
    }
}
