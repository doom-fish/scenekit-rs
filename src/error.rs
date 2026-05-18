use std::error::Error;
use std::ffi::CStr;
use std::fmt;

/// Represents an error surfaced by a SceneKit wrapper call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SceneKitError {
    message: String,
}

impl SceneKitError {
    /// Creates a wrapped `SCNErrorDomain` instance.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for SceneKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for SceneKitError {}

pub(crate) unsafe fn take_string(ptr: *mut libc::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    libc::free(ptr.cast());
    Some(value)
}

pub(crate) unsafe fn take_error(ptr: *mut libc::c_char, fallback: &str) -> SceneKitError {
    SceneKitError::new(take_string(ptr).unwrap_or_else(|| fallback.to_owned()))
}
