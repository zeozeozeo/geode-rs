// FIXME: should be windows specific.. later implement for other platforms

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StlString {
    _bx: StlStringContainer,
    _mysize: usize,
    _myres: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
union StlStringContainer {
    _buf: [u8; 16],
    _ptr: *mut u8,
}

impl StlString {
    pub fn new_from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        let len = bytes.len();

        if len < 16 {
            let mut buf = [0u8; 16];
            buf[..len].copy_from_slice(bytes);
            Self {
                _bx: StlStringContainer { _buf: buf },
                _mysize: len,
                _myres: 15,
            }
        } else {
            let mut v = bytes.to_vec();
            v.push(0);
            let ptr = v.as_mut_ptr();
            let capacity = v.capacity() - 1;
            std::mem::forget(v);
            Self {
                _bx: StlStringContainer { _ptr: ptr },
                _mysize: len,
                _myres: capacity,
            }
        }
    }

    pub const fn empty() -> Self {
        Self {
            _bx: StlStringContainer { _buf: [0u8; 16] },
            _mysize: 0,
            _myres: 15,
        }
    }
}

impl Default for StlString {
    fn default() -> Self {
        Self::empty()
    }
}

#[repr(C)]
pub struct StlVector<T> {
    _myfirst: *const T,
    _mylast: *const T,
    _myend: *const T,
}

impl<T> StlVector<T> {
    pub fn from_vec(v: Vec<T>) -> Self {
        if v.is_empty() {
            Self {
                _myfirst: std::ptr::null(),
                _mylast: std::ptr::null(),
                _myend: std::ptr::null(),
            }
        } else {
            let ptr = v.as_ptr();
            let len = v.len();
            let cap = v.capacity();
            std::mem::forget(v);
            Self {
                _myfirst: ptr,
                _mylast: unsafe { ptr.add(len) },
                _myend: unsafe { ptr.add(cap) },
            }
        }
    }

    pub const fn empty() -> Self {
        Self {
            _myfirst: std::ptr::null(),
            _mylast: std::ptr::null(),
            _myend: std::ptr::null(),
        }
    }
}

impl<T> Default for StlVector<T> {
    fn default() -> Self {
        Self::empty()
    }
}

#[repr(C)]
pub struct StlSharedPtr<T> {
    ptr: *const T,
    rep: *const std::ffi::c_void,
}

impl<T> StlSharedPtr<T> {
    pub const fn empty() -> Self {
        Self {
            ptr: std::ptr::null(),
            rep: std::ptr::null(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }
}

impl<T> Default for StlSharedPtr<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Clone for StlSharedPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for StlSharedPtr<T> {}
