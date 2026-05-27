#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct path {
    inner: crate::gnustl::string,
}

impl path {
    pub fn new() -> Self {
        Self {
            inner: crate::gnustl::string::new(),
        }
    }

    pub fn from_str(data: &str) -> Self {
        Self {
            inner: crate::gnustl::string::from(data),
        }
    }

    pub fn native(&self) -> &crate::gnustl::string {
        &self.inner
    }

    pub fn to_string_lossy(&self) -> String {
        self.inner.to_string()
    }
}

impl Default for path {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for path {
    fn clone(&self) -> Self {
        Self::from_str(&self.to_string_lossy())
    }
}
