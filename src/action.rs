use core::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Mutex;

use crate::ffi;
use crate::math::Vector3;
use crate::node::Node;
use crate::private::handle_type;

handle_type!(Action);

type BoxedActionCallback = Box<dyn FnMut(Node, f64) + Send>;

struct ActionCallbackState {
    callback: Mutex<BoxedActionCallback>,
}

extern "C" fn action_invoke(context: *mut c_void, node: *mut c_void, elapsed: f64) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let Some(context) = core::ptr::NonNull::new(context.cast::<ActionCallbackState>()) else {
            return;
        };
        let state = unsafe { context.as_ref() };
        if let Ok(mut callback) = state.callback.lock() {
            let node = unsafe { Node::from_raw_borrowed(node) };
            callback(node, elapsed);
        }
    }));
}

extern "C" fn action_drop(context: *mut c_void) {
    if let Some(context) = core::ptr::NonNull::new(context.cast::<ActionCallbackState>()) {
        unsafe { drop(Box::from_raw(context.as_ptr())) };
    }
}

impl Action {
    /// Mirrors `SCNAction.moveTo`.
    #[must_use]
    pub fn move_to(position: Vector3, duration: f64) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::scn_action_move_to(
                position.x, position.y, position.z, duration,
            ))
        }
    }

    /// Mirrors `SCNAction.moveBy`.
    #[must_use]
    pub fn move_by(delta: Vector3, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_action_move_by(delta.x, delta.y, delta.z, duration)) }
    }

    /// Mirrors `SCNAction.rotateBy`.
    #[must_use]
    pub fn rotate_by(delta: Vector3, duration: f64) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::scn_action_rotate_by(
                delta.x, delta.y, delta.z, duration,
            ))
        }
    }

    /// Mirrors `SCNAction.scaleBy`.
    #[must_use]
    pub fn scale_by(scale: f32, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_action_scale_by(scale, duration)) }
    }

    /// Mirrors `SCNAction.sequence`.
    #[must_use]
    pub fn sequence(actions: &[&Self]) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = actions.iter().map(|action| action.as_ptr()).collect();
        unsafe { Self::from_raw(ffi::scn_action_sequence(raw.as_mut_ptr().cast(), raw.len())) }
    }

    /// Mirrors `SCNAction.group`.
    #[must_use]
    pub fn group(actions: &[&Self]) -> Option<Self> {
        let mut raw: Vec<*mut c_void> = actions.iter().map(|action| action.as_ptr()).collect();
        unsafe { Self::from_raw(ffi::scn_action_group(raw.as_mut_ptr().cast(), raw.len())) }
    }

    /// Mirrors `SCNAction.repeatCount`.
    #[must_use]
    pub fn repeat_count(action: &Self, count: usize) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_action_repeat(action.as_ptr(), count)) }
    }

    /// Mirrors `SCNAction.repeatForever`.
    #[must_use]
    pub fn repeat_forever(action: &Self) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_action_repeat_forever(action.as_ptr())) }
    }

    /// Mirrors `SCNAction.custom`.
    #[must_use]
    pub fn custom<F>(duration: f64, callback: F) -> Option<Self>
    where
        F: FnMut(Node, f64) + Send + 'static,
    {
        let state = Box::new(ActionCallbackState {
            callback: Mutex::new(Box::new(callback)),
        });
        unsafe {
            Self::from_raw(ffi::scn_action_custom(
                duration,
                Box::into_raw(state).cast(),
                action_invoke,
                action_drop,
            ))
        }
    }
}
