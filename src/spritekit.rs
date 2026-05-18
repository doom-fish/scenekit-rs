use core::ffi::c_void;

use crate::private::handle_type;

extern "C" {
    fn scn_sprite_scene_new(width: f64, height: f64) -> *mut c_void;
    fn scn_sprite_transition_fade(duration: f64) -> *mut c_void;
}

handle_type!(SpriteScene, "SKScene");
handle_type!(SpriteTransition, "SKTransition");

impl SpriteScene {
    /// Creates a wrapped `SKScene` instance.
    #[must_use]
    pub fn new(width: f64, height: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_sprite_scene_new(width, height)) }
    }
}

impl SpriteTransition {
    /// Mirrors `SKTransition.fade`.
    #[must_use]
    pub fn fade(duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_sprite_transition_fade(duration)) }
    }
}
