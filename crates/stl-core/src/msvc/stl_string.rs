use std::fmt::Display;

use crate::types as c;

const SSO_CAPACITY: usize = 15;
const SSO_CAPACITY_WITH_NULL: usize = SSO_CAPACITY + 1;

#[repr(C)]
union Storage {
    buffer: [u8; SSO_CAPACITY_WITH_NULL],
    heap: *mut c::char,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            buffer: [0; SSO_CAPACITY_WITH_NULL],
        }
    }

    pub fn from_heap(ptr: *mut c::char) -> Self {
        Self { heap: ptr }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut storage = Self::new();
        if slice.len() <= SSO_CAPACITY {
            unsafe {
                storage.buffer[..slice.len()].copy_from_slice(slice);
                storage.buffer[slice.len()] = 0;
            }
        } else {
            panic!("Slice too large for small string optimization");
        }
        storage
    }
}

#[repr(C)]
pub struct string {
    storage: Storage,

    // invariant: capacity >= size, capacity >= SSO_CAPACITY,
    // neither size nor capacity include null terminator
    size: c::size_t,
    capacity: c::size_t,
}

impl string {
    pub fn new() -> Self {
        Self {
            storage: Storage::new(),
            size: 0,
            capacity: 15, // 15 bytes of sso and null
        }
    }

    pub fn data(&self) -> *const c::char {
        if self.is_heap_allocated() {
            unsafe { self.storage.heap as *const c::char }
        } else {
            unsafe { self.storage.buffer.as_ptr() as *const c::char }
        }
    }

    pub fn data_mut(&mut self) -> *mut c::char {
        if self.is_heap_allocated() {
            unsafe { self.storage.heap }
        } else {
            unsafe { self.storage.buffer.as_mut_ptr() as *mut c::char }
        }
    }

    pub fn new_from_str(data: &str) -> Self {
        let bytes = data.as_bytes();
        let (storage, capacity) = if bytes.len() <= 15 {
            (Storage::from_slice(bytes), 15)
        } else {
            unsafe {
                let memory = libc::malloc(bytes.len() + 1) as *mut c::char;
                assert!(!memory.is_null(), "Failed to allocate memory for string");
                std::ptr::copy_nonoverlapping(
                    bytes.as_ptr() as *const c::char,
                    memory,
                    bytes.len(),
                );
                *memory.add(bytes.len()) = 0;
                (Storage::from_heap(memory), bytes.len())
            }
        };

        Self {
            storage,
            size: bytes.len(),
            capacity,
        }
    }

    pub fn is_heap_allocated(&self) -> bool {
        self.capacity > SSO_CAPACITY
    }

    /// Like C++ reserve, increases capacity to be max(new_cap, capacity)
    pub fn reserve(&mut self, new_cap: usize) {
        if new_cap <= self.capacity {
            return;
        }

        assert!(new_cap > SSO_CAPACITY); // should be impossible to fail

        let new_size = new_cap + 1; // +1 for null terminator
        let new_ptr = unsafe { libc::malloc(new_size) as *mut c::char };
        assert!(!new_ptr.is_null(), "Failed to allocate memory for string");

        // copy the data
        unsafe {
            std::ptr::copy_nonoverlapping(self.data(), new_ptr, self.size);
        }

        if self.is_heap_allocated() {
            // free previous allocation
            unsafe {
                libc::free(self.storage.heap as *mut libc::c_void);
            }
        }

        self.storage = Storage::from_heap(new_ptr);
        self.capacity = new_cap;
    }

    /// Like Rust reserve, increases capacity to be at least size + additional
    pub fn reserve_extra(&mut self, extra: usize) {
        let new_cap = (self.size + extra).next_power_of_two();
        self.reserve(new_cap);
    }

    /// Ensures `n` bytes can be appended to the string, reallocates if needed.
    /// Returns the pointer where data must be written, inserts the null terminator and increments size for you :)
    fn make_room_for(&mut self, n: usize) -> *mut c::char {
        self.reserve_extra(n);

        unsafe {
            let write_ptr = self.data_mut().add(self.size);
            write_ptr.add(n).write(0); // null terminator
            self.size += n;
            write_ptr
        }
    }

    pub fn push_back(&mut self, c: c::char) {
        unsafe {
            *self.make_room_for(1) = c;
        }
    }

    pub fn append(&mut self, str: &str) {
        let bytes = str.as_bytes();
        let n = bytes.len();
        let ptr = self.make_room_for(n);

        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c::char, ptr, n);
        }
    }

    fn erase_chars(&mut self, start_idx: usize, end_idx: usize) {
        assert!(start_idx <= end_idx && end_idx <= self.size);

        let num_to_erase = end_idx - start_idx;
        if num_to_erase == 0 {
            return;
        }

        let ptr = self.data_mut();
        unsafe {
            std::ptr::copy(ptr.add(end_idx), ptr.add(start_idx), self.size - end_idx);
            *ptr.add(self.size - num_to_erase) = 0;
        }

        self.size -= num_to_erase;
    }

    pub fn clear(&mut self) {
        self.erase_chars(0, self.size);
    }

    pub fn erase(&mut self, pos: usize, len: usize) {
        assert!(pos <= self.size);
        let end_idx = std::cmp::min(pos + len, self.size);
        self.erase_chars(pos, end_idx);
    }
}

impl Default for string {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for string {
    fn drop(&mut self) {
        if self.is_heap_allocated() {
            unsafe {
                libc::free(self.storage.heap as *mut libc::c_void);
            }
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
        let slice = if self.is_heap_allocated() {
            unsafe { std::slice::from_raw_parts(self.storage.heap as *const u8, self.size) }
        } else {
            unsafe { std::slice::from_raw_parts(self.storage.buffer.as_ptr(), self.size) }
        };

        let s = std::str::from_utf8(slice).map_err(|_| std::fmt::Error)?;
        write!(f, "{s}")
    }
}
