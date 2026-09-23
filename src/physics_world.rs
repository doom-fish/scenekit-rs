use core::ffi::c_void;
use core::ptr;

use crate::ffi;
use crate::math::Vector3;
use crate::node::Node;
use crate::physics::PhysicsBody;
use crate::private::{
    handle_type, invoke_callback, lookup_string_constant, CallbackState, DelegateObject,
};
use crate::scene::Scene;

handle_type!(PhysicsContact);
handle_type!(PhysicsWorld);

macro_rules! string_constant_fn {
    ($name:ident, $symbol:literal) => {
        #[doc = concat!("Returns the SceneKit constant `", $symbol, "`.")]
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

type PhysicsContactCallback = Box<dyn FnMut(Option<&PhysicsContact>) + Send>;

/// Stores Rust callbacks backing `SCNPhysicsContactDelegate`.
#[derive(Default)]
pub struct PhysicsContactDelegateCallbacks {
    begin: Option<PhysicsContactCallback>,
    update: Option<PhysicsContactCallback>,
    end: Option<PhysicsContactCallback>,
}

impl PhysicsContactDelegateCallbacks {
    /// Creates a wrapped `SCNPhysicsContactDelegate` instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            begin: None,
            update: None,
            end: None,
        }
    }

    /// Mirrors `SCNPhysicsContactDelegate.onDidBeginContact`.
    #[must_use]
    pub fn on_did_begin_contact<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<&PhysicsContact>) + Send + 'static,
    {
        self.begin = Some(Box::new(callback));
        self
    }

    /// Mirrors `SCNPhysicsContactDelegate.onDidUpdateContact`.
    #[must_use]
    pub fn on_did_update_contact<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<&PhysicsContact>) + Send + 'static,
    {
        self.update = Some(Box::new(callback));
        self
    }

    /// Mirrors `SCNPhysicsContactDelegate.onDidEndContact`.
    #[must_use]
    pub fn on_did_end_contact<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<&PhysicsContact>) + Send + 'static,
    {
        self.end = Some(Box::new(callback));
        self
    }
}

/// Wraps `SCNPhysicsContactDelegate`.
#[derive(Debug)]
pub struct PhysicsContactDelegate {
    inner: DelegateObject<PhysicsContactDelegateCallbacks>,
}

unsafe fn with_contact_callback(
    context: *mut c_void,
    site: &str,
    contact: *mut c_void,
    select: impl FnOnce(&mut PhysicsContactDelegateCallbacks) -> &mut Option<PhysicsContactCallback>,
) {
    let contact =
        (!contact.is_null()).then(|| unsafe { PhysicsContact::from_raw_borrowed(contact) });
    unsafe {
        invoke_callback::<PhysicsContactDelegateCallbacks, _>(context, site, |callbacks| {
            if let Some(callback) = select(callbacks).as_mut() {
                callback(contact.as_ref());
            }
        });
    }
}

unsafe extern "C" fn physics_did_begin_contact_trampoline(
    context: *mut c_void,
    contact: *mut c_void,
) {
    unsafe {
        with_contact_callback(
            context,
            "scenekit::PhysicsContactDelegate::did_begin_contact",
            contact,
            |callbacks| &mut callbacks.begin,
        );
    }
}

unsafe extern "C" fn physics_did_update_contact_trampoline(
    context: *mut c_void,
    contact: *mut c_void,
) {
    unsafe {
        with_contact_callback(
            context,
            "scenekit::PhysicsContactDelegate::did_update_contact",
            contact,
            |callbacks| &mut callbacks.update,
        );
    }
}

unsafe extern "C" fn physics_did_end_contact_trampoline(
    context: *mut c_void,
    contact: *mut c_void,
) {
    unsafe {
        with_contact_callback(
            context,
            "scenekit::PhysicsContactDelegate::did_end_contact",
            contact,
            |callbacks| &mut callbacks.end,
        );
    }
}

impl PhysicsContactDelegate {
    /// Creates a wrapped `SCNPhysicsContactDelegate` instance.
    #[must_use]
    pub fn new(callbacks: PhysicsContactDelegateCallbacks) -> Option<Self> {
        DelegateObject::new(callbacks, |context| unsafe {
            ffi::scn_physics_contact_delegate_new(
                context,
                CallbackState::<PhysicsContactDelegateCallbacks>::RELEASE,
                physics_did_begin_contact_trampoline,
                physics_did_update_contact_trampoline,
                physics_did_end_contact_trampoline,
            )
        })
        .map(|inner| Self { inner })
    }

    /// Returns the Objective-C pointer backing this `SCNPhysicsContactDelegate` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.inner.as_ptr()
    }
}

impl Scene {
    /// Mirrors `SCNScene.physicsWorld`.
    #[must_use]
    pub fn physics_world(&self) -> PhysicsWorld {
        unsafe { PhysicsWorld::from_raw_unchecked(ffi::scn_scene_physics_world(self.ptr)) }
    }
}

impl PhysicsContact {
    /// Mirrors `SCNPhysicsContact.nodeA`.
    #[must_use]
    pub fn node_a(&self) -> Option<Node> {
        unsafe { Node::from_raw(ffi::scn_physics_contact_get_node_a(self.ptr)) }
    }

    /// Mirrors `SCNPhysicsContact.nodeB`.
    #[must_use]
    pub fn node_b(&self) -> Option<Node> {
        unsafe { Node::from_raw(ffi::scn_physics_contact_get_node_b(self.ptr)) }
    }

    /// Mirrors `SCNPhysicsContact.contactPoint`.
    #[must_use]
    pub fn contact_point(&self) -> Vector3 {
        let mut value = Vector3::default();
        let _ = unsafe {
            ffi::scn_physics_contact_get_contact_point(self.ptr, value.as_mut_ptr().cast())
        };
        value
    }

    /// Mirrors `SCNPhysicsContact.contactNormal`.
    #[must_use]
    pub fn contact_normal(&self) -> Vector3 {
        let mut value = Vector3::default();
        let _ = unsafe {
            ffi::scn_physics_contact_get_contact_normal(self.ptr, value.as_mut_ptr().cast())
        };
        value
    }

    /// Mirrors `SCNPhysicsContact.collisionImpulse`.
    #[must_use]
    pub fn collision_impulse(&self) -> f64 {
        unsafe { ffi::scn_physics_contact_get_collision_impulse(self.ptr) }
    }

    /// Mirrors `SCNPhysicsContact.penetrationDistance`.
    #[must_use]
    pub fn penetration_distance(&self) -> f64 {
        unsafe { ffi::scn_physics_contact_get_penetration_distance(self.ptr) }
    }

    /// Mirrors `SCNPhysicsContact.sweepTestFraction`.
    #[must_use]
    pub fn sweep_test_fraction(&self) -> f64 {
        unsafe { ffi::scn_physics_contact_get_sweep_test_fraction(self.ptr) }
    }
}

impl PhysicsWorld {
    /// Mirrors `SCNPhysicsWorld.gravity`.
    #[must_use]
    pub fn gravity(&self) -> Vector3 {
        let mut gravity = Vector3::default();
        let _ =
            unsafe { ffi::scn_physics_world_get_gravity(self.ptr, gravity.as_mut_ptr().cast()) };
        gravity
    }

    /// Sets the `SCNPhysicsWorld.gravity` member.
    pub fn set_gravity(&self, gravity: Vector3) {
        unsafe { ffi::scn_physics_world_set_gravity(self.ptr, gravity.as_ptr().cast_mut().cast()) };
    }

    /// Mirrors `SCNPhysicsWorld.speed`.
    #[must_use]
    pub fn speed(&self) -> f64 {
        unsafe { ffi::scn_physics_world_get_speed(self.ptr) }
    }

    /// Sets the `SCNPhysicsWorld.speed` member.
    pub fn set_speed(&self, speed: f64) {
        unsafe { ffi::scn_physics_world_set_speed(self.ptr, speed) };
    }

    /// Mirrors `SCNPhysicsWorld.timeStep`.
    #[must_use]
    pub fn time_step(&self) -> f64 {
        unsafe { ffi::scn_physics_world_get_time_step(self.ptr) }
    }

    /// Sets the `SCNPhysicsWorld.timeStep` member.
    pub fn set_time_step(&self, time_step: f64) {
        unsafe { ffi::scn_physics_world_set_time_step(self.ptr, time_step) };
    }

    /// Sets the `SCNPhysicsWorld.contactDelegate` member.
    pub fn set_contact_delegate(&self, delegate: Option<&PhysicsContactDelegate>) {
        unsafe {
            ffi::scn_physics_world_set_contact_delegate(
                self.ptr,
                delegate.map_or(ptr::null_mut(), PhysicsContactDelegate::as_ptr),
            );
        };
    }

    /// Mirrors `SCNPhysicsWorld.updateCollisionPairs`.
    pub fn update_collision_pairs(&self) {
        unsafe { ffi::scn_physics_world_update_collision_pairs(self.ptr) };
    }

    /// Mirrors `SCNPhysicsWorld.contactTestWithBody`.
    #[must_use]
    pub fn contact_test_with_body(&self, physics_body: &PhysicsBody) -> usize {
        unsafe {
            ffi::scn_physics_world_contact_test_with_body_count(self.ptr, physics_body.as_ptr())
        }
    }

    /// Mirrors `SCNPhysicsWorld.contactTestBetweenBodies`.
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
