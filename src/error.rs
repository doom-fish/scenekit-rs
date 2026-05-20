use std::error::Error;
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
    doom_fish_utils::ffi_string::take_owned_cstring_c(ptr, |p| libc::free(p.cast()))
}

pub(crate) unsafe fn take_error(ptr: *mut libc::c_char, fallback: &str) -> SceneKitError {
    SceneKitError::new(take_string(ptr).unwrap_or_else(|| fallback.to_owned()))
}
