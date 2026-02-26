use std::{fmt::Display, mem::MaybeUninit};

#[repr(C)]
pub struct optional<T> {
    data: MaybeUninit<T>,
    has_value: bool,
}

impl<T> optional<T> {
    pub fn new() -> Self {
        Self {
            data: MaybeUninit::uninit(),
            has_value: false,
        }
    }

    pub fn new_with_value(value: T) -> Self {
        Self {
            data: MaybeUninit::new(value),
            has_value: true,
        }
    }

    pub fn has_value(&self) -> bool {
        self.has_value
    }

    pub fn value(&self) -> Option<&T> {
        if self.has_value {
            Some(unsafe { self.data.assume_init_ref() })
        } else {
            None
        }
    }

    pub fn value_mut(&mut self) -> Option<&mut T> {
        if self.has_value {
            Some(unsafe { self.data.assume_init_mut() })
        } else {
            None
        }
    }

    pub fn take(&mut self) -> Option<T> {
        if self.has_value {
            self.has_value = false;
            Some(unsafe { self.data.assume_init_read() })
        } else {
            None
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.has_value {
            unsafe {
                std::ptr::drop_in_place(self.data.as_mut_ptr());
            }
        }
        self.data = MaybeUninit::new(value);
        self.has_value = true;
    }

    pub fn clear(&mut self) {
        if self.has_value {
            unsafe {
                std::ptr::drop_in_place(self.data.as_mut_ptr());
            }
            self.has_value = false;
        }
    }

    pub fn unwrap(mut self) -> T {
        self.take()
            .expect("called `optional::unwrap()` on a `None` value")
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn unwrap_unchecked(mut self) -> T {
        self.has_value = false;
        unsafe { self.data.assume_init_read() }
    }

    pub fn unwrap_or(mut self, default: T) -> T {
        self.take().unwrap_or(default)
    }

    pub fn unwrap_or_else<F: FnOnce() -> T>(mut self, f: F) -> T {
        self.take().unwrap_or_else(f)
    }
}

impl<T> Default for optional<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for optional<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T: Clone> Clone for optional<T> {
    fn clone(&self) -> Self {
        if self.has_value {
            Self::new_with_value(unsafe { self.data.assume_init_ref().clone() })
        } else {
            Self::new()
        }
    }
}

impl<T> Display for optional<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_value {
            write!(f, "Some({})", unsafe { self.data.assume_init_ref() })
        } else {
            write!(f, "None")
        }
    }
}

impl<T> From<Option<T>> for optional<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => Self::new_with_value(v),
            None => Self::new(),
        }
    }
}

impl<T> From<optional<T>> for Option<T> {
    fn from(mut value: optional<T>) -> Self {
        value.take()
    }
}
