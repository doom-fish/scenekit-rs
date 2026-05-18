use core::ptr;

use crate::action::Action;
use crate::camera::Camera;
use crate::error::take_string;
use crate::ffi;
use crate::geometry::Geometry;
use crate::light::Light;
use crate::math::{Matrix4, Vector3, Vector4};
use crate::physics::PhysicsBody;
use crate::private::{cstring_from_str, handle_type};

handle_type!(Node);

impl Node {
    /// Creates a wrapped `SCNNode` instance.
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_node_new()) }
    }

    /// Mirrors `SCNNode.withGeometry`.
    #[must_use]
    pub fn with_geometry(geometry: Option<&Geometry>) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::scn_node_new_with_geometry(
                geometry.map_or(ptr::null_mut(), Geometry::as_ptr),
            ))
        }
    }

    /// Mirrors `SCNNode.addChildNode`.
    pub fn add_child_node(&self, child: &Self) {
        unsafe { ffi::scn_node_add_child(self.ptr, child.ptr) };
    }

    /// Mirrors `SCNNode.removeFromParent`.
    pub fn remove_from_parent(&self) {
        unsafe { ffi::scn_node_remove_from_parent(self.ptr) };
    }

    /// Mirrors `SCNNode.name`.
    #[must_use]
    pub fn name(&self) -> Option<String> {
        unsafe { take_string(ffi::scn_node_copy_name(self.ptr)) }
    }

    /// Sets the `SCNNode.name` member.
    pub fn set_name(&self, name: &str) {
        if let Some(name) = cstring_from_str(name) {
            unsafe { ffi::scn_node_set_name(self.ptr, name.as_ptr()) };
        }
    }

    /// Mirrors `SCNNode.transform`.
    #[must_use]
    pub fn transform(&self) -> Matrix4 {
        let mut matrix = Matrix4::default();
        let _ = unsafe { ffi::scn_node_get_transform(self.ptr, matrix.as_mut_ptr().cast()) };
        matrix
    }

    /// Sets the `SCNNode.transform` member.
    pub fn set_transform(&self, transform: Matrix4) {
        unsafe { ffi::scn_node_set_transform(self.ptr, transform.as_ptr().cast_mut().cast()) };
    }

    /// Mirrors `SCNNode.position`.
    #[must_use]
    pub fn position(&self) -> Vector3 {
        let mut value = Vector3::default();
        let _ = unsafe { ffi::scn_node_get_position(self.ptr, value.as_mut_ptr().cast()) };
        value
    }

    /// Sets the `SCNNode.position` member.
    pub fn set_position(&self, position: Vector3) {
        unsafe { ffi::scn_node_set_position(self.ptr, position.as_ptr().cast_mut().cast()) };
    }

    /// Mirrors `SCNNode.rotation`.
    #[must_use]
    pub fn rotation(&self) -> Vector4 {
        let mut value = Vector4::default();
        let _ = unsafe { ffi::scn_node_get_rotation(self.ptr, value.as_mut_ptr().cast()) };
        value
    }

    /// Sets the `SCNNode.rotation` member.
    pub fn set_rotation(&self, rotation: Vector4) {
        unsafe { ffi::scn_node_set_rotation(self.ptr, rotation.as_ptr().cast_mut().cast()) };
    }

    /// Mirrors `SCNNode.scale`.
    #[must_use]
    pub fn scale(&self) -> Vector3 {
        let mut value = Vector3::default();
        let _ = unsafe { ffi::scn_node_get_scale(self.ptr, value.as_mut_ptr().cast()) };
        value
    }

    /// Sets the `SCNNode.scale` member.
    pub fn set_scale(&self, scale: Vector3) {
        unsafe { ffi::scn_node_set_scale(self.ptr, scale.as_ptr().cast_mut().cast()) };
    }

    /// Mirrors `SCNNode.eulerAngles`.
    #[must_use]
    pub fn euler_angles(&self) -> Vector3 {
        let mut value = Vector3::default();
        let _ = unsafe { ffi::scn_node_get_euler_angles(self.ptr, value.as_mut_ptr().cast()) };
        value
    }

    /// Sets the `SCNNode.eulerAngles` member.
    pub fn set_euler_angles(&self, euler_angles: Vector3) {
        unsafe {
            ffi::scn_node_set_euler_angles(self.ptr, euler_angles.as_ptr().cast_mut().cast());
        };
    }

    /// Mirrors `SCNNode.pivot`.
    #[must_use]
    pub fn pivot(&self) -> Matrix4 {
        let mut matrix = Matrix4::default();
        let _ = unsafe { ffi::scn_node_get_pivot(self.ptr, matrix.as_mut_ptr().cast()) };
        matrix
    }

    /// Sets the `SCNNode.pivot` member.
    pub fn set_pivot(&self, pivot: Matrix4) {
        unsafe { ffi::scn_node_set_pivot(self.ptr, pivot.as_ptr().cast_mut().cast()) };
    }

    /// Returns the `SCNNode.isHidden` value.
    #[must_use]
    pub fn is_hidden(&self) -> bool {
        unsafe { ffi::scn_node_get_hidden(self.ptr) }
    }

    /// Sets the `SCNNode.hidden` member.
    pub fn set_hidden(&self, hidden: bool) {
        unsafe { ffi::scn_node_set_hidden(self.ptr, hidden) };
    }

    /// Mirrors `SCNNode.geometry`.
    #[must_use]
    pub fn geometry(&self) -> Option<Geometry> {
        unsafe { Geometry::from_raw(ffi::scn_node_get_geometry(self.ptr)) }
    }

    /// Sets the `SCNNode.geometry` member.
    pub fn set_geometry(&self, geometry: Option<&Geometry>) {
        unsafe {
            ffi::scn_node_set_geometry(
                self.ptr,
                geometry.map_or(ptr::null_mut(), Geometry::as_ptr),
            );
        };
    }

    /// Mirrors `SCNNode.light`.
    #[must_use]
    pub fn light(&self) -> Option<Light> {
        unsafe { Light::from_raw(ffi::scn_node_get_light(self.ptr)) }
    }

    /// Sets the `SCNNode.light` member.
    pub fn set_light(&self, light: Option<&Light>) {
        unsafe { ffi::scn_node_set_light(self.ptr, light.map_or(ptr::null_mut(), Light::as_ptr)) };
    }

    /// Mirrors `SCNNode.camera`.
    #[must_use]
    pub fn camera(&self) -> Option<Camera> {
        unsafe { Camera::from_raw(ffi::scn_node_get_camera(self.ptr)) }
    }

    /// Sets the `SCNNode.camera` member.
    pub fn set_camera(&self, camera: Option<&Camera>) {
        unsafe {
            ffi::scn_node_set_camera(self.ptr, camera.map_or(ptr::null_mut(), Camera::as_ptr));
        };
    }

    /// Mirrors `SCNNode.physicsBody`.
    #[must_use]
    pub fn physics_body(&self) -> Option<PhysicsBody> {
        unsafe { PhysicsBody::from_raw(ffi::scn_node_get_physics_body(self.ptr)) }
    }

    /// Sets the `SCNNode.physicsBody` member.
    pub fn set_physics_body(&self, physics_body: Option<&PhysicsBody>) {
        unsafe {
            ffi::scn_node_set_physics_body(
                self.ptr,
                physics_body.map_or(ptr::null_mut(), PhysicsBody::as_ptr),
            );
        };
    }

    /// Mirrors `SCNNode.runAction`.
    pub fn run_action(&self, action: &Action) {
        unsafe { ffi::scn_node_run_action(self.ptr, action.as_ptr()) };
    }
}
