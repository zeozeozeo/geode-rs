use crate::types as c;
use std::fmt::Display;

const SHORT_MASK: u8 = 0x01;
const SHORT_MAX_SIZE: usize = 22;

#[repr(C)]
union StringUnion {
    short: ShortString,
    long: LongString,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct ShortString {
    size_and_flag: u8,
    buffer: [u8; 23],
}

#[derive(Clone, Copy)]
#[repr(C)]
struct LongString {
    cap_flagged: usize,
    size: usize,
    ptr: *mut u8,
}

#[repr(C)]
pub struct string {
    data: StringUnion,
}

impl string {
    pub fn new() -> Self {
        Self {
            data: StringUnion {
                short: ShortString {
                    size_and_flag: 0,
                    buffer: [0u8; 23],
                },
            },
        }
    }

    fn is_long(&self) -> bool {
        unsafe { self.data.short.size_and_flag & SHORT_MASK != 0 }
    }

    pub fn data(&self) -> *const c::char {
        if self.is_long() {
            unsafe { self.data.long.ptr as *const c::char }
        } else {
            unsafe { self.data.short.buffer.as_ptr() as *const c::char }
        }
    }

    pub fn data_mut(&mut self) -> *mut c::char {
        if self.is_long() {
            unsafe { self.data.long.ptr as *mut c::char }
        } else {
            unsafe { self.data.short.buffer.as_mut_ptr() as *mut c::char }
        }
    }

    pub fn size(&self) -> usize {
        if self.is_long() {
            unsafe { self.data.long.size }
        } else {
            unsafe { (self.data.short.size_and_flag >> 1) as usize }
        }
    }

    pub fn new_from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        let len = bytes.len();

        if len <= SHORT_MAX_SIZE {
            let mut result = Self::new();
            unsafe {
                result.data.short.buffer[..len].copy_from_slice(bytes);
                result.data.short.buffer[len] = 0;
                result.data.short.size_and_flag = (len as u8) << 1;
            }
            result
        } else {
            unsafe {
                let ptr = libc::malloc(len + 1) as *mut u8;
                assert!(!ptr.is_null(), "Buy more RAM");
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);
                *ptr.add(len) = 0;
                Self {
                    data: StringUnion {
                        long: LongString {
                            cap_flagged: len | 1,
                            size: len,
                            ptr,
                        },
                    },
                }
            }
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
        if self.is_long() {
            unsafe {
                libc::free(self.data.long.ptr as *mut libc::c_void);
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
        let slice = unsafe { std::slice::from_raw_parts(self.data() as *const u8, self.size()) };
        let s = std::str::from_utf8(slice).map_err(|_| std::fmt::Error)?;
        write!(f, "{s}")
    }
}
