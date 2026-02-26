use crate::types as c;
use std::fmt::Display;

#[repr(C)]
struct StringRep {
    m_size: c::size_t,
    m_capacity: c::size_t,
    m_refcount: i32,
    // <data>
}

impl StringRep {
    fn data_ptr(&self) -> *const c::char {
        unsafe {
            let rep_ptr = self as *const StringRep as *const u8;
            rep_ptr.add(std::mem::size_of::<StringRep>()) as *const c::char
        }
    }

    fn data_ptr_mut(&mut self) -> *mut c::char {
        unsafe {
            let rep_ptr = self as *mut StringRep as *mut u8;
            rep_ptr.add(std::mem::size_of::<StringRep>()) as *mut c::char
        }
    }
}

static EMPTY_REP: StringRepStatic = StringRepStatic {
    m_size: 0,
    m_capacity: 0,
    m_refcount: -1,
    m_null: 0,
};

#[repr(C)]
struct StringRepStatic {
    m_size: c::size_t,
    m_capacity: c::size_t,
    m_refcount: i32,
    m_null: u8,
}

#[repr(C)]
pub struct string {
    m_data: *mut StringRep,
}

unsafe impl Send for string {}
unsafe impl Sync for string {}

impl string {
    fn empty_rep() -> *mut StringRep {
        &EMPTY_REP as *const StringRepStatic as *const StringRep as *mut StringRep
    }

    pub fn new() -> Self {
        Self {
            m_data: Self::empty_rep(),
        }
    }

    fn rep(&self) -> &StringRep {
        unsafe { &*self.m_data }
    }

    fn rep_mut(&mut self) -> &mut StringRep {
        unsafe { &mut *self.m_data }
    }

    pub fn size(&self) -> usize {
        self.rep().m_size
    }

    pub fn capacity(&self) -> usize {
        self.rep().m_capacity
    }

    pub fn data(&self) -> *const c::char {
        self.rep().data_ptr()
    }

    pub fn data_mut(&mut self) -> *mut c::char {
        self.rep_mut().data_ptr_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn is_static(&self) -> bool {
        self.rep().m_refcount < 0
    }

    unsafe fn alloc_rep(capacity: usize) -> *mut StringRep {
        let rep_size = std::mem::size_of::<StringRep>();
        let total = rep_size + capacity + 1;
        let ptr = unsafe { libc::malloc(total) as *mut StringRep };
        assert!(!ptr.is_null(), "Failed to allocate gnustl string");
        unsafe {
            (*ptr).m_size = 0;
            (*ptr).m_capacity = capacity;
            (*ptr).m_refcount = 0;
        }
        ptr
    }

    pub fn new_from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        let len = bytes.len();

        if len == 0 {
            return Self::new();
        }

        unsafe {
            let rep = Self::alloc_rep(len);
            let data = (*rep).data_ptr_mut();
            std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c::char, data, len);
            *data.add(len) = 0;
            (*rep).m_size = len;
            Self { m_data: rep }
        }
    }

    fn detach_or_clone_if_needed(&mut self, new_cap: usize) {
        let old_rep = self.rep();
        let is_shared = old_rep.m_refcount > 0;
        let needs_realloc = new_cap > old_rep.m_capacity;

        if self.is_static() || is_shared || needs_realloc {
            let old_size = old_rep.m_size;
            let actual_cap = new_cap.max(old_rep.m_capacity);

            unsafe {
                let new_rep = Self::alloc_rep(actual_cap);
                let old_data = old_rep.data_ptr();
                let new_data = (*new_rep).data_ptr_mut();
                std::ptr::copy_nonoverlapping(old_data, new_data, old_size);
                *new_data.add(old_size) = 0;
                (*new_rep).m_size = old_size;

                // release old if heap-allocated
                if !self.is_static() {
                    let old_rc = (*self.m_data).m_refcount;
                    if old_rc <= 0 {
                        libc::free(self.m_data as *mut libc::c_void);
                    } else {
                        (*self.m_data).m_refcount -= 1;
                    }
                }

                self.m_data = new_rep;
            }
        }
    }

    pub fn push_back(&mut self, c: c::char) {
        let new_size = self.size() + 1;
        self.detach_or_clone_if_needed(new_size);
        unsafe {
            let rep = &mut *self.m_data;
            let data = rep.data_ptr_mut();
            *data.add(rep.m_size) = c;
            rep.m_size += 1;
            *data.add(rep.m_size) = 0;
        }
    }

    pub fn clear(&mut self) {
        if self.is_static() {
            return;
        }
        unsafe {
            (*self.m_data).m_size = 0;
            *(*self.m_data).data_ptr_mut() = 0;
        }
    }
}

impl Default for string {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for string {
    fn drop(&mut self) {
        if !self.is_static() {
            unsafe {
                let rep = &mut *self.m_data;
                if rep.m_refcount <= 0 {
                    // last owner - free
                    libc::free(self.m_data as *mut libc::c_void);
                } else {
                    rep.m_refcount -= 1;
                }
            }
        }
    }
}

impl Clone for string {
    fn clone(&self) -> Self {
        if self.is_static() {
            return Self::new();
        }
        unsafe {
            (*self.m_data).m_refcount += 1;
        }
        Self {
            m_data: self.m_data,
        }
    }
}

impl From<&str> for string {
    fn from(s: &str) -> Self {
        Self::new_from_str(s)
    }
}

impl Display for string {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice = unsafe { std::slice::from_raw_parts(self.data() as *const u8, self.size()) };
        let s = std::str::from_utf8(slice).map_err(|_| std::fmt::Error)?;
        write!(f, "{s}")
    }
}
