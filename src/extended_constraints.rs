use core::ffi::c_void;
use core::ops::Deref;
use core::ptr;

use crate::constraint::Constraint;
use crate::math::Vector3;
use crate::node::Node;
use crate::symbols::BillboardAxis;

extern "C" {
    fn scn_billboard_constraint_new() -> *mut c_void;
    fn scn_billboard_constraint_get_free_axes(constraint: *mut c_void) -> usize;
    fn scn_billboard_constraint_set_free_axes(constraint: *mut c_void, free_axes: usize);

    fn scn_transform_constraint_new_passthrough(world_space: bool) -> *mut c_void;

    fn scn_ik_constraint_new(chain_root_node: *mut c_void) -> *mut c_void;
    fn scn_ik_constraint_get_target_position(
        constraint: *mut c_void,
        out_vector: *mut c_void,
    ) -> bool;
    fn scn_ik_constraint_set_target_position(constraint: *mut c_void, target_position: *mut c_void);

    fn scn_replicator_constraint_new(target: *mut c_void) -> *mut c_void;
    fn scn_replicator_constraint_get_replicates_position(constraint: *mut c_void) -> bool;
    fn scn_replicator_constraint_set_replicates_position(constraint: *mut c_void, value: bool);
    fn scn_replicator_constraint_get_replicates_orientation(constraint: *mut c_void) -> bool;
    fn scn_replicator_constraint_set_replicates_orientation(constraint: *mut c_void, value: bool);
    fn scn_replicator_constraint_get_replicates_scale(constraint: *mut c_void) -> bool;
    fn scn_replicator_constraint_set_replicates_scale(constraint: *mut c_void, value: bool);

    fn scn_acceleration_constraint_new() -> *mut c_void;
    fn scn_acceleration_constraint_get_maximum_linear_acceleration(constraint: *mut c_void) -> f64;
    fn scn_acceleration_constraint_set_maximum_linear_acceleration(
        constraint: *mut c_void,
        value: f64,
    );

    fn scn_slider_constraint_new() -> *mut c_void;
    fn scn_slider_constraint_get_collision_category_bit_mask(constraint: *mut c_void) -> usize;
    fn scn_slider_constraint_set_collision_category_bit_mask(constraint: *mut c_void, mask: usize);

    fn scn_avoid_occluder_constraint_new(target: *mut c_void) -> *mut c_void;
    fn scn_avoid_occluder_constraint_get_target(constraint: *mut c_void) -> *mut c_void;
    fn scn_avoid_occluder_constraint_set_target(constraint: *mut c_void, target: *mut c_void);
    fn scn_avoid_occluder_constraint_get_occluder_category_bit_mask(
        constraint: *mut c_void,
    ) -> usize;
    fn scn_avoid_occluder_constraint_set_occluder_category_bit_mask(
        constraint: *mut c_void,
        mask: usize,
    );
    fn scn_avoid_occluder_constraint_get_bias(constraint: *mut c_void) -> f64;
    fn scn_avoid_occluder_constraint_set_bias(constraint: *mut c_void, bias: f64);
}

macro_rules! constraint_newtype {
    ($name:ident) => {
        pub struct $name(Constraint);

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl Deref for $name {
            type Target = Constraint;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$name> for Constraint {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl $name {
            unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
                Constraint::from_raw(ptr).map(Self)
            }
        }
    };
}

constraint_newtype!(BillboardConstraint);
constraint_newtype!(TransformConstraint);
constraint_newtype!(IKConstraint);
constraint_newtype!(ReplicatorConstraint);
constraint_newtype!(AccelerationConstraint);
constraint_newtype!(SliderConstraint);
constraint_newtype!(AvoidOccluderConstraint);

impl BillboardConstraint {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(scn_billboard_constraint_new()) }
    }

    #[must_use]
    pub fn free_axes(&self) -> BillboardAxis {
        BillboardAxis::from_bits(unsafe { scn_billboard_constraint_get_free_axes(self.as_ptr()) })
    }

    pub fn set_free_axes(&self, free_axes: BillboardAxis) {
        unsafe { scn_billboard_constraint_set_free_axes(self.as_ptr(), free_axes.bits()) };
    }
}

impl TransformConstraint {
    #[must_use]
    pub fn passthrough(world_space: bool) -> Option<Self> {
        unsafe { Self::from_raw(scn_transform_constraint_new_passthrough(world_space)) }
    }
}

impl IKConstraint {
    #[must_use]
    pub fn new(chain_root_node: &Node) -> Option<Self> {
        unsafe { Self::from_raw(scn_ik_constraint_new(chain_root_node.as_ptr())) }
    }

    #[must_use]
    pub fn target_position(&self) -> Option<Vector3> {
        let mut value = Vector3::default();
        let ok = unsafe {
            scn_ik_constraint_get_target_position(self.as_ptr(), value.as_mut_ptr().cast())
        };
        ok.then_some(value)
    }

    pub fn set_target_position(&self, target_position: Vector3) {
        let mut target_position = target_position;
        unsafe {
            scn_ik_constraint_set_target_position(
                self.as_ptr(),
                target_position.as_mut_ptr().cast(),
            );
        };
    }
}

impl ReplicatorConstraint {
    #[must_use]
    pub fn new(target: Option<&Node>) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_replicator_constraint_new(
                target.map_or(ptr::null_mut(), Node::as_ptr),
            ))
        }
    }

    #[must_use]
    pub fn replicates_position(&self) -> bool {
        unsafe { scn_replicator_constraint_get_replicates_position(self.as_ptr()) }
    }

    pub fn set_replicates_position(&self, value: bool) {
        unsafe { scn_replicator_constraint_set_replicates_position(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn replicates_orientation(&self) -> bool {
        unsafe { scn_replicator_constraint_get_replicates_orientation(self.as_ptr()) }
    }

    pub fn set_replicates_orientation(&self, value: bool) {
        unsafe { scn_replicator_constraint_set_replicates_orientation(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn replicates_scale(&self) -> bool {
        unsafe { scn_replicator_constraint_get_replicates_scale(self.as_ptr()) }
    }

    pub fn set_replicates_scale(&self, value: bool) {
        unsafe { scn_replicator_constraint_set_replicates_scale(self.as_ptr(), value) };
    }
}

impl AccelerationConstraint {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(scn_acceleration_constraint_new()) }
    }

    #[must_use]
    pub fn maximum_linear_acceleration(&self) -> f64 {
        unsafe { scn_acceleration_constraint_get_maximum_linear_acceleration(self.as_ptr()) }
    }

    pub fn set_maximum_linear_acceleration(&self, value: f64) {
        unsafe {
            scn_acceleration_constraint_set_maximum_linear_acceleration(self.as_ptr(), value);
        };
    }
}

impl SliderConstraint {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(scn_slider_constraint_new()) }
    }

    #[must_use]
    pub fn collision_category_bit_mask(&self) -> usize {
        unsafe { scn_slider_constraint_get_collision_category_bit_mask(self.as_ptr()) }
    }

    pub fn set_collision_category_bit_mask(&self, mask: usize) {
        unsafe { scn_slider_constraint_set_collision_category_bit_mask(self.as_ptr(), mask) };
    }
}

impl AvoidOccluderConstraint {
    #[must_use]
    pub fn new(target: Option<&Node>) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_avoid_occluder_constraint_new(
                target.map_or(ptr::null_mut(), Node::as_ptr),
            ))
        }
    }

    #[must_use]
    pub fn target(&self) -> Option<Node> {
        unsafe { Node::from_raw(scn_avoid_occluder_constraint_get_target(self.as_ptr())) }
    }

    pub fn set_target(&self, target: Option<&Node>) {
        unsafe {
            scn_avoid_occluder_constraint_set_target(
                self.as_ptr(),
                target.map_or(ptr::null_mut(), Node::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn occluder_category_bit_mask(&self) -> usize {
        unsafe { scn_avoid_occluder_constraint_get_occluder_category_bit_mask(self.as_ptr()) }
    }

    pub fn set_occluder_category_bit_mask(&self, mask: usize) {
        unsafe {
            scn_avoid_occluder_constraint_set_occluder_category_bit_mask(self.as_ptr(), mask);
        };
    }

    #[must_use]
    pub fn bias(&self) -> f64 {
        unsafe { scn_avoid_occluder_constraint_get_bias(self.as_ptr()) }
    }

    pub fn set_bias(&self, bias: f64) {
        unsafe { scn_avoid_occluder_constraint_set_bias(self.as_ptr(), bias) };
    }
}
