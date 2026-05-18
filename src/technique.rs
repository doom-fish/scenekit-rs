use core::ptr;

use crate::ffi;
use crate::private::{cstring_from_str, handle_type};
use crate::view::View;

handle_type!(Technique);

impl Technique {
    /// Mirrors `SCNTechnique.minimalDrawScene`.
    #[must_use]
    pub fn minimal_draw_scene() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_technique_new_minimal_draw_scene()) }
    }

    /// Mirrors `SCNTechnique.dictionaryKeyCount`.
    #[must_use]
    pub fn dictionary_key_count(&self) -> usize {
        unsafe { ffi::scn_technique_dictionary_key_count(self.ptr) }
    }

    /// Sets the `SCNTechnique.floatSymbol` member.
    pub fn set_float_symbol(&self, key: &str, value: f64) {
        if let Some(key) = cstring_from_str(key) {
            unsafe { ffi::scn_technique_set_float_symbol(self.ptr, key.as_ptr(), value) };
        }
    }

    /// Mirrors `SCNTechnique.floatSymbol`.
    #[must_use]
    pub fn float_symbol(&self, key: &str) -> Option<f64> {
        let key = cstring_from_str(key)?;
        let mut value = 0.0_f64;
        let ok = unsafe { ffi::scn_technique_get_float_symbol(self.ptr, key.as_ptr(), &mut value) };
        ok.then_some(value)
    }
}

impl View {
    /// Sets the `SCNView.technique` member.
    pub fn set_technique(&self, technique: Option<&Technique>) {
        unsafe {
            ffi::scn_view_set_technique(
                self.ptr,
                technique.map_or(ptr::null_mut(), Technique::as_ptr),
            );
        };
    }

    /// Mirrors `SCNView.technique`.
    #[must_use]
    pub fn technique(&self) -> Option<Technique> {
        unsafe { Technique::from_raw(ffi::scn_view_technique(self.ptr)) }
    }
}
