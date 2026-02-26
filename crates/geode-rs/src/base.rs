#![allow(unsafe_op_in_unsafe_fn, clippy::missing_safety_doc)]

#[cfg(target_os = "windows")]
mod windows_mod {
    use std::sync::OnceLock;
    use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
    use windows::core::PCSTR;

    static BASE: OnceLock<usize> = OnceLock::new();
    static GEODE_BASE: OnceLock<usize> = OnceLock::new();
    static COCOS_BASE: OnceLock<usize> = OnceLock::new();
    static EXTENSIONS_BASE: OnceLock<usize> = OnceLock::new();

    pub fn get() -> usize {
        *BASE.get_or_init(|| unsafe { GetModuleHandleA(None).map(|h| h.0 as usize).unwrap_or(0) })
    }

    pub fn get_geode() -> usize {
        *GEODE_BASE.get_or_init(|| unsafe {
            GetModuleHandleA(windows::core::s!("Geode.dll"))
                .map(|h| h.0 as usize)
                .unwrap_or(0)
        })
    }

    pub fn get_cocos() -> usize {
        *COCOS_BASE.get_or_init(|| unsafe {
            GetModuleHandleA(windows::core::s!("libcocos2d.dll"))
                .map(|h| h.0 as usize)
                .unwrap_or(0)
        })
    }

    pub fn get_extensions() -> usize {
        *EXTENSIONS_BASE.get_or_init(|| unsafe {
            GetModuleHandleA(windows::core::s!("libExtensions.dll"))
                .map(|h| h.0 as usize)
                .unwrap_or(0)
        })
    }

    pub unsafe fn get_proc_address(module: usize, name: &[u8]) -> Option<usize> {
        let name_c = std::ffi::CString::new(name).ok()?;
        let addr = GetProcAddress(
            windows::Win32::Foundation::HMODULE(module as *mut _),
            PCSTR(name_c.as_ptr() as _),
        )?;
        Some(addr as usize)
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use std::sync::OnceLock;

    static BASE: OnceLock<usize> = OnceLock::new();
    static GEODE_BASE: OnceLock<usize> = OnceLock::new();

    pub fn get() -> usize {
        *BASE.get_or_init(|| unsafe {
            let image_count = _dyld_image_count();
            for i in 0..image_count {
                let name = std::ffi::CStr::from_ptr(_dyld_get_image_name(i));
                let name_str = name.to_string_lossy();
                if !name_str.ends_with(".dylib") {
                    return _dyld_get_image_vmaddr_slide(i) + 0x100000000;
                }
            }
            _dyld_get_image_vmaddr_slide(0) + 0x100000000
        })
    }

    pub fn get_geode() -> usize {
        *GEODE_BASE.get_or_init(|| unsafe {
            let image_count = _dyld_image_count();
            for i in 0..image_count {
                let name = std::ffi::CStr::from_ptr(_dyld_get_image_name(i));
                let name_str = name.to_string_lossy();
                if name_str.contains("Geode") {
                    return _dyld_get_image_vmaddr_slide(i);
                }
            }
            0
        })
    }

    unsafe extern "C" {
        fn _dyld_image_count() -> u32;
        fn _dyld_get_image_name(image_index: u32) -> *const std::os::raw::c_char;
        fn _dyld_get_image_vmaddr_slide(image_index: u32) -> usize;
    }
}

#[cfg(target_os = "ios")]
mod ios {
    use std::sync::OnceLock;

    static BASE: OnceLock<usize> = OnceLock::new();
    static GEODE_BASE: OnceLock<usize> = OnceLock::new();

    pub fn get() -> usize {
        *BASE.get_or_init(|| unsafe {
            let image_count = _dyld_image_count();
            for i in 0..image_count {
                let name = std::ffi::CStr::from_ptr(_dyld_get_image_name(i));
                let name_str = name.to_string_lossy();
                if name_str.ends_with("GeometryJump") {
                    return _dyld_get_image_vmaddr_slide(i) + 0x100000000;
                }
            }
            0
        })
    }

    pub fn get_geode() -> usize {
        *GEODE_BASE.get_or_init(|| unsafe {
            let image_count = _dyld_image_count();
            for i in 0..image_count {
                let name = std::ffi::CStr::from_ptr(_dyld_get_image_name(i));
                let name_str = name.to_string_lossy();
                if name_str.contains("Geode") {
                    return _dyld_get_image_vmaddr_slide(i);
                }
            }
            0
        })
    }

    unsafe extern "C" {
        fn _dyld_image_count() -> u32;
        fn _dyld_get_image_name(image_index: u32) -> *const std::os::raw::c_char;
        fn _dyld_get_image_vmaddr_slide(image_index: u32) -> usize;
    }
}

#[cfg(target_os = "android")]
mod android {
    use std::sync::OnceLock;

    static BASE: OnceLock<usize> = OnceLock::new();
    static GEODE_BASE: OnceLock<usize> = OnceLock::new();

    pub fn get() -> usize {
        *BASE.get_or_init(|| unsafe {
            let handle = dlopen(
                b"libcocos2dcpp.so\0".as_ptr() as *const std::os::raw::c_char,
                RTLD_LAZY | RTLD_NOLOAD,
            );
            if handle.is_null() {
                return 0;
            }

            let sym = dlsym(
                handle,
                b"JNI_OnLoad\0".as_ptr() as *const std::os::raw::c_char,
            );
            if sym.is_null() {
                dlclose(handle);
                return 0;
            }

            let mut info: Dl_info = std::mem::zeroed();
            if dladdr(sym, &mut info) != 0 {
                info.dli_fbase as usize
            } else {
                0
            }
        })
    }

    pub fn get_geode() -> usize {
        *GEODE_BASE.get_or_init(|| unsafe {
            let handle = dlopen(
                b"libGeode.so\0".as_ptr() as *const std::os::raw::c_char,
                RTLD_LAZY | RTLD_NOLOAD,
            );
            if handle.is_null() {
                return 0;
            }

            let sym = dlsym(
                handle,
                b"geodeImplicitEntry\0".as_ptr() as *const std::os::raw::c_char,
            );
            if sym.is_null() {
                dlclose(handle);
                return 0;
            }

            let mut info: Dl_info = std::mem::zeroed();
            if dladdr(sym, &mut info) != 0 {
                info.dli_fbase as usize
            } else {
                0
            }
        })
    }

    const RTLD_LAZY: std::os::raw::c_int = 0x00001;
    const RTLD_NOLOAD: std::os::raw::c_int = 0x00004;

    #[repr(C)]
    struct Dl_info {
        dli_fname: *const std::os::raw::c_char,
        dli_fbase: *mut std::os::raw::c_void,
        dli_sname: *const std::os::raw::c_char,
        dli_saddr: *mut std::os::raw::c_void,
    }

    fn get_gd_handle() -> *mut std::os::raw::c_void {
        static GD_HANDLE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
        let handle = *GD_HANDLE.get_or_init(|| {
            const RTLD_LAZY: std::os::raw::c_int = 0x1;
            const RTLD_NOLOAD: std::os::raw::c_int = 0x4;
            unsafe {
                dlopen(
                    b"libcocos2dcpp.so\0".as_ptr() as *const std::os::raw::c_char,
                    RTLD_LAZY | RTLD_NOLOAD,
                ) as usize
            }
        });
        handle as *mut std::os::raw::c_void
    }

    pub fn android_resolve_sym(sym_bytes: &[u8], slot: &std::sync::atomic::AtomicUsize) -> usize {
        use std::sync::atomic::Ordering;
        const SENTINEL: usize = usize::MAX;

        let cached = slot.load(Ordering::Relaxed);
        if cached == SENTINEL {
            return 0;
        }
        if cached != 0 {
            return cached;
        }

        if sym_bytes.is_empty() || sym_bytes[0] == 0 {
            slot.store(SENTINEL, Ordering::Relaxed);
            return 0;
        }

        let handle = get_gd_handle();
        let addr = unsafe {
            let sym = dlsym(handle, sym_bytes.as_ptr() as *const std::os::raw::c_char);
            sym as usize
        };

        if addr == 0 {
            slot.store(SENTINEL, Ordering::Relaxed);
            return 0;
        }

        let base = get();
        let offset = if addr > base { addr - base } else { 0 };
        slot.store(
            if offset == 0 { SENTINEL } else { offset },
            Ordering::Relaxed,
        );
        offset
    }

    unsafe extern "C" {
        fn dlopen(
            filename: *const std::os::raw::c_char,
            flag: std::os::raw::c_int,
        ) -> *mut std::os::raw::c_void;
        fn dlsym(
            handle: *mut std::os::raw::c_void,
            symbol: *const std::os::raw::c_char,
        ) -> *mut std::os::raw::c_void;
        fn dlclose(handle: *mut std::os::raw::c_void) -> std::os::raw::c_int;
        fn dladdr(addr: *const std::os::raw::c_void, info: *mut Dl_info) -> std::os::raw::c_int;
    }
}

#[cfg(target_os = "windows")]
pub use windows_mod::{get, get_cocos, get_extensions, get_geode, get_proc_address};

#[cfg(target_os = "macos")]
pub use macos::{get, get_geode};

#[cfg(target_os = "ios")]
pub use ios::{get, get_geode};

#[cfg(target_os = "android")]
pub use android::{android_resolve_sym, get, get_geode};

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
)))]
pub fn get() -> usize {
    0
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
)))]
pub fn get_geode() -> usize {
    0
}
