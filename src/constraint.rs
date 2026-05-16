use core::ffi::c_void;
use core::ptr;

use crate::ffi;
use crate::node::Node;
use crate::private::handle_type;

handle_type!(Constraint);

impl Constraint {
    #[must_use]
    pub fn look_at(target: Option<&Node>) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::scn_constraint_new_look_at(
                target.map_or(ptr::null_mut(), Node::as_ptr),
            ))
        }
    }

    #[must_use]
    pub fn distance(target: Option<&Node>) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::scn_constraint_new_distance(
                target.map_or(ptr::null_mut(), Node::as_ptr),
            ))
        }
    }

    #[must_use]
    pub fn influence_factor(&self) -> f64 {
        unsafe { ffi::scn_constraint_get_influence_factor(self.ptr) }
    }

    pub fn set_influence_factor(&self, influence_factor: f64) {
        unsafe { ffi::scn_constraint_set_influence_factor(self.ptr, influence_factor) };
    }

    #[must_use]
    pub fn gimbal_lock_enabled(&self) -> bool {
        unsafe { ffi::scn_constraint_get_gimbal_lock_enabled(self.ptr) }
    }

    pub fn set_gimbal_lock_enabled(&self, gimbal_lock_enabled: bool) {
        unsafe { ffi::scn_constraint_set_gimbal_lock_enabled(self.ptr, gimbal_lock_enabled) };
    }

    #[must_use]
    pub fn minimum_distance(&self) -> f64 {
        unsafe { ffi::scn_constraint_get_minimum_distance(self.ptr) }
    }

    pub fn set_minimum_distance(&self, minimum_distance: f64) {
        unsafe { ffi::scn_constraint_set_minimum_distance(self.ptr, minimum_distance) };
    }

    #[must_use]
    pub fn maximum_distance(&self) -> f64 {
        unsafe { ffi::scn_constraint_get_maximum_distance(self.ptr) }
    }

    pub fn set_maximum_distance(&self, maximum_distance: f64) {
        unsafe { ffi::scn_constraint_set_maximum_distance(self.ptr, maximum_distance) };
    }
}

impl Node {
    pub fn set_constraints(&self, constraints: &[&Constraint]) {
        let mut raw: Vec<*mut c_void> = constraints
            .iter()
            .map(|constraint| constraint.as_ptr())
            .collect();
        let raw_ptr = if constraints.is_empty() {
            ptr::null_mut()
        } else {
            raw.as_mut_ptr().cast()
        };
        unsafe { ffi::scn_node_set_constraints(self.ptr, raw_ptr, constraints.len()) };
    }

    #[must_use]
    pub fn constraints_count(&self) -> usize {
        unsafe { ffi::scn_node_constraints_count(self.ptr) }
    }
}
