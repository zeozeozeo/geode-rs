#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct path {
    inner: crate::msvc::wstring,
}

impl path {
    pub fn new() -> Self {
        Self {
            inner: crate::msvc::wstring::new(),
        }
    }

    pub fn from_wide(data: &[u16]) -> Self {
        Self {
            inner: crate::msvc::wstring::new_from_slice(data),
        }
    }

    pub fn native(&self) -> &crate::msvc::wstring {
        &self.inner
    }

    pub fn as_slice(&self) -> &[u16] {
        self.inner.as_slice()
    }

    pub fn to_string_lossy(&self) -> String {
        self.inner.to_string_lossy()
    }
}

impl Default for path {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for path {
    fn clone(&self) -> Self {
        Self::from_wide(self.as_slice())
    }
}
