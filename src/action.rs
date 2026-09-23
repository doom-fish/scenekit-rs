use core::ffi::c_void;
use core::mem::ManuallyDrop;

use crate::ffi;
use crate::math::Vector3;
use crate::node::Node;
use crate::private::{handle_type, invoke_callback, CallbackCell, CallbackState};

handle_type!(Action);

type BoxedActionCallback = Box<dyn FnMut(&Node, f64) + Send>;

unsafe extern "C" fn action_invoke(context: *mut c_void, node: *mut c_void, elapsed: f64) {
    if node.is_null() {
        return;
    }
    let node = unsafe { Node::from_raw_borrowed(node) };
    unsafe {
        invoke_callback::<BoxedActionCallback, _>(
            context,
            "scenekit::Action::custom",
            |callback| {
                callback(&node, elapsed);
            },
        );
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
        F: FnMut(&Node, f64) + Send + 'static,
    {
        let callback: BoxedActionCallback = Box::new(callback);
        let context = ManuallyDrop::new(CallbackState::new(CallbackCell::new(callback)));
        unsafe {
            Self::from_raw(ffi::scn_action_custom(
                duration,
                context.as_ptr(),
                action_invoke,
                CallbackState::<BoxedActionCallback>::RELEASE,
            ))
        }
    }
}
