#[repr(C)]
#[derive(Default, Clone)]
pub struct pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> pair<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Self { first, second }
    }

    pub fn into_tuple(self) -> (T, U) {
        (self.first, self.second)
    }
}
