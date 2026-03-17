#[cfg(windows)]
pub use stl_core::msvc::{Variant2, optional, shared_ptr, span, string, string_view, vector};

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "android"))]
pub use stl_core::libcxx::{Variant2, optional, shared_ptr, span, string, string_view, vector};

pub use stl_core::containers::{map, set, unordered_map, unordered_set};

pub type StlString = string;
pub type StlStringView = string_view;
pub type StlOptional<T> = optional<T>;
pub type StlSpan<T> = span<T>;
pub type StlVector<T> = vector<T>;
pub type StlSharedPtr<T> = shared_ptr<T>;
pub type StlSet<T> = set<T>;
pub type StlMap<K, V> = map<K, V>;
pub type StlUnorderedMap<K, V> = unordered_map<K, V>;
pub type StlUnorderedSet<T> = unordered_set<T>;

#[cfg(not(any(windows, target_os = "macos", target_os = "ios", target_os = "android")))]
mod fallback {
    #[repr(C)]
    pub struct string {
        _data: [u8; 24],
    }
    impl Default for string {
        fn default() -> Self {
            Self { _data: [0u8; 24] }
        }
    }
    impl From<&str> for string {
        fn from(_: &str) -> Self {
            Self::default()
        }
    }

    #[repr(C)]
    pub struct vector<T> {
        _begin: *mut T,
        _end: *mut T,
        _cap: *mut T,
    }
    impl<T> vector<T> {
        pub fn new() -> Self {
            Self {
                _begin: std::ptr::null_mut(),
                _end: std::ptr::null_mut(),
                _cap: std::ptr::null_mut(),
            }
        }
    }
    impl<T> Default for vector<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    #[repr(C)]
    pub struct shared_ptr<T> {
        _ptr: *mut T,
        _ctrl: *mut std::ffi::c_void,
    }
    impl<T> shared_ptr<T> {
        pub fn is_null(&self) -> bool {
            self._ptr.is_null()
        }
        pub fn as_ptr(&self) -> *mut T {
            self._ptr
        }
    }
    impl<T> Default for shared_ptr<T> {
        fn default() -> Self {
            Self {
                _ptr: std::ptr::null_mut(),
                _ctrl: std::ptr::null_mut(),
            }
        }
    }

    pub type optional<T> = Option<T>;
    #[repr(C)]
    pub struct span<T> {
        _data: *const T,
        _size: usize,
    }
    impl<T> span<T> {
        pub fn from_slice(value: &[T]) -> Self {
            Self {
                _data: value.as_ptr(),
                _size: value.len(),
            }
        }
    }
    pub type string_view = ();
    pub type Variant2<A, B> = (A, B);
}

#[cfg(not(any(windows, target_os = "macos", target_os = "ios", target_os = "android")))]
pub use fallback::{Variant2, optional, shared_ptr, span, string, string_view, vector};

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct ZStringView {
    data: *const i8,
    length: usize,
}

impl ZStringView {
    pub const fn new(data: *const i8, length: usize) -> Self {
        Self { data, length }
    }

    pub fn as_str(&self) -> &str {
        if self.data.is_null() {
            return "";
        }

        let len = if self.length == usize::MAX {
            unsafe { std::ffi::CStr::from_ptr(self.data.cast()).to_bytes().len() }
        } else {
            self.length
        };

        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.data.cast(), len)) }
    }

    pub fn to_string_lossy(&self) -> String {
        self.as_str().to_owned()
    }
}
