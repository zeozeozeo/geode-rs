#![allow(unsafe_op_in_unsafe_fn, clippy::missing_safety_doc)]

use std::ffi::c_void;
use std::sync::OnceLock;

use crate::CallingConvention;
use crate::tulip::TulipConvention;

#[cfg(windows)]
pub(crate) mod geode_ffi {
    use super::*;
    use crate::base::get_proc_address;
    use crate::stl::{StlSharedPtr, StlString};
    use crate::tulip::{HandlerMetadata, HookMetadata};
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

        let mut result: StlSharedPtr<c_void> = Default::default();
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

#[cfg(target_os = "android")]
pub(crate) mod geode_ffi {
    use super::*;
    use std::sync::OnceLock;

    // geode::Loader::get()
    const SYM_LOADER_GET: &[u8] = b"_ZN5geode6Loader3getEv\0";
    // geode::Loader::takeNextMod()
    const SYM_TAKE_NEXT_MOD: &[u8] = b"_ZN5geode6Loader11takeNextModEv\0";
    // geode::Mod::claimHook(shared_ptr<Hook>) -> Result<Hook*>
    const SYM_CLAIM_HOOK: &[u8] = b"_ZN5geode3Mod9claimHookENSt6__ndk110shared_ptrINS_4HookEEE\0";
    // geode::Hook::create(void*, void*, std::basic_string<...>, HandlerMetadata, HookMetadata) -> shared_ptr<Hook>
    const SYM_HOOK_CREATE: &[u8] = b"_ZN5geode4Hook6createEPvS1_NSt6__ndk112basic_stringIcNS2_11char_traitsIcEENS2_9allocatorIcEEEEN5tulip4hook15HandlerMetadataENSA_12HookMetadataE\0";
    // geode::hook::createConvention(tulip::hook::TulipConvention) -> shared_ptr<CallingConvention>
    const SYM_CREATE_CONVENTION: &[u8] =
        b"_ZN5geode4hook16createConventionEN5tulip4hook15TulipConventionE\0";
    // geode::Mod::getID() const -> geode::BasicZStringView<char>
    const SYM_MOD_GET_ID: &[u8] = b"_ZNK5geode3Mod5getIDEv\0";

    unsafe fn get_geode_handle() -> *mut c_void {
        unsafe extern "C" {
            fn dlopen(
                filename: *const std::os::raw::c_char,
                flag: std::os::raw::c_int,
            ) -> *mut c_void;
        }
        const RTLD_LAZY: std::os::raw::c_int = 0x1;
        const RTLD_NOLOAD: std::os::raw::c_int = 0x4;

        #[cfg(target_arch = "aarch64")]
        let names: &[&[u8]] = &[b"Geode.android64.so\0", b"libGeode.so\0"];
        #[cfg(target_arch = "arm")]
        let names: &[&[u8]] = &[b"Geode.android32.so\0", b"libGeode.so\0"];
        #[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
        let names: &[&[u8]] = &[b"libGeode.so\0"];

        for name in names {
            let h = unsafe {
                dlopen(
                    name.as_ptr() as *const std::os::raw::c_char,
                    RTLD_LAZY | RTLD_NOLOAD,
                )
            };
            if !h.is_null() {
                return h;
            }
        }
        std::ptr::null_mut()
    }

    unsafe fn find_sym(name: &[u8]) -> Option<usize> {
        unsafe extern "C" {
            fn dlsym(handle: *mut c_void, symbol: *const std::os::raw::c_char) -> *mut c_void;
        }

        let handle = unsafe { get_geode_handle() };
        if !handle.is_null() {
            let sym = unsafe { dlsym(handle, name.as_ptr() as *const std::os::raw::c_char) };
            if !sym.is_null() {
                return Some(sym as usize);
            }
        }

        const RTLD_DEFAULT: *mut c_void = std::ptr::null_mut();
        let sym = unsafe { dlsym(RTLD_DEFAULT, name.as_ptr() as *const std::os::raw::c_char) };
        if sym.is_null() {
            None
        } else {
            Some(sym as usize)
        }
    }

    // geode::BasicZStringView<char>
    #[repr(C)]
    pub struct BasicZStringView {
        pub ptr: *const std::os::raw::c_char,
        pub len: usize,
    }

    pub struct GeodeLoader {
        pub loader_get: unsafe extern "C" fn() -> *mut c_void,
        pub take_next_mod: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        pub hook_create: usize,
        pub claim_hook: usize,
        pub create_convention: usize,
        pub mod_get_id: usize,
    }

    pub unsafe fn alog(msg: &[u8]) {
        unsafe extern "C" {
            fn __android_log_print(prio: i32, tag: *const u8, fmt: *const u8, ...) -> i32;
        }
        unsafe { __android_log_print(3, b"geode-rs\0".as_ptr(), msg.as_ptr()) };
    }

    pub fn init_loader() -> Option<&'static GeodeLoader> {
        static LOADER: OnceLock<Option<GeodeLoader>> = OnceLock::new();
        LOADER
            .get_or_init(|| unsafe {
                let lg = find_sym(SYM_LOADER_GET);
                let tn = find_sym(SYM_TAKE_NEXT_MOD);
                let hc = find_sym(SYM_HOOK_CREATE);
                let ch = find_sym(SYM_CLAIM_HOOK);
                let cc = find_sym(SYM_CREATE_CONVENTION);
                let mgi = find_sym(SYM_MOD_GET_ID);
                if lg.is_none() {
                    alog(b"init_loader: FAILED Loader::get\0");
                }
                if tn.is_none() {
                    alog(b"init_loader: FAILED takeNextMod\0");
                }
                if hc.is_none() {
                    alog(b"init_loader: FAILED Hook::create\0");
                }
                if ch.is_none() {
                    alog(b"init_loader: FAILED Mod::claimHook\0");
                }
                if cc.is_none() {
                    alog(b"init_loader: FAILED createConvention\0");
                }
                if mgi.is_none() {
                    alog(b"init_loader: FAILED Mod::getID\0");
                }
                let (lg, tn, hc, ch, cc, mgi) = (lg?, tn?, hc?, ch?, cc?, mgi?);
                Some(GeodeLoader {
                    loader_get: std::mem::transmute(lg),
                    take_next_mod: std::mem::transmute(tn),
                    hook_create: hc,
                    claim_hook: ch,
                    create_convention: cc,
                    mod_get_id: mgi,
                })
            })
            .as_ref()
    }

    impl GeodeLoader {
        pub unsafe fn get_mod_id(&self, mod_ptr: *mut c_void) -> Option<String> {
            type ModGetIdFn = unsafe extern "C" fn(*mut c_void) -> [usize; 2];
            let func: ModGetIdFn = std::mem::transmute(self.mod_get_id);
            let result = func(mod_ptr);
            let ptr = result[0] as *const u8;
            if ptr.is_null() {
                return None;
            }
            unsafe extern "C" {
                fn strlen(s: *const u8) -> usize;
            }
            let len = unsafe { strlen(ptr) };
            if len == 0 {
                return None;
            }
            let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
            Some(String::from_utf8_lossy(bytes).into_owned())
        }

        pub unsafe fn create_convention_fn(&self, conv: i32) -> (*mut c_void, *mut c_void) {
            #[repr(C)]
            struct SharedPtrBuf {
                ptr: usize,
                ctrl: usize,
            }
            let mut result = SharedPtrBuf { ptr: 0, ctrl: 0 };
            #[cfg(target_arch = "aarch64")]
            std::arch::asm!(
                "blr {fn_ptr}",
                fn_ptr = in(reg) self.create_convention,
                in("w0") conv,
                in("x8") &mut result as *mut SharedPtrBuf,
                clobber_abi("C"),
            );
            #[cfg(target_arch = "arm")]
            {
                type CreateConvFn = unsafe extern "C" fn(*mut SharedPtrBuf, i32);
                let func: CreateConvFn = std::mem::transmute(self.create_convention);
                func(&mut result, conv);
            }
            (result.ptr as *mut c_void, result.ctrl as *mut c_void)
        }

        pub unsafe fn call_claim_hook(
            &self,
            mod_ptr: *mut c_void,
            hook_ptr: *mut c_void,
            hook_ctrl: *mut c_void,
        ) {
            #[repr(C)]
            struct SharedPtrBuf {
                ptr: usize,
                ctrl: usize,
            }
            let shared = SharedPtrBuf {
                ptr: hook_ptr as usize,
                ctrl: hook_ctrl as usize,
            };
            let mut result_buf = [0u8; 128];

            #[cfg(target_arch = "aarch64")]
            std::arch::asm!(
                "blr {fn_ptr}",
                fn_ptr = in(reg) self.claim_hook,
                in("x0") mod_ptr,
                in("x1") &shared as *const SharedPtrBuf,
                in("x8") result_buf.as_mut_ptr(),
                clobber_abi("C"),
            );
            #[cfg(target_arch = "arm")]
            {
                type ClaimHookFn = unsafe extern "C" fn(*mut u8, *mut c_void, *const SharedPtrBuf);
                let func: ClaimHookFn = std::mem::transmute(self.claim_hook);
                func(result_buf.as_mut_ptr(), mod_ptr, &shared);
            }
        }
    }

    #[repr(C)]
    pub struct HookSharedPtr {
        pub ptr: *mut c_void,
        pub ctrl: *mut c_void,
    }

    #[repr(C)]
    pub struct AndroidAbstractType {
        pub m_size: usize,
        pub m_kind: u8,
        pub _pad: [u8; 7],
    }

    #[repr(C)]
    pub struct AndroidVector {
        pub begin: *mut c_void,
        pub end: *mut c_void,
        pub cap: *mut c_void,
    }

    #[repr(C)]
    pub struct AndroidAbstractFunction {
        pub m_return: AndroidAbstractType,
        pub m_parameters: AndroidVector,
    }

    #[repr(C)]
    pub struct AndroidHandlerMetadata {
        pub convention_ptr: *mut c_void,
        pub convention_ctrl: *mut c_void,
        pub m_abstract: AndroidAbstractFunction,
    }

    impl AndroidHandlerMetadata {
        pub fn new(conv_ptr: *mut c_void, conv_ctrl: *mut c_void) -> Self {
            Self {
                convention_ptr: conv_ptr,
                convention_ctrl: conv_ctrl,
                m_abstract: AndroidAbstractFunction {
                    m_return: AndroidAbstractType {
                        m_size: 1,
                        m_kind: 0,
                        _pad: [0u8; 7],
                    },
                    m_parameters: AndroidVector {
                        begin: std::ptr::null_mut(),
                        end: std::ptr::null_mut(),
                        cap: std::ptr::null_mut(),
                    },
                },
            }
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct AndroidHookMetadata {
        pub priority: i32,
    }

    fn build_libcxx_string(name: &str) -> Option<([u8; 24], Option<*mut u8>)> {
        let bytes = name.as_bytes();
        let len = bytes.len();
        let mut str_buf = [0u8; 24];
        let mut heap: Option<*mut u8> = None;

        if len <= 22 {
            str_buf[0] = (len as u8) << 1;
            str_buf[1..1 + len].copy_from_slice(bytes);
        } else {
            unsafe extern "C" {
                fn malloc(size: usize) -> *mut c_void;
            }
            let ptr = unsafe { malloc(len + 1) } as *mut u8;
            if ptr.is_null() {
                return None;
            }
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);
                *ptr.add(len) = 0;
            }
            let str_ptr = str_buf.as_mut_ptr();
            unsafe {
                (str_ptr as *mut usize).write(len | 1);
                (str_ptr.add(8) as *mut usize).write(len);
                (str_ptr.add(16) as *mut *mut u8).write(ptr);
            }
            heap = Some(ptr);
        }
        Some((str_buf, heap))
    }

    pub unsafe fn call_hook_create(
        address: *mut c_void,
        detour: *mut c_void,
        name: &str,
        handler_meta: AndroidHandlerMetadata,
        hook_meta: AndroidHookMetadata,
    ) -> Option<HookSharedPtr> {
        let (str_buf, heap_ptr) = build_libcxx_string(name)?;
        let loader = init_loader()?;

        #[repr(C)]
        struct HookSharedPtrBuf {
            ptr: usize,
            ctrl: usize,
        }
        let mut result = HookSharedPtrBuf { ptr: 0, ctrl: 0 };

        #[cfg(target_arch = "aarch64")]
        std::arch::asm!(
            "blr {fn_ptr}",
            fn_ptr = in(reg) loader.hook_create,
            in("x0") address,
            in("x1") detour,
            in("x2") &str_buf as *const [u8; 24],
            in("x3") &handler_meta as *const AndroidHandlerMetadata,
            in("x4") hook_meta.priority,
            in("x8") &mut result as *mut HookSharedPtrBuf,
            clobber_abi("C"),
        );
        #[cfg(target_arch = "arm")]
        {
            type HookCreateFn = unsafe extern "C" fn(
                *mut HookSharedPtrBuf,
                *mut c_void,
                *mut c_void,
                *const [u8; 24],
                *const AndroidHandlerMetadata,
                AndroidHookMetadata,
            );
            let func: HookCreateFn = std::mem::transmute(loader.hook_create);
            func(
                &mut result,
                address,
                detour,
                &str_buf,
                &handler_meta,
                hook_meta,
            );
        }

        if let Some(p) = heap_ptr {
            unsafe extern "C" {
                fn free(ptr: *mut c_void);
            }
            unsafe { free(p as *mut c_void) };
        }

        let hook_ptr = result.ptr as *mut c_void;
        let hook_ctrl = result.ctrl as *mut c_void;
        if hook_ptr.is_null() {
            alog(b"call_hook_create: Hook::create returned null!\0");
            None
        } else {
            Some(HookSharedPtr {
                ptr: hook_ptr,
                ctrl: hook_ctrl,
            })
        }
    }
}

#[cfg(not(any(windows, target_os = "android")))]
pub(crate) mod geode_ffi {
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
        _name: &str,
        _convention: crate::CallingConvention,
        _priority: i32,
    ) -> Option<*mut c_void> {
        None
    }

    pub unsafe fn call_hook_enable(_hook_ptr: *mut c_void) -> bool {
        false
    }
}

pub use geode_ffi::*;

#[cfg(target_os = "android")]
pub fn android_log(msg: &[u8]) {
    unsafe { geode_ffi::alog(msg) }
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
        #[cfg(windows)]
        {
            use crate::stl::{StlSharedPtr, StlString};
            use crate::tulip::{HandlerMetadata, HookMetadata};

            let loader = init_loader()?;

            let tulip_conv = TulipConvention::from(convention);
            let mut conv_ptr: StlSharedPtr<c_void> = Default::default();
            (loader.create_convention)(&mut conv_ptr, tulip_conv as i32);

            if conv_ptr.is_null() {
                return None;
            }

            let display_name = StlString::from(name);
            let handler_meta = HandlerMetadata::with_convention(conv_ptr);
            let hook_meta = HookMetadata::new(priority);

            let hook_shared_ptr =
                call_hook_create(address, detour, &display_name, &handler_meta, &hook_meta)?;

            let hook_raw = hook_shared_ptr.as_ptr();
            std::mem::forget(hook_shared_ptr);
            Some(Self { ptr: hook_raw })
        }

        #[cfg(target_os = "android")]
        {
            use geode_ffi::{AndroidHandlerMetadata, AndroidHookMetadata, call_hook_create};

            let loader = init_loader()?;
            let tulip_conv = TulipConvention::from(convention);

            let (conv_ptr, conv_ctrl) = unsafe { loader.create_convention_fn(tulip_conv as i32) };
            if conv_ptr.is_null() {
                geode_ffi::alog(b"Hook::create: createConvention returned null\0");
                return None;
            }

            let handler_meta = AndroidHandlerMetadata::new(conv_ptr, conv_ctrl);
            let hook_meta = AndroidHookMetadata { priority };
            let result = call_hook_create(address, detour, name, handler_meta, hook_meta)?;

            let mod_ptr = Mod::get().map(|m| m.ptr()).unwrap_or(std::ptr::null_mut());
            if mod_ptr.is_null() {
                geode_ffi::alog(b"Hook::create: Mod ptr null, cannot claimHook\0");
                return None;
            }
            loader.call_claim_hook(mod_ptr, result.ptr, result.ctrl);

            Some(Self { ptr: result.ptr })
        }

        #[cfg(not(any(windows, target_os = "android")))]
        {
            let _ = (address, detour, name, convention, priority);
            None
        }
    }

    pub unsafe fn enable(&self) -> bool {
        #[cfg(windows)]
        return geode_ffi::call_hook_enable(self.ptr);
        #[cfg(not(windows))]
        false
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.ptr as *const c_void
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
                #[cfg(any(windows, target_os = "android"))]
                {
                    unsafe {
                        init_loader()
                            .and_then(|l| l.get_mod_id(self.ptr))
                            .unwrap_or_default()
                    }
                }
                #[cfg(not(any(windows, target_os = "android")))]
                {
                    String::new()
                }
            })
            .as_str()
    }

    pub unsafe fn take_next_mod() -> Option<*mut c_void> {
        #[cfg(windows)]
        {
            let loader = init_loader()?;
            let loader_ptr = (loader.loader_get)();
            if loader_ptr.is_null() {
                return None;
            }
            let ptr = (loader.take_next_mod)(loader_ptr);
            if ptr.is_null() { None } else { Some(ptr) }
        }
        #[cfg(target_os = "android")]
        {
            let loader = init_loader()?;
            let loader_ptr = (loader.loader_get)();
            if loader_ptr.is_null() {
                return None;
            }
            let ptr = (loader.take_next_mod)(loader_ptr);
            if ptr.is_null() { None } else { Some(ptr) }
        }
        #[cfg(not(any(windows, target_os = "android")))]
        {
            None
        }
    }
}

pub mod internal {
    use super::*;

    #[cfg(target_os = "android")]
    unsafe fn android_log(msg: &[u8]) {
        unsafe extern "C" {
            fn __android_log_print(prio: i32, tag: *const u8, fmt: *const u8, ...) -> i32;
        }
        unsafe { __android_log_print(3, b"geode-rs\0".as_ptr(), msg.as_ptr()) };
    }

    pub fn init_mod() {
        #[cfg(any(windows, target_os = "android"))]
        unsafe {
            if let Some(ptr) = Mod::take_next_mod() {
                Mod::set_shared(ptr);
            }
        }
    }
}
