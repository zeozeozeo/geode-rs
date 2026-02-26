/// yay.
use std::{
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

#[repr(C)]
#[doc(hidden)]
pub struct RefCountVtable {
    destroy: extern "C" fn(*mut RefCountBase),
    delete_this: extern "C" fn(*mut RefCountBase),
}

/// Base that can never be constructed
#[repr(C)]
#[doc(hidden)]
pub struct RefCountBase {
    vtable: *const RefCountVtable,
    strong: AtomicU32,
    weak: AtomicU32,
}

/// Actual RefCount struct
#[repr(C)]
#[doc(hidden)]
pub struct RefCount<T: SharedPtrVtable> {
    base: RefCountBase,
    data: *mut T,
}

#[repr(C)]
pub struct shared_ptr<T: SharedPtrVtable> {
    ptr: *mut T,
    ref_count: *mut RefCountBase,
}

impl RefCountBase {
    fn destroy(&self) {
        unsafe {
            (self.vtable.as_ref().unwrap().destroy)(self as *const _ as *mut _);
        }
    }

    fn delete_this(&self) {
        unsafe {
            (self.vtable.as_ref().unwrap().delete_this)(self as *const _ as *mut _);
        }
    }

    fn inc_ref(&self) {
        self.strong.fetch_add(1, Ordering::Relaxed);
    }

    fn inc_weak_ref(&self) {
        self.weak.fetch_add(1, Ordering::Relaxed);
    }

    fn dec_ref(&self) {
        if self.strong.fetch_sub(1, Ordering::AcqRel) == 0 {
            // destroy the object and release weak reference
            self.destroy();
            self.dec_weak_ref();
        }
    }

    fn dec_weak_ref(&self) {
        if self.weak.fetch_sub(1, Ordering::AcqRel) == 0 {
            // delete the refcount itself
            self.delete_this();
        }
    }
}

impl<T: SharedPtrVtable> shared_ptr<T> {
    pub fn new(val: T) -> Self {
        let vtable = &<T as SharedPtrVtable>::VTABLE;
        let boxed = Box::new(val);

        let ref_count = Box::new(RefCount::<T> {
            base: RefCountBase {
                vtable,
                strong: AtomicU32::new(1),
                weak: AtomicU32::new(1),
            },
            data: Box::into_raw(boxed),
        });

        let ptr = ref_count.data;
        Self::from_raw_parts(ptr, Box::into_raw(ref_count) as *mut RefCountBase)
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }

    fn from_raw_parts(ptr: *mut T, ref_count: *mut RefCountBase) -> Self {
        Self { ptr, ref_count }
    }

    fn refcount(&self) -> Option<&RefCountBase> {
        unsafe { self.ref_count.as_ref() }
    }

    pub fn use_count(&self) -> u32 {
        self.refcount()
            .map_or(0, |rc| rc.strong.load(Ordering::Relaxed))
    }

    fn inc_ref(&self, weak: bool) {
        let rc = self.refcount().expect("called inc_ref on null shared_ptr");
        if weak {
            rc.inc_weak_ref();
        } else {
            rc.inc_ref();
        }
    }

    fn dec_ref(&self, weak: bool) {
        let rc = self.refcount().expect("called dec_ref on null shared_ptr");
        if weak {
            rc.dec_weak_ref();
        } else {
            rc.dec_ref();
        }
    }
}

impl<T: SharedPtrVtable> Deref for shared_ptr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        assert!(!self.ptr.is_null(), "Dereferencing null shared_ptr");
        unsafe { &*self.ptr }
    }
}

impl<T: SharedPtrVtable> DerefMut for shared_ptr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.ptr.is_null(), "Dereferencing null shared_ptr");
        unsafe { &mut *self.ptr }
    }
}

impl<T: SharedPtrVtable> Drop for shared_ptr<T> {
    fn drop(&mut self) {
        if !self.is_null() {
            self.dec_ref(false);
        }
    }
}

impl<T: SharedPtrVtable> Default for shared_ptr<T> {
    fn default() -> Self {
        Self::from_raw_parts(std::ptr::null_mut(), std::ptr::null_mut())
    }
}

impl<T: SharedPtrVtable> Clone for shared_ptr<T> {
    fn clone(&self) -> Self {
        self.inc_ref(false);
        Self::from_raw_parts(self.ptr, self.ref_count)
    }
}

extern "C" fn destroy_impl<T: SharedPtrVtable>(ptr: *mut RefCountBase) {
    unsafe {
        let rc = ptr as *mut RefCount<T>;
        let _ = Box::from_raw((*rc).data);
    }
}

extern "C" fn delete_this_impl<T: SharedPtrVtable>(ptr: *mut RefCountBase) {
    unsafe {
        let rc = ptr as *mut RefCount<T>;
        let _ = Box::from_raw(rc);
    }
}

pub trait SharedPtrVtable: Sized {
    const VTABLE: RefCountVtable = RefCountVtable {
        destroy: destroy_impl::<Self>,
        delete_this: delete_this_impl::<Self>,
    };
}

// generic impl
impl<T> SharedPtrVtable for T {}
