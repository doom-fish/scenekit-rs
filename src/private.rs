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
