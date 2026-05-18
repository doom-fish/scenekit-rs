use crate::ffi;
use crate::math::Vector3;
use crate::private::handle_type;

handle_type!(PhysicsBody);

/// Mirrors `SCNPhysicsBodyType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PhysicsBodyType {
    /// Corresponds to the `SCNPhysicsBodyType::Static` case.
    Static = 0,
    /// Corresponds to the `SCNPhysicsBodyType::Dynamic` case.
    Dynamic = 1,
    /// Corresponds to the `SCNPhysicsBodyType::Kinematic` case.
    Kinematic = 2,
}

impl PhysicsBodyType {
    const fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Static),
            1 => Some(Self::Dynamic),
            2 => Some(Self::Kinematic),
            _ => None,
        }
    }
}

impl PhysicsBody {
    /// Mirrors `SCNPhysicsBody.staticBody`.
    #[must_use]
    pub fn static_body() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_physics_body_static()) }
    }

    /// Mirrors `SCNPhysicsBody.dynamicBody`.
    #[must_use]
    pub fn dynamic_body() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_physics_body_dynamic()) }
    }

    /// Mirrors `SCNPhysicsBody.kinematicBody`.
    #[must_use]
    pub fn kinematic_body() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_physics_body_kinematic()) }
    }

    /// Mirrors `SCNPhysicsBody.bodyType`.
    #[must_use]
    pub fn body_type(&self) -> Option<PhysicsBodyType> {
        PhysicsBodyType::from_raw(unsafe { ffi::scn_physics_body_get_type(self.ptr) })
    }

    /// Sets the `SCNPhysicsBody.bodyType` member.
    pub fn set_body_type(&self, body_type: PhysicsBodyType) {
        unsafe { ffi::scn_physics_body_set_type(self.ptr, body_type as i32) };
    }

    /// Mirrors `SCNPhysicsBody.mass`.
    #[must_use]
    pub fn mass(&self) -> f64 {
        unsafe { ffi::scn_physics_body_get_mass(self.ptr) }
    }

    /// Sets the `SCNPhysicsBody.mass` member.
    pub fn set_mass(&self, mass: f64) {
        unsafe { ffi::scn_physics_body_set_mass(self.ptr, mass) };
    }

    /// Mirrors `SCNPhysicsBody.restitution`.
    #[must_use]
    pub fn restitution(&self) -> f64 {
        unsafe { ffi::scn_physics_body_get_restitution(self.ptr) }
    }

    /// Sets the `SCNPhysicsBody.restitution` member.
    pub fn set_restitution(&self, restitution: f64) {
        unsafe { ffi::scn_physics_body_set_restitution(self.ptr, restitution) };
    }

    /// Mirrors `SCNPhysicsBody.friction`.
    #[must_use]
    pub fn friction(&self) -> f64 {
        unsafe { ffi::scn_physics_body_get_friction(self.ptr) }
    }

    /// Sets the `SCNPhysicsBody.friction` member.
    pub fn set_friction(&self, friction: f64) {
        unsafe { ffi::scn_physics_body_set_friction(self.ptr, friction) };
    }

    /// Mirrors `SCNPhysicsBody.applyForce`.
    pub fn apply_force(&self, force: Vector3, impulse: bool) {
        unsafe { ffi::scn_physics_body_apply_force(self.ptr, force.x, force.y, force.z, impulse) };
    }
}
