const SSO_CAPACITY: usize = 7;
const SSO_CAPACITY_WITH_NULL: usize = SSO_CAPACITY + 1;

#[repr(C)]
union Storage {
    buffer: [u16; SSO_CAPACITY_WITH_NULL],
    heap: *mut u16,
}

impl Storage {
    fn new() -> Self {
        Self {
            buffer: [0; SSO_CAPACITY_WITH_NULL],
        }
    }

    fn from_heap(ptr: *mut u16) -> Self {
        Self { heap: ptr }
    }

    fn from_slice(slice: &[u16]) -> Self {
        let mut storage = Self::new();
        if slice.len() <= SSO_CAPACITY {
            unsafe {
                storage.buffer[..slice.len()].copy_from_slice(slice);
                storage.buffer[slice.len()] = 0;
            }
        } else {
            panic!("slice too large for MSVC wide-string SSO");
        }
        storage
    }
}

#[repr(C)]
pub struct wstring {
    storage: Storage,
    size: usize,
    capacity: usize,
}

impl wstring {
    pub fn new() -> Self {
        Self {
            storage: Storage::new(),
            size: 0,
            capacity: SSO_CAPACITY,
        }
    }

    pub fn new_from_slice(data: &[u16]) -> Self {
        let (storage, capacity) = if data.len() <= SSO_CAPACITY {
            (Storage::from_slice(data), SSO_CAPACITY)
        } else {
            unsafe {
                let elems = data.len() + 1;
                let memory = libc::malloc(elems * std::mem::size_of::<u16>()) as *mut u16;
                assert!(!memory.is_null(), "failed to allocate MSVC wstring");
                std::ptr::copy_nonoverlapping(data.as_ptr(), memory, data.len());
                *memory.add(data.len()) = 0;
                (Storage::from_heap(memory), data.len())
            }
        };

        Self {
            storage,
            size: data.len(),
            capacity,
        }
    }

    pub fn is_heap_allocated(&self) -> bool {
        self.capacity > SSO_CAPACITY
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn data(&self) -> *const u16 {
        if self.is_heap_allocated() {
            unsafe { self.storage.heap }
        } else {
            unsafe { self.storage.buffer.as_ptr() }
        }
    }

    pub fn as_slice(&self) -> &[u16] {
        unsafe { std::slice::from_raw_parts(self.data(), self.size) }
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_slice())
    }
}

impl Default for wstring {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for wstring {
    fn clone(&self) -> Self {
        Self::new_from_slice(self.as_slice())
    }
}

impl Drop for wstring {
    fn drop(&mut self) {
        if self.is_heap_allocated() {
            unsafe {
                libc::free(self.storage.heap.cast());
            }
        }
    }
}
