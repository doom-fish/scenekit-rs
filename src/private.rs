use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

/// Marker trait sealing internal SceneKit extension implementations.
pub trait Sealed {}

macro_rules! handle_type {
    (@emit $name:ident, $type_doc:expr, $ptr_doc:expr) => {
        #[doc = $type_doc]
        pub struct $name {
            pub(crate) ptr: *mut core::ffi::c_void,
            owned: bool,
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ptr", &self.ptr)
                    .field("owned", &self.owned)
                    .finish()
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                if self.owned && !self.ptr.is_null() {
                    unsafe { crate::ffi::scn_release(self.ptr) };
                    self.ptr = core::ptr::null_mut();
                }
            }
        }

        #[allow(dead_code)]
        impl $name {
            pub(crate) unsafe fn from_raw(ptr: *mut core::ffi::c_void) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { ptr, owned: true })
                }
            }

            pub(crate) const unsafe fn from_raw_unchecked(ptr: *mut core::ffi::c_void) -> Self {
                Self { ptr, owned: true }
            }

            pub(crate) const unsafe fn from_raw_borrowed(ptr: *mut core::ffi::c_void) -> Self {
                Self { ptr, owned: false }
            }

            #[doc = $ptr_doc]
            #[must_use]
            pub const fn as_ptr(&self) -> *mut core::ffi::c_void {
                self.ptr
            }
        }
    };
    ($name:ident) => {
        handle_type!(
            @emit
            $name,
            concat!("Wraps `SCN", stringify!($name), "`."),
            concat!(
                "Returns the Objective-C pointer backing this `SCN",
                stringify!($name),
                "` wrapper."
            )
        );
    };
    ($name:ident, $counterpart:literal) => {
        handle_type!(
            @emit
            $name,
            concat!("Wraps `", $counterpart, "`."),
            concat!(
                "Returns the Objective-C pointer backing this `",
                $counterpart,
                "` wrapper."
            )
        );
    };
}

pub(crate) use handle_type;

/// Generate a `Drop` impl for a release-only pointer wrapper.
///
/// Many delegate wrappers hold a single `*mut c_void` pointer to a retained
/// Swift object and hand-roll an identical `Drop` that null-checks the pointer,
/// calls `scn_release`, and clears the field. `scn_retained!` consolidates that
/// boilerplate into a single audited place while preserving the exact behavior
/// of the previous hand-written versions:
/// - `Drop` null-checks `self.<field>` before calling the supplied `release`
///   FFI fn (matching the original `if !ptr.is_null()` guards), then resets the
///   field to a null pointer.
///
/// Wrappers whose `Drop` carries extra logic beyond release + null-check are
/// intentionally left hand-written.
macro_rules! scn_retained {
    // Named-field struct (`{ ptr, .. }`), Drop only.
    ($ty:ty, field = $field:ident, release = $release:path $(,)?) => {
        impl Drop for $ty {
            fn drop(&mut self) {
                if !self.$field.is_null() {
                    unsafe { $release(self.$field) };
                    self.$field = core::ptr::null_mut();
                }
            }
        }
    };
}

pub(crate) use scn_retained;

/// Builds a `CString` for SceneKit bridge calls.
pub fn cstring_from_str(value: &str) -> Option<CString> {
    CString::new(value).ok()
}

/// Builds a `CString` from a filesystem path for SceneKit bridge calls.
pub fn cstring_from_path(path: &Path) -> Option<CString> {
    CString::new(path.as_os_str().as_bytes()).ok()
}

/// Looks up a SceneKit string constant by symbol name.
pub fn lookup_string_constant(symbol: &str) -> String {
    let c_string = cstring_from_str(symbol)
        .expect("SceneKit constant symbol names never contain interior NUL bytes");
    unsafe { crate::error::take_string(crate::ffi::scn_constant_lookup(c_string.as_ptr())) }
        .unwrap_or_else(|| symbol.to_owned())
}
