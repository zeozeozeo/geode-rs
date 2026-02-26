use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

#[derive(Default)]
#[repr(C)]
pub struct vector<T> {
    begin: *mut T,
    end: *mut T,
    end_cap: *mut T,
}

impl<T> vector<T> {
    pub fn new() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            end: std::ptr::null_mut(),
            end_cap: std::ptr::null_mut(),
        }
    }

    pub fn data(&self) -> *mut T {
        self.begin
    }

    fn data_nonnull(&self) -> *mut T {
        // this is what from_raw_parts suggests doing, as passing null to it is invalid
        if self.begin.is_null() {
            NonNull::dangling().as_ptr()
        } else {
            self.begin
        }
    }

    pub fn begin(&self) -> *mut T {
        self.begin
    }

    pub fn end(&self) -> *mut T {
        self.end
    }

    pub fn end_cap(&self) -> *mut T {
        self.end_cap
    }

    pub fn size(&self) -> usize {
        unsafe { self.end.offset_from(self.begin) as usize }
    }

    pub fn capacity(&self) -> usize {
        unsafe { self.end_cap.offset_from(self.begin) as usize }
    }

    pub fn empty(&self) -> bool {
        self.begin == self.end
    }

    pub fn clear(&mut self) {
        for ptr in self.as_mut() {
            unsafe {
                std::ptr::drop_in_place(ptr);
            }
        }

        self.end = self.begin;
    }

    /// Like C++ reserve, increases capacity to be max(new_cap, capacity)
    pub fn reserve(&mut self, new_cap: usize) {
        if new_cap <= self.capacity() {
            return;
        }

        let new_size = new_cap * std::mem::size_of::<T>();
        let new_ptr = unsafe { libc::malloc(new_size) as *mut T };
        let prev_size = self.size();
        assert!(!new_ptr.is_null(), "Failed to allocate memory for vector");

        if !self.begin.is_null() {
            unsafe {
                std::ptr::copy_nonoverlapping(self.begin, new_ptr, self.size());
                libc::free(self.begin as *mut libc::c_void);
            }
        }

        self.begin = new_ptr;
        self.end = unsafe { new_ptr.add(prev_size) };
        self.end_cap = unsafe { new_ptr.add(new_cap) };
    }

    /// Like Rust reserve, increases capacity to be at least size + additional
    pub fn reserve_extra(&mut self, additional: usize) {
        let new_cap = (self.len() + additional).next_power_of_two();
        self.reserve(new_cap);
    }

    fn ensure_capacity(&mut self, num: usize) {
        if self.size() + num > self.capacity() {
            let new_cap = if self.capacity() == 0 {
                pick_default_capacity(std::mem::size_of::<T>())
            } else {
                self.capacity() * 2
            };

            self.reserve(new_cap);
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.ensure_capacity(1);

        unsafe {
            std::ptr::write(self.end, value);
            self.end = self.end.add(1);
        }
    }

    pub fn insert(&mut self, values: &[T])
    where
        T: Copy,
    {
        let num = values.len();
        self.ensure_capacity(num);

        unsafe {
            std::ptr::copy_nonoverlapping(values.as_ptr(), self.end, num);
            self.end = self.end.add(num);
        }
    }
}

impl<T> Drop for vector<T> {
    fn drop(&mut self) {
        self.clear();

        if !self.begin.is_null() {
            unsafe {
                libc::free(self.begin as *mut libc::c_void);
            }
        }
    }
}

impl<T: Clone> Clone for vector<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Self::new();
        let size = self.size();
        new_vec.reserve(size);

        for item in self.deref() {
            new_vec.push_back(item.clone());
        }

        new_vec
    }
}

fn pick_default_capacity(elem_size: usize) -> usize {
    if elem_size == 1 {
        32
    } else if elem_size == 2 {
        16
    } else if elem_size <= 4 {
        8
    } else if elem_size <= 8 {
        4
    } else if elem_size <= 16 {
        2
    } else {
        1
    }
}

impl<T> Deref for vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.data_nonnull(), self.size()) }
    }
}

impl<T> DerefMut for vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.data_nonnull(), self.size()) }
    }
}
