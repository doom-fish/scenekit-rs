use core::ffi::c_void;

use crate::ffi;
use crate::material::Material;
use crate::private::{cstring_from_str, handle_type};

handle_type!(Geometry);

impl Geometry {
    #[must_use]
    pub fn box_geometry(width: f64, height: f64, length: f64, chamfer_radius: f64) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::scn_geometry_new_box(
                width,
                height,
                length,
                chamfer_radius,
            ))
        }
    }

    #[must_use]
    pub fn sphere(radius: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_geometry_new_sphere(radius)) }
    }

    #[must_use]
    pub fn cylinder(radius: f64, height: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_geometry_new_cylinder(radius, height)) }
    }

    #[must_use]
    pub fn cone(top_radius: f64, bottom_radius: f64, height: f64) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::scn_geometry_new_cone(
                top_radius,
                bottom_radius,
                height,
            ))
        }
    }

    #[must_use]
    pub fn plane(width: f64, height: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_geometry_new_plane(width, height)) }
    }

    #[must_use]
    pub fn floor() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_geometry_new_floor()) }
    }

    #[must_use]
    pub fn text(text: &str, extrusion_depth: f64) -> Option<Self> {
        let text = cstring_from_str(text)?;
        unsafe { Self::from_raw(ffi::scn_geometry_new_text(text.as_ptr(), extrusion_depth)) }
    }

    /// Wrap a raw `MDLMesh *` pointer and build an `SCNGeometry` from it.
    ///
    /// # Safety
    ///
    /// `mesh` must point to a valid `MDLMesh` object that stays alive for the duration of the call.
    #[must_use]
    pub unsafe fn from_mdl_mesh_raw(mesh: *mut c_void) -> Option<Self> {
        Self::from_raw(ffi::scn_geometry_new_from_mdl_mesh(mesh))
    }

    #[must_use]
    pub fn first_material(&self) -> Option<Material> {
        unsafe { Material::from_raw(ffi::scn_geometry_first_material(self.ptr)) }
    }

    pub fn set_first_material(&self, material: Option<&Material>) {
        unsafe {
            ffi::scn_geometry_set_first_material(
                self.ptr,
                material.map_or(core::ptr::null_mut(), Material::as_ptr),
            );
        };
    }
}
