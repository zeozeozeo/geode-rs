use std::ffi::c_void;

#[repr(C)]
pub struct set<T> {
    _head: *mut c_void,
    _size: usize,
    _marker: std::marker::PhantomData<T>,
}

unsafe impl<T: Send> Send for set<T> {}
unsafe impl<T: Sync> Sync for set<T> {}

impl<T> Default for set<T> {
    fn default() -> Self {
        Self {
            _head: std::ptr::null_mut(),
            _size: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> set<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self._size
    }

    pub fn is_empty(&self) -> bool {
        self._size == 0
    }
}

#[repr(C)]
pub struct map<K, V> {
    _head: *mut c_void,
    _size: usize,
    _marker: std::marker::PhantomData<(K, V)>,
}

unsafe impl<K: Send, V: Send> Send for map<K, V> {}
unsafe impl<K: Sync, V: Sync> Sync for map<K, V> {}

impl<K, V> Default for map<K, V> {
    fn default() -> Self {
        Self {
            _head: std::ptr::null_mut(),
            _size: 0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<K, V> map<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self._size
    }

    pub fn is_empty(&self) -> bool {
        self._size == 0
    }
}

#[repr(C)]
pub struct unordered_map<K, V> {
    #[cfg(windows)]
    _data: [usize; 8],
    #[cfg(not(windows))]
    _data: [usize; 4],
    _marker: std::marker::PhantomData<(K, V)>,
}

unsafe impl<K: Send, V: Send> Send for unordered_map<K, V> {}
unsafe impl<K: Sync, V: Sync> Sync for unordered_map<K, V> {}

impl<K, V> Default for unordered_map<K, V> {
    fn default() -> Self {
        Self {
            _data: Default::default(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<K, V> unordered_map<K, V> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[repr(C)]
pub struct unordered_set<T> {
    #[cfg(windows)]
    _data: [usize; 8],
    #[cfg(not(windows))]
    _data: [usize; 4],
    _marker: std::marker::PhantomData<T>,
}

unsafe impl<T: Send> Send for unordered_set<T> {}
unsafe impl<T: Sync> Sync for unordered_set<T> {}

impl<T> Default for unordered_set<T> {
    fn default() -> Self {
        Self {
            _data: Default::default(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> unordered_set<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        0
    }

    pub fn is_empty(&self) -> bool {
        true
    }
}
