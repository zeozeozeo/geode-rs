use std::{fmt::Display, ops::Deref};
use crate::types as c;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct string_view {
    data: *const c::char,
    size: c::size_t,
    // _marker: PhantomData<&'a c::char>,
}

impl string_view {
    pub fn new(data: *const c::char, size: c::size_t) -> Self {
        Self {
            data,
            size,
            // _marker: PhantomData,
        }
    }

    pub fn new_from_str(s: &str) -> Self {
        let bytes = s.as_bytes();

        Self {
            data: bytes.as_ptr() as *const c::char,
            size: bytes.len() as c::size_t,
            // _marker: PhantomData,
        }
    }

    pub fn as_str(&self) -> &str {
        if self.data.is_null() {
            return "";
        }

        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                self.data as *const u8,
                self.size,
            ))
        }
    }

    pub fn len(&self) -> c::size_t {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl From<&str> for string_view {
    fn from(s: &str) -> Self {
        Self::new_from_str(s)
    }
}

impl Default for string_view {
    fn default() -> Self {
        Self {
            data: std::ptr::null(),
            size: 0,
            // _marker: PhantomData,
        }
    }
}

impl Display for string_view {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Deref for string_view {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

