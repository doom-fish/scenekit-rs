use std::path::Path;

use crate::color::Color;
use crate::error::{take_error, SceneKitError};
use crate::ffi;
use crate::material::MaterialProperty;
use crate::node::Node;
use crate::private::{cstring_from_path, cstring_from_str, handle_type};

handle_type!(Scene);

impl Scene {
    /// Creates a wrapped `SCNScene` instance.
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_scene_new()) }
    }

    /// Mirrors `SCNScene.named`.
    #[must_use]
    pub fn named(name: &str) -> Option<Self> {
        let name = cstring_from_str(name)?;
        unsafe { Self::from_raw(ffi::scn_scene_new_named(name.as_ptr())) }
    }

    /// Mirrors `SCNScene.fromUrl`.
    pub fn from_url(path: impl AsRef<Path>) -> Result<Self, SceneKitError> {
        let path = cstring_from_path(path.as_ref())
            .ok_or_else(|| SceneKitError::new("path contains an interior NUL byte"))?;
        let mut error = core::ptr::null_mut();
        let ptr = unsafe { ffi::scn_scene_new_url(path.as_ptr(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { take_error(error, "SCNScene(url:) returned nil") })
        } else {
            Ok(unsafe { Self::from_raw_unchecked(ptr) })
        }
    }

    /// Mirrors `SCNScene.rootNode`.
    #[must_use]
    pub fn root_node(&self) -> Node {
        unsafe { Node::from_raw_unchecked(ffi::scn_scene_root_node(self.ptr)) }
    }

    /// Mirrors `SCNScene.background`.
    #[must_use]
    pub fn background(&self) -> MaterialProperty {
        unsafe { MaterialProperty::from_raw_unchecked(ffi::scn_scene_background(self.ptr)) }
    }

    /// Mirrors `SCNScene.lightingEnvironment`.
    #[must_use]
    pub fn lighting_environment(&self) -> MaterialProperty {
        unsafe {
            MaterialProperty::from_raw_unchecked(ffi::scn_scene_lighting_environment(self.ptr))
        }
    }

    /// Sets the `SCNScene.fogColor` member.
    pub fn set_fog_color(&self, color: Color) {
        unsafe { ffi::scn_scene_set_fog_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    /// Mirrors `SCNScene.fogColor`.
    #[must_use]
    pub fn fog_color(&self) -> Option<Color> {
        let mut rgba = [0.0_f32; 4];
        let ok = unsafe { ffi::scn_scene_copy_fog_color(self.ptr, rgba.as_mut_ptr().cast()) };
        ok.then(|| Color::rgba(rgba[0], rgba[1], rgba[2], rgba[3]))
    }
}
