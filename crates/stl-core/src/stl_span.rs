use crate::types as c;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct span<T> {
    data: *const T,
    size: c::size_t,
}

impl<T> span<T> {
    pub const fn new(data: *const T, size: c::size_t) -> Self {
        Self { data, size }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        Self::new(slice.as_ptr(), slice.len())
    }

    pub const fn data(&self) -> *const T {
        self.data
    }

    pub const fn len(&self) -> c::size_t {
        self.size
    }

    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn as_slice(&self) -> &[T] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.data, self.size) }
        }
    }
}

impl<T> From<&[T]> for span<T> {
    fn from(value: &[T]) -> Self {
        Self::from_slice(value)
    }
}
