use std::path::Path;

use apple_cf::cg::CGImage;
use apple_metal::MetalTexture;

use crate::color::Color;
use crate::ffi;
use crate::private::{cstring_from_path, handle_type};

handle_type!(Material);
handle_type!(MaterialProperty);

impl Material {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_material_new()) }
    }

    #[must_use]
    pub fn diffuse(&self) -> MaterialProperty {
        unsafe { MaterialProperty::from_raw_unchecked(ffi::scn_material_diffuse(self.ptr)) }
    }

    #[must_use]
    pub fn normal(&self) -> MaterialProperty {
        unsafe { MaterialProperty::from_raw_unchecked(ffi::scn_material_normal(self.ptr)) }
    }

    #[must_use]
    pub fn specular(&self) -> MaterialProperty {
        unsafe { MaterialProperty::from_raw_unchecked(ffi::scn_material_specular(self.ptr)) }
    }

    #[must_use]
    pub fn emission(&self) -> MaterialProperty {
        unsafe { MaterialProperty::from_raw_unchecked(ffi::scn_material_emission(self.ptr)) }
    }

    #[must_use]
    pub fn ambient(&self) -> MaterialProperty {
        unsafe { MaterialProperty::from_raw_unchecked(ffi::scn_material_ambient(self.ptr)) }
    }

    #[must_use]
    pub fn transparent(&self) -> MaterialProperty {
        unsafe { MaterialProperty::from_raw_unchecked(ffi::scn_material_transparent(self.ptr)) }
    }

    #[must_use]
    pub fn multiply(&self) -> MaterialProperty {
        unsafe { MaterialProperty::from_raw_unchecked(ffi::scn_material_multiply(self.ptr)) }
    }
}

impl MaterialProperty {
    pub fn set_color(&self, color: Color) {
        unsafe {
            ffi::scn_material_property_set_color(self.ptr, color.r, color.g, color.b, color.a);
        };
    }

    #[must_use]
    pub fn color(&self) -> Option<Color> {
        let mut rgba = [0.0_f32; 4];
        let ok =
            unsafe { ffi::scn_material_property_copy_color(self.ptr, rgba.as_mut_ptr().cast()) };
        ok.then(|| Color::rgba(rgba[0], rgba[1], rgba[2], rgba[3]))
    }

    pub fn set_cg_image(&self, image: &CGImage) {
        unsafe { ffi::scn_material_property_set_cg_image(self.ptr, image.as_ptr()) };
    }

    pub fn set_metal_texture(&self, texture: &MetalTexture) {
        unsafe { ffi::scn_material_property_set_metal_texture(self.ptr, texture.as_ptr()) };
    }

    pub fn set_file_url(&self, path: impl AsRef<Path>) {
        if let Some(path) = cstring_from_path(path.as_ref()) {
            unsafe { ffi::scn_material_property_set_file_url(self.ptr, path.as_ptr()) };
        }
    }

    pub fn clear_contents(&self) {
        unsafe { ffi::scn_material_property_clear_contents(self.ptr) };
    }

    #[must_use]
    pub fn intensity(&self) -> f64 {
        unsafe { ffi::scn_material_property_get_intensity(self.ptr) }
    }

    pub fn set_intensity(&self, intensity: f64) {
        unsafe { ffi::scn_material_property_set_intensity(self.ptr, intensity) };
    }
}
