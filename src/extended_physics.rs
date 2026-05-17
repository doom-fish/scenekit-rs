use core::ffi::c_void;
use core::ops::Deref;
use core::ptr;

use crate::math::{Matrix4, Vector3};
use crate::node::Node;
use crate::physics::PhysicsBody;
use crate::physics_world::PhysicsWorld;

extern "C" {
    fn scn_physics_shape_new_with_geometry(geometry: *mut c_void) -> *mut c_void;
    fn scn_physics_shape_new_with_node(node: *mut c_void) -> *mut c_void;
    fn scn_physics_body_get_shape(body: *mut c_void) -> *mut c_void;
    fn scn_physics_body_set_shape(body: *mut c_void, shape: *mut c_void);

    fn scn_physics_field_drag() -> *mut c_void;
    fn scn_physics_field_vortex() -> *mut c_void;
    fn scn_physics_field_radial_gravity() -> *mut c_void;
    fn scn_physics_field_linear_gravity() -> *mut c_void;
    fn scn_physics_field_get_scope(field: *mut c_void) -> i32;
    fn scn_physics_field_set_scope(field: *mut c_void, scope: i32);

    fn scn_physics_world_add_behavior(world: *mut c_void, behavior: *mut c_void);
    fn scn_physics_world_remove_behavior(world: *mut c_void, behavior: *mut c_void);
    fn scn_physics_world_remove_all_behaviors(world: *mut c_void);

    fn scn_node_get_physics_field(node: *mut c_void) -> *mut c_void;
    fn scn_node_set_physics_field(node: *mut c_void, field: *mut c_void);

    fn scn_physics_ball_socket_joint_new(
        body_a: *mut c_void,
        anchor_a: *mut c_void,
        body_b: *mut c_void,
        anchor_b: *mut c_void,
    ) -> *mut c_void;
    fn scn_physics_ball_socket_joint_new_single(body: *mut c_void, anchor: *mut c_void) -> *mut c_void;

    fn scn_physics_hinge_joint_new(
        body_a: *mut c_void,
        axis_a: *mut c_void,
        anchor_a: *mut c_void,
        body_b: *mut c_void,
        axis_b: *mut c_void,
        anchor_b: *mut c_void,
    ) -> *mut c_void;
    fn scn_physics_hinge_joint_new_single(
        body: *mut c_void,
        axis: *mut c_void,
        anchor: *mut c_void,
    ) -> *mut c_void;

    fn scn_physics_slider_joint_new(
        body_a: *mut c_void,
        axis_a: *mut c_void,
        anchor_a: *mut c_void,
        body_b: *mut c_void,
        axis_b: *mut c_void,
        anchor_b: *mut c_void,
    ) -> *mut c_void;
    fn scn_physics_slider_joint_new_single(
        body: *mut c_void,
        axis: *mut c_void,
        anchor: *mut c_void,
    ) -> *mut c_void;

    fn scn_physics_cone_twist_joint_new(
        body_a: *mut c_void,
        frame_a: *mut c_void,
        body_b: *mut c_void,
        frame_b: *mut c_void,
    ) -> *mut c_void;
    fn scn_physics_cone_twist_joint_new_single(body: *mut c_void, frame: *mut c_void) -> *mut c_void;

    fn scn_physics_vehicle_wheel_new(node: *mut c_void) -> *mut c_void;
    fn scn_physics_vehicle_new(
        chassis_body: *mut c_void,
        wheels: *mut c_void,
        wheel_count: usize,
    ) -> *mut c_void;
    fn scn_physics_vehicle_get_speed_in_kilometers_per_hour(vehicle: *mut c_void) -> f64;
    fn scn_physics_vehicle_apply_engine_force(vehicle: *mut c_void, value: f64, wheel_index: usize);
    fn scn_physics_vehicle_set_steering_angle(vehicle: *mut c_void, value: f64, wheel_index: usize);
    fn scn_physics_vehicle_apply_braking_force(vehicle: *mut c_void, value: f64, wheel_index: usize);
}

use crate::geometry::Geometry;
use crate::private::handle_type;
use crate::symbols::PhysicsFieldScope;

handle_type!(PhysicsBehavior);
handle_type!(PhysicsShape);
handle_type!(PhysicsField);
handle_type!(PhysicsVehicleWheel);

macro_rules! physics_behavior_newtype {
    ($name:ident) => {
        pub struct $name(PhysicsBehavior);

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl Deref for $name {
            type Target = PhysicsBehavior;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$name> for PhysicsBehavior {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl $name {
            unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
                PhysicsBehavior::from_raw(ptr).map(Self)
            }
        }
    };
}

physics_behavior_newtype!(PhysicsBallSocketJoint);
physics_behavior_newtype!(PhysicsHingeJoint);
physics_behavior_newtype!(PhysicsSliderJoint);
physics_behavior_newtype!(PhysicsConeTwistJoint);
physics_behavior_newtype!(PhysicsVehicle);

impl PhysicsShape {
    #[must_use]
    pub fn with_geometry(geometry: &Geometry) -> Option<Self> {
        unsafe { Self::from_raw(scn_physics_shape_new_with_geometry(geometry.as_ptr())) }
    }

    #[must_use]
    pub fn with_node(node: &Node) -> Option<Self> {
        unsafe { Self::from_raw(scn_physics_shape_new_with_node(node.as_ptr())) }
    }
}

impl PhysicsBody {
    #[must_use]
    pub fn physics_shape(&self) -> Option<PhysicsShape> {
        unsafe { PhysicsShape::from_raw(scn_physics_body_get_shape(self.as_ptr())) }
    }

    pub fn set_physics_shape(&self, physics_shape: Option<&PhysicsShape>) {
        unsafe {
            scn_physics_body_set_shape(
                self.as_ptr(),
                physics_shape.map_or(ptr::null_mut(), PhysicsShape::as_ptr),
            );
        };
    }
}

impl PhysicsField {
    #[must_use]
    pub fn drag() -> Option<Self> {
        unsafe { Self::from_raw(scn_physics_field_drag()) }
    }

    #[must_use]
    pub fn vortex() -> Option<Self> {
        unsafe { Self::from_raw(scn_physics_field_vortex()) }
    }

    #[must_use]
    pub fn radial_gravity() -> Option<Self> {
        unsafe { Self::from_raw(scn_physics_field_radial_gravity()) }
    }

    #[must_use]
    pub fn linear_gravity() -> Option<Self> {
        unsafe { Self::from_raw(scn_physics_field_linear_gravity()) }
    }

    #[must_use]
    pub fn scope(&self) -> PhysicsFieldScope {
        match unsafe { scn_physics_field_get_scope(self.as_ptr()) } {
            1 => PhysicsFieldScope::OutsideExtent,
            _ => PhysicsFieldScope::InsideExtent,
        }
    }

    pub fn set_scope(&self, scope: PhysicsFieldScope) {
        unsafe { scn_physics_field_set_scope(self.as_ptr(), scope as i32) };
    }
}

impl PhysicsWorld {
    pub fn add_behavior(&self, behavior: &PhysicsBehavior) {
        unsafe { scn_physics_world_add_behavior(self.as_ptr(), behavior.as_ptr()) };
    }

    pub fn remove_behavior(&self, behavior: &PhysicsBehavior) {
        unsafe { scn_physics_world_remove_behavior(self.as_ptr(), behavior.as_ptr()) };
    }

    pub fn remove_all_behaviors(&self) {
        unsafe { scn_physics_world_remove_all_behaviors(self.as_ptr()) };
    }
}

impl Node {
    #[must_use]
    pub fn physics_field(&self) -> Option<PhysicsField> {
        unsafe { PhysicsField::from_raw(scn_node_get_physics_field(self.as_ptr())) }
    }

    pub fn set_physics_field(&self, field: Option<&PhysicsField>) {
        unsafe {
            scn_node_set_physics_field(
                self.as_ptr(),
                field.map_or(ptr::null_mut(), PhysicsField::as_ptr),
            );
        };
    }
}

impl PhysicsBallSocketJoint {
    #[must_use]
    pub fn new(body_a: &PhysicsBody, anchor_a: Vector3, body_b: &PhysicsBody, anchor_b: Vector3) -> Option<Self> {
        let mut anchor_a = anchor_a;
        let mut anchor_b = anchor_b;
        unsafe {
            Self::from_raw(scn_physics_ball_socket_joint_new(
                body_a.as_ptr(),
                anchor_a.as_mut_ptr().cast(),
                body_b.as_ptr(),
                anchor_b.as_mut_ptr().cast(),
            ))
        }
    }

    #[must_use]
    pub fn with_anchor(body: &PhysicsBody, anchor: Vector3) -> Option<Self> {
        let mut anchor = anchor;
        unsafe { Self::from_raw(scn_physics_ball_socket_joint_new_single(body.as_ptr(), anchor.as_mut_ptr().cast())) }
    }
}

impl PhysicsHingeJoint {
    #[must_use]
    pub fn new(
        body_a: &PhysicsBody,
        axis_a: Vector3,
        anchor_a: Vector3,
        body_b: &PhysicsBody,
        axis_b: Vector3,
        anchor_b: Vector3,
    ) -> Option<Self> {
        let mut axis_a = axis_a;
        let mut anchor_a = anchor_a;
        let mut axis_b = axis_b;
        let mut anchor_b = anchor_b;
        unsafe {
            Self::from_raw(scn_physics_hinge_joint_new(
                body_a.as_ptr(),
                axis_a.as_mut_ptr().cast(),
                anchor_a.as_mut_ptr().cast(),
                body_b.as_ptr(),
                axis_b.as_mut_ptr().cast(),
                anchor_b.as_mut_ptr().cast(),
            ))
        }
    }

    #[must_use]
    pub fn with_anchor(body: &PhysicsBody, axis: Vector3, anchor: Vector3) -> Option<Self> {
        let mut axis = axis;
        let mut anchor = anchor;
        unsafe {
            Self::from_raw(scn_physics_hinge_joint_new_single(
                body.as_ptr(),
                axis.as_mut_ptr().cast(),
                anchor.as_mut_ptr().cast(),
            ))
        }
    }
}

impl PhysicsSliderJoint {
    #[must_use]
    pub fn new(
        body_a: &PhysicsBody,
        axis_a: Vector3,
        anchor_a: Vector3,
        body_b: &PhysicsBody,
        axis_b: Vector3,
        anchor_b: Vector3,
    ) -> Option<Self> {
        let mut axis_a = axis_a;
        let mut anchor_a = anchor_a;
        let mut axis_b = axis_b;
        let mut anchor_b = anchor_b;
        unsafe {
            Self::from_raw(scn_physics_slider_joint_new(
                body_a.as_ptr(),
                axis_a.as_mut_ptr().cast(),
                anchor_a.as_mut_ptr().cast(),
                body_b.as_ptr(),
                axis_b.as_mut_ptr().cast(),
                anchor_b.as_mut_ptr().cast(),
            ))
        }
    }

    #[must_use]
    pub fn with_anchor(body: &PhysicsBody, axis: Vector3, anchor: Vector3) -> Option<Self> {
        let mut axis = axis;
        let mut anchor = anchor;
        unsafe {
            Self::from_raw(scn_physics_slider_joint_new_single(
                body.as_ptr(),
                axis.as_mut_ptr().cast(),
                anchor.as_mut_ptr().cast(),
            ))
        }
    }
}

impl PhysicsConeTwistJoint {
    #[must_use]
    pub fn new(body_a: &PhysicsBody, frame_a: Matrix4, body_b: &PhysicsBody, frame_b: Matrix4) -> Option<Self> {
        let mut frame_a = frame_a;
        let mut frame_b = frame_b;
        unsafe {
            Self::from_raw(scn_physics_cone_twist_joint_new(
                body_a.as_ptr(),
                frame_a.as_mut_ptr().cast(),
                body_b.as_ptr(),
                frame_b.as_mut_ptr().cast(),
            ))
        }
    }

    #[must_use]
    pub fn with_frame(body: &PhysicsBody, frame: Matrix4) -> Option<Self> {
        let mut frame = frame;
        unsafe { Self::from_raw(scn_physics_cone_twist_joint_new_single(body.as_ptr(), frame.as_mut_ptr().cast())) }
    }
}

impl PhysicsVehicleWheel {
    #[must_use]
    pub fn with_node(node: &Node) -> Option<Self> {
        unsafe { Self::from_raw(scn_physics_vehicle_wheel_new(node.as_ptr())) }
    }
}

impl PhysicsVehicle {
    #[must_use]
    pub fn new(chassis_body: &PhysicsBody, wheels: &[&PhysicsVehicleWheel]) -> Option<Self> {
        let mut wheel_ptrs: Vec<*mut c_void> = wheels.iter().map(|wheel| wheel.as_ptr()).collect();
        unsafe {
            Self::from_raw(scn_physics_vehicle_new(
                chassis_body.as_ptr(),
                if wheel_ptrs.is_empty() {
                    ptr::null_mut()
                } else {
                    wheel_ptrs.as_mut_ptr().cast()
                },
                wheel_ptrs.len(),
            ))
        }
    }

    #[must_use]
    pub fn speed_in_kilometers_per_hour(&self) -> f64 {
        unsafe { scn_physics_vehicle_get_speed_in_kilometers_per_hour(self.as_ptr()) }
    }

    pub fn apply_engine_force(&self, value: f64, wheel_index: usize) {
        unsafe { scn_physics_vehicle_apply_engine_force(self.as_ptr(), value, wheel_index) };
    }

    pub fn set_steering_angle(&self, value: f64, wheel_index: usize) {
        unsafe { scn_physics_vehicle_set_steering_angle(self.as_ptr(), value, wheel_index) };
    }

    pub fn apply_braking_force(&self, value: f64, wheel_index: usize) {
        unsafe { scn_physics_vehicle_apply_braking_force(self.as_ptr(), value, wheel_index) };
    }
}
