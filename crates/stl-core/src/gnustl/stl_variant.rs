use std::mem::ManuallyDrop;

// TODO: i want more than 2 but i failed miserably to vibecode a macro

#[repr(C)]
union Variant2Union<T1, T2> {
    val1: ManuallyDrop<T1>,
    val2: ManuallyDrop<T2>,
}

#[repr(C)]
pub struct Variant2<T1, T2> {
    data: Variant2Union<T1, T2>,
    index: i8, // i8 for 0-127 types
}

impl<T1, T2> Variant2<T1, T2> {
    pub fn new_val1(value: T1) -> Self {
        Self {
            data: Variant2Union {
                val1: ManuallyDrop::new(value),
            },
            index: 0,
        }
    }

    pub fn new_val2(value: T2) -> Self {
        Self {
            data: Variant2Union {
                val2: ManuallyDrop::new(value),
            },
            index: 1,
        }
    }

    pub fn is_val1(&self) -> bool {
        self.index == 0
    }

    pub fn is_val2(&self) -> bool {
        self.index == 1
    }

    pub fn as_val1(&self) -> Option<&T1> {
        if self.is_val1() {
            unsafe { Some(&self.data.val1) }
        } else {
            None
        }
    }

    pub fn as_val2(&self) -> Option<&T2> {
        if self.is_val2() {
            unsafe { Some(&self.data.val2) }
        } else {
            None
        }
    }

    pub fn into_val1(mut self) -> Option<T1> {
        if self.is_val1() {
            self.index = -1;
            unsafe { Some(ManuallyDrop::take(&mut self.data.val1)) }
        } else {
            None
        }
    }

    pub fn into_val2(mut self) -> Option<T2> {
        if self.is_val2() {
            self.index = -1;
            unsafe { Some(ManuallyDrop::take(&mut self.data.val2)) }
        } else {
            None
        }
    }
}

impl<T1, T2> Drop for Variant2<T1, T2> {
    fn drop(&mut self) {
        unsafe {
            match self.index {
                0 => ManuallyDrop::drop(&mut self.data.val1),
                1 => ManuallyDrop::drop(&mut self.data.val2),

                _ => {}
            }
        }
    }
}

impl<T1: Default, T2> Default for Variant2<T1, T2> {
    fn default() -> Self {
        Self::new_val1(T1::default())
    }
}

impl<T1: Clone, T2: Clone> Clone for Variant2<T1, T2> {
    fn clone(&self) -> Self {
        unsafe {
            match self.index {
                0 => Self::new_val1(self.as_val1().unwrap_unchecked().clone()),
                1 => Self::new_val2(self.as_val2().unwrap_unchecked().clone()),
                _ => panic!("cloning invalid variant"),
            }
        }
    }
}
