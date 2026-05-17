use core::ffi::c_void;
use core::ptr;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::ffi;
use crate::math::Vector3;
use crate::node::Node;
use crate::physics::PhysicsBody;
use crate::private::{handle_type, lookup_string_constant};
use crate::scene::Scene;

handle_type!(PhysicsContact);
handle_type!(PhysicsWorld);

macro_rules! string_constant_fn {
    ($name:ident, $symbol:literal) => {
        #[must_use]
        pub fn $name() -> String {
            lookup_string_constant($symbol)
        }
    };
}

string_constant_fn!(
    physics_test_backface_culling_key,
    "SCNPhysicsTestBackfaceCullingKey"
);
string_constant_fn!(
    physics_test_collision_bit_mask_key,
    "SCNPhysicsTestCollisionBitMaskKey"
);
string_constant_fn!(physics_test_search_mode_all, "SCNPhysicsTestSearchModeAll");
string_constant_fn!(physics_test_search_mode_any, "SCNPhysicsTestSearchModeAny");
string_constant_fn!(
    physics_test_search_mode_closest,
    "SCNPhysicsTestSearchModeClosest"
);
string_constant_fn!(physics_test_search_mode_key, "SCNPhysicsTestSearchModeKey");

type PhysicsContactCallback = Box<dyn FnMut(Option<&PhysicsContact>)>;

#[derive(Default)]
pub struct PhysicsContactDelegateCallbacks {
    begin: Option<PhysicsContactCallback>,
    update: Option<PhysicsContactCallback>,
    end: Option<PhysicsContactCallback>,
}

impl PhysicsContactDelegateCallbacks {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            begin: None,
            update: None,
            end: None,
        }
    }

    #[must_use]
    pub fn on_did_begin_contact<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<&PhysicsContact>) + 'static,
    {
        self.begin = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_did_update_contact<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<&PhysicsContact>) + 'static,
    {
        self.update = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_did_end_contact<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<&PhysicsContact>) + 'static,
    {
        self.end = Some(Box::new(callback));
        self
    }
}

struct PhysicsContactDelegateState {
    callbacks: PhysicsContactDelegateCallbacks,
}

pub struct PhysicsContactDelegate {
    ptr: *mut c_void,
}

impl core::fmt::Debug for PhysicsContactDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PhysicsContactDelegate")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl Drop for PhysicsContactDelegate {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::scn_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

unsafe fn physics_delegate_state_from_context<'a>(
    context: *mut c_void,
) -> &'a mut PhysicsContactDelegateState {
    &mut *context.cast::<PhysicsContactDelegateState>()
}

extern "C" fn release_physics_delegate_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(context.cast::<PhysicsContactDelegateState>()));
    }
}

unsafe fn with_contact_callback(
    context: *mut c_void,
    contact: *mut c_void,
    select: impl FnOnce(&mut PhysicsContactDelegateCallbacks) -> &mut Option<PhysicsContactCallback>,
) {
    if context.is_null() {
        return;
    }
    let state = unsafe { physics_delegate_state_from_context(context) };
    let callback_slot = select(&mut state.callbacks);
    if let Some(callback) = callback_slot.as_mut() {
        if contact.is_null() {
            callback(None);
        } else {
            let contact = unsafe { PhysicsContact::from_raw_borrowed(contact) };
            callback(Some(&contact));
        }
    }
}

extern "C" fn physics_did_begin_contact_trampoline(context: *mut c_void, contact: *mut c_void) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        unsafe { with_contact_callback(context, contact, |callbacks| &mut callbacks.begin) };
    }));
}

extern "C" fn physics_did_update_contact_trampoline(context: *mut c_void, contact: *mut c_void) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        unsafe { with_contact_callback(context, contact, |callbacks| &mut callbacks.update) };
    }));
}

extern "C" fn physics_did_end_contact_trampoline(context: *mut c_void, contact: *mut c_void) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        unsafe { with_contact_callback(context, contact, |callbacks| &mut callbacks.end) };
    }));
}

impl PhysicsContactDelegate {
    #[must_use]
    pub fn new(callbacks: PhysicsContactDelegateCallbacks) -> Option<Self> {
        let state = Box::new(PhysicsContactDelegateState { callbacks });
        let context = Box::into_raw(state).cast::<c_void>();
        let ptr = unsafe {
            ffi::scn_physics_contact_delegate_new(
                context,
                release_physics_delegate_context,
                physics_did_begin_contact_trampoline,
                physics_did_update_contact_trampoline,
                physics_did_end_contact_trampoline,
            )
        };
        if ptr.is_null() {
            release_physics_delegate_context(context);
            None
        } else {
            Some(Self { ptr })
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Scene {
    #[must_use]
    pub fn physics_world(&self) -> PhysicsWorld {
        unsafe { PhysicsWorld::from_raw_unchecked(ffi::scn_scene_physics_world(self.ptr)) }
    }
}

impl PhysicsContact {
    #[must_use]
    pub fn node_a(&self) -> Option<Node> {
        unsafe { Node::from_raw(ffi::scn_physics_contact_get_node_a(self.ptr)) }
    }

    #[must_use]
    pub fn node_b(&self) -> Option<Node> {
        unsafe { Node::from_raw(ffi::scn_physics_contact_get_node_b(self.ptr)) }
    }

    #[must_use]
    pub fn contact_point(&self) -> Vector3 {
        let mut value = Vector3::default();
        let _ = unsafe {
            ffi::scn_physics_contact_get_contact_point(self.ptr, value.as_mut_ptr().cast())
        };
        value
    }

    #[must_use]
    pub fn contact_normal(&self) -> Vector3 {
        let mut value = Vector3::default();
        let _ = unsafe {
            ffi::scn_physics_contact_get_contact_normal(self.ptr, value.as_mut_ptr().cast())
        };
        value
    }

    #[must_use]
    pub fn collision_impulse(&self) -> f64 {
        unsafe { ffi::scn_physics_contact_get_collision_impulse(self.ptr) }
    }

    #[must_use]
    pub fn penetration_distance(&self) -> f64 {
        unsafe { ffi::scn_physics_contact_get_penetration_distance(self.ptr) }
    }

    #[must_use]
    pub fn sweep_test_fraction(&self) -> f64 {
        unsafe { ffi::scn_physics_contact_get_sweep_test_fraction(self.ptr) }
    }
}

impl PhysicsWorld {
    #[must_use]
    pub fn gravity(&self) -> Vector3 {
        let mut gravity = Vector3::default();
        let _ =
            unsafe { ffi::scn_physics_world_get_gravity(self.ptr, gravity.as_mut_ptr().cast()) };
        gravity
    }

    pub fn set_gravity(&self, gravity: Vector3) {
        unsafe { ffi::scn_physics_world_set_gravity(self.ptr, gravity.as_ptr().cast_mut().cast()) };
    }

    #[must_use]
    pub fn speed(&self) -> f64 {
        unsafe { ffi::scn_physics_world_get_speed(self.ptr) }
    }

    pub fn set_speed(&self, speed: f64) {
        unsafe { ffi::scn_physics_world_set_speed(self.ptr, speed) };
    }

    #[must_use]
    pub fn time_step(&self) -> f64 {
        unsafe { ffi::scn_physics_world_get_time_step(self.ptr) }
    }

    pub fn set_time_step(&self, time_step: f64) {
        unsafe { ffi::scn_physics_world_set_time_step(self.ptr, time_step) };
    }

    pub fn set_contact_delegate(&self, delegate: Option<&PhysicsContactDelegate>) {
        unsafe {
            ffi::scn_physics_world_set_contact_delegate(
                self.ptr,
                delegate.map_or(ptr::null_mut(), PhysicsContactDelegate::as_ptr),
            );
        };
    }

    pub fn update_collision_pairs(&self) {
        unsafe { ffi::scn_physics_world_update_collision_pairs(self.ptr) };
    }

    #[must_use]
    pub fn contact_test_with_body(&self, physics_body: &PhysicsBody) -> usize {
        unsafe {
            ffi::scn_physics_world_contact_test_with_body_count(self.ptr, physics_body.as_ptr())
        }
    }

    #[must_use]
    pub fn contact_test_between_bodies(&self, body_a: &PhysicsBody, body_b: &PhysicsBody) -> usize {
        unsafe {
            ffi::scn_physics_world_contact_test_between_bodies_count(
                self.ptr,
                body_a.as_ptr(),
                body_b.as_ptr(),
            )
        }
    }
}
