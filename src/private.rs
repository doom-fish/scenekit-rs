use core::ffi::c_void;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::sync::{Mutex, PoisonError};
use std::thread::{self, ThreadId};

use doom_fish_utils::callback_context::CallbackContext;

/// Marker trait sealing internal SceneKit extension implementations.
pub trait Sealed {}

pub struct CallbackCell<T> {
    value: Mutex<T>,
    running_on: Mutex<Option<ThreadId>>,
}

struct RunningReset<'a>(&'a Mutex<Option<ThreadId>>);

impl Drop for RunningReset<'_> {
    fn drop(&mut self) {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = None;
    }
}

impl<T> CallbackCell<T> {
    pub(crate) const fn new(value: T) -> Self {
        Self {
            value: Mutex::new(value),
            running_on: Mutex::new(None),
        }
    }

    fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        let current = thread::current().id();
        if *self
            .running_on
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            == Some(current)
        {
            return None;
        }
        let mut value = self.value.lock().ok()?;
        *self
            .running_on
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = Some(current);
        let _reset = RunningReset(&self.running_on);
        Some(f(&mut value))
    }
}

pub type CallbackState<T> = CallbackContext<CallbackCell<T>>;

pub unsafe fn invoke_callback<T: Send + 'static, R>(
    context: *mut c_void,
    site: &str,
    f: impl FnOnce(&mut T) -> R,
) -> Option<R> {
    if context.is_null() {
        return None;
    }
    unsafe { CallbackState::<T>::RETAIN(context) };
    let result = unsafe { CallbackState::<T>::with(context, site, |cell| cell.with(f)) };
    unsafe { CallbackState::<T>::RELEASE(context) };
    result.flatten()
}

pub struct DelegateObject<T: Send + 'static> {
    ptr: *mut c_void,
    context: Option<CallbackState<T>>,
}

impl<T: Send + 'static> DelegateObject<T> {
    pub(crate) fn new(value: T, create: impl FnOnce(*mut c_void) -> *mut c_void) -> Option<Self> {
        let context = CallbackState::new(CallbackCell::new(value));
        let retained = context.retained_ptr();
        let ptr = create(retained);
        if ptr.is_null() {
            unsafe { CallbackState::<T>::RELEASE(retained) };
            return None;
        }
        Some(Self {
            ptr,
            context: Some(context),
        })
    }

    pub(crate) fn from_retained(ptr: *mut c_void) -> Option<Self> {
        (!ptr.is_null()).then_some(Self { ptr, context: None })
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub(crate) fn is_active(&self) -> bool {
        self.context
            .as_ref()
            .is_some_and(CallbackContext::is_active)
    }
}

impl<T: Send + 'static> Drop for DelegateObject<T> {
    fn drop(&mut self) {
        if let Some(context) = &self.context {
            context.deactivate();
        }
        if !self.ptr.is_null() {
            unsafe { crate::ffi::scn_release(self.ptr) };
            self.ptr = core::ptr::null_mut();
        }
    }
}

impl<T: Send + 'static> core::fmt::Debug for DelegateObject<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DelegateObject")
            .field("ptr", &self.ptr)
            .field("owns_callbacks", &self.context.is_some())
            .field("active", &self.is_active())
            .finish()
    }
}

pub fn is_main_thread() -> bool {
    unsafe { libc::pthread_main_np() != 0 }
}

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
