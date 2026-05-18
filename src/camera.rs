use crate::ffi;
use crate::math::Matrix4;
use crate::private::handle_type;

handle_type!(Camera);

impl Camera {
    /// Creates a wrapped `SCNCamera` instance.
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_camera_new()) }
    }

    /// Mirrors `SCNCamera.fieldOfView`.
    #[must_use]
    pub fn field_of_view(&self) -> f64 {
        unsafe { ffi::scn_camera_get_field_of_view(self.ptr) }
    }

    /// Sets the `SCNCamera.fieldOfView` member.
    pub fn set_field_of_view(&self, field_of_view: f64) {
        unsafe { ffi::scn_camera_set_field_of_view(self.ptr, field_of_view) };
    }

    /// Mirrors `SCNCamera.zNear`.
    #[must_use]
    pub fn z_near(&self) -> f64 {
        unsafe { ffi::scn_camera_get_z_near(self.ptr) }
    }

    /// Sets the `SCNCamera.zNear` member.
    pub fn set_z_near(&self, z_near: f64) {
        unsafe { ffi::scn_camera_set_z_near(self.ptr, z_near) };
    }

    /// Mirrors `SCNCamera.zFar`.
    #[must_use]
    pub fn z_far(&self) -> f64 {
        unsafe { ffi::scn_camera_get_z_far(self.ptr) }
    }

    /// Sets the `SCNCamera.zFar` member.
    pub fn set_z_far(&self, z_far: f64) {
        unsafe { ffi::scn_camera_set_z_far(self.ptr, z_far) };
    }

    /// Mirrors `SCNCamera.projectionMatrix`.
    #[must_use]
    pub fn projection_matrix(&self) -> Matrix4 {
        let mut matrix = Matrix4::default();
        let _ = unsafe {
            ffi::scn_camera_get_projection_transform(self.ptr, matrix.as_mut_ptr().cast())
        };
        matrix
    }

    /// Sets the `SCNCamera.projectionMatrix` member.
    pub fn set_projection_matrix(&self, matrix: Matrix4) {
        unsafe {
            ffi::scn_camera_set_projection_transform(self.ptr, matrix.as_ptr().cast_mut().cast());
        };
    }
}
