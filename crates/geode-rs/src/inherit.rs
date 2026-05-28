use std::ffi::c_void;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::base::{SymbolScope, resolve_symbol};
use crate::classes::CCObject;

pub struct Obj<T> {
    ptr: NonNull<T>,
}

impl<T> Obj<T> {
    pub fn from_raw(ptr: *mut T) -> Self {
        Self {
            ptr: NonNull::new(ptr).expect("Cocos function returned null"),
        }
    }

    pub fn from_non_null(ptr: NonNull<T>) -> Self {
        Self { ptr }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub fn as_non_null(&self) -> NonNull<T> {
        self.ptr
    }

    pub fn cast<U>(self) -> Obj<U> {
        Obj {
            ptr: self.ptr.cast(),
        }
    }
}

impl<T> Deref for Obj<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for Obj<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
const CXX_OPERATOR_NEW: &[u8] = b"??2@YAPEAX_K@Z\0";
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
const CXX_OPERATOR_NEW: &[u8] = b"??2@YAPAXI@Z\0";
#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
const CXX_OPERATOR_DELETE: &[u8] = b"??3@YAXPEAX@Z\0";
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
const CXX_OPERATOR_DELETE: &[u8] = b"??3@YAXPAX@Z\0";
#[cfg(all(not(target_os = "windows"), target_pointer_width = "64"))]
const CXX_OPERATOR_NEW: &[u8] = b"_Znwm\0";
#[cfg(all(not(target_os = "windows"), target_pointer_width = "32"))]
const CXX_OPERATOR_NEW: &[u8] = b"_Znwj\0";
#[cfg(not(target_os = "windows"))]
const CXX_OPERATOR_DELETE: &[u8] = b"_ZdlPv\0";

static CXX_OPERATOR_NEW_ADDR: AtomicUsize = AtomicUsize::new(0);
static CXX_OPERATOR_DELETE_ADDR: AtomicUsize = AtomicUsize::new(0);

pub fn try_cxx_operator_new(size: usize) -> Option<NonNull<c_void>> {
    let ptr = unsafe {
        let addr = resolve_cxx_operator_new();
        if addr != 0 {
            let func: unsafe extern "C" fn(usize) -> *mut c_void = std::mem::transmute(addr);
            func(size)
        } else {
            platform_malloc(size)
        }
    };

    NonNull::new(ptr)
}

pub unsafe fn cxx_operator_new(size: usize) -> *mut c_void {
    try_cxx_operator_new(size).map_or(std::ptr::null_mut(), NonNull::as_ptr)
}

pub unsafe fn cxx_operator_delete(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }

    let addr = resolve_cxx_operator_delete();
    if addr != 0 {
        unsafe {
            let func: unsafe extern "C" fn(*mut c_void) = std::mem::transmute(addr);
            func(ptr);
        }
        return;
    }

    platform_free(ptr);
}

pub fn try_alloc_cocos_object<T>() -> Option<NonNull<T>> {
    try_cxx_operator_new(std::mem::size_of::<T>()).map(NonNull::cast)
}

pub unsafe fn alloc_cocos_object<T>() -> *mut T {
    try_alloc_cocos_object::<T>().map_or(std::ptr::null_mut(), NonNull::as_ptr)
}

pub fn try_autorelease_as_ccobject<T>(this: NonNull<T>) -> NonNull<T> {
    unsafe {
        (*(this.as_ptr() as *mut CCObject)).autorelease();
    }
    this
}

pub unsafe fn as_primary_base<T, U>(this: *mut T) -> *mut U {
    this.cast()
}

pub unsafe fn autorelease_as_ccobject<T>(this: *mut T) -> *mut T {
    let Some(this) = NonNull::new(this) else {
        return std::ptr::null_mut();
    };
    try_autorelease_as_ccobject(this).as_ptr()
}

pub fn try_primary_vtable<T>(this: NonNull<T>) -> Option<NonNull<*mut c_void>> {
    let vtable = unsafe { *(this.as_ptr() as *const *mut *mut c_void) };
    NonNull::new(vtable)
}

pub fn try_set_primary_vtable<T>(this: NonNull<T>, vtable: NonNull<*mut c_void>) {
    unsafe {
        *(this.as_ptr() as *mut *mut *mut c_void) = vtable.as_ptr();
    }
}

pub unsafe fn primary_vtable<T>(this: *const T) -> *mut *mut c_void {
    let Some(this) = NonNull::new(this.cast_mut()) else {
        return std::ptr::null_mut();
    };
    try_primary_vtable(this).map_or(std::ptr::null_mut(), NonNull::as_ptr)
}

pub unsafe fn set_primary_vtable<T>(this: *mut T, vtable: *mut *mut c_void) {
    let Some(this) = NonNull::new(this) else {
        return;
    };
    let Some(vtable) = NonNull::new(vtable) else {
        return;
    };
    try_set_primary_vtable(this, vtable);
}

pub fn try_clone_primary_vtable<T>(this: NonNull<T>, slots: usize) -> Option<Box<[*mut c_void]>> {
    if slots == 0 {
        return None;
    }

    let vtable = try_primary_vtable(this)?;
    let mut cloned = Vec::with_capacity(slots);
    for i in 0..slots {
        cloned.push(unsafe { *vtable.as_ptr().add(i) });
    }
    Some(cloned.into_boxed_slice())
}

pub unsafe fn clone_primary_vtable<T>(this: *const T, slots: usize) -> Box<[*mut c_void]> {
    let Some(this) = NonNull::new(this.cast_mut()) else {
        return Box::new([]);
    };
    try_clone_primary_vtable(this, slots).unwrap_or_else(|| Box::new([]))
}

fn resolve_cxx_operator_new() -> usize {
    resolve_runtime_symbol(CXX_OPERATOR_NEW, &CXX_OPERATOR_NEW_ADDR)
}

fn resolve_cxx_operator_delete() -> usize {
    resolve_runtime_symbol(CXX_OPERATOR_DELETE, &CXX_OPERATOR_DELETE_ADDR)
}

fn resolve_runtime_symbol(name: &[u8], slot: &AtomicUsize) -> usize {
    let cached = slot.load(Ordering::Relaxed);
    if cached != 0 && cached != usize::MAX {
        return cached;
    }

    let addr = resolve_symbol(SymbolScope::Process, name, slot);
    if addr != 0 {
        return addr;
    }

    let addr = resolve_runtime_symbol_fallback(name);
    if addr != 0 {
        slot.store(addr, Ordering::Relaxed);
    }
    addr
}

#[cfg(target_os = "windows")]
fn resolve_runtime_symbol_fallback(name: &[u8]) -> usize {
    use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryA};
    use windows::core::PCSTR;

    const MODULES: &[&[u8]] = &[
        b"vcruntime140.dll\0",
        b"vcruntime140_1.dll\0",
        b"msvcp140.dll\0",
        b"ucrtbase.dll\0",
    ];

    for module_name in MODULES {
        let handle = unsafe {
            GetModuleHandleA(PCSTR(module_name.as_ptr()))
                .or_else(|_| LoadLibraryA(PCSTR(module_name.as_ptr())))
        };
        let Ok(handle) = handle else {
            continue;
        };

        let Some(addr) = (unsafe { GetProcAddress(handle, PCSTR(name.as_ptr())) }) else {
            continue;
        };
        return addr as usize;
    }

    0
}

#[cfg(not(target_os = "windows"))]
fn resolve_runtime_symbol_fallback(_name: &[u8]) -> usize {
    0
}

#[cfg(target_os = "windows")]
static CRT_MALLOC_ADDR: AtomicUsize = AtomicUsize::new(0);
#[cfg(target_os = "windows")]
static CRT_FREE_ADDR: AtomicUsize = AtomicUsize::new(0);

#[cfg(target_os = "windows")]
fn platform_malloc(size: usize) -> *mut c_void {
    let addr = resolve_windows_crt_heap_symbol(b"malloc\0", &CRT_MALLOC_ADDR);
    if addr == 0 {
        report_runtime_error("failed to resolve CRT malloc");
        return std::ptr::null_mut();
    }

    unsafe {
        let func: unsafe extern "C" fn(usize) -> *mut c_void = std::mem::transmute(addr);
        func(size)
    }
}

#[cfg(target_os = "windows")]
fn platform_free(ptr: *mut c_void) {
    let addr = resolve_windows_crt_heap_symbol(b"free\0", &CRT_FREE_ADDR);
    if addr == 0 {
        report_runtime_error("failed to resolve CRT free");
        return;
    }

    unsafe {
        let func: unsafe extern "C" fn(*mut c_void) = std::mem::transmute(addr);
        func(ptr);
    }
}

#[cfg(target_os = "windows")]
fn resolve_windows_crt_heap_symbol(name: &[u8], slot: &AtomicUsize) -> usize {
    use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryA};
    use windows::core::PCSTR;

    let cached = slot.load(Ordering::Relaxed);
    if cached != 0 && cached != usize::MAX {
        return cached;
    }

    const MODULES: &[&[u8]] = &[
        b"api-ms-win-crt-heap-l1-1-0.dll\0",
        b"ucrtbase.dll\0",
        b"msvcrt.dll\0",
    ];

    for module_name in MODULES {
        let handle = unsafe {
            GetModuleHandleA(PCSTR(module_name.as_ptr()))
                .or_else(|_| LoadLibraryA(PCSTR(module_name.as_ptr())))
        };
        let Ok(handle) = handle else {
            continue;
        };

        let Some(addr) = (unsafe { GetProcAddress(handle, PCSTR(name.as_ptr())) }) else {
            continue;
        };
        let addr = addr as usize;
        slot.store(addr, Ordering::Relaxed);
        return addr;
    }

    slot.store(usize::MAX, Ordering::Relaxed);
    0
}

#[cfg(target_os = "android")]
fn platform_malloc(size: usize) -> *mut c_void {
    unsafe { libc::malloc(size) }
}

#[cfg(target_os = "android")]
fn platform_free(ptr: *mut c_void) {
    unsafe {
        libc::free(ptr);
    }
}

#[cfg(not(any(target_os = "windows", target_os = "android")))]
fn platform_malloc(_size: usize) -> *mut c_void {
    report_runtime_error("no platform malloc fallback available");
    std::ptr::null_mut()
}

#[cfg(not(any(target_os = "windows", target_os = "android")))]
fn platform_free(_ptr: *mut c_void) {
    report_runtime_error("no platform free fallback available");
}

#[allow(dead_code)]
fn report_runtime_error(message: &str) {
    #[cfg(target_os = "android")]
    crate::loader::android_log(format!("ERROR: {message}\0").as_bytes());
    #[cfg(not(target_os = "android"))]
    eprintln!("[geode-rs] ERROR: {message}");
}
