use crate::ffi;
use crate::node::Node;
use crate::private::{cstring_from_str, handle_type};

handle_type!(Animation);
handle_type!(AnimationPlayer);

impl Animation {
    #[must_use]
    pub fn opacity(from: f32, to: f32, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_animation_new_opacity(from, to, duration)) }
    }

    #[must_use]
    pub fn duration(&self) -> f64 {
        unsafe { ffi::scn_animation_get_duration(self.ptr) }
    }

    pub fn set_duration(&self, duration: f64) {
        unsafe { ffi::scn_animation_set_duration(self.ptr, duration) };
    }

    #[must_use]
    pub fn repeat_count(&self) -> f64 {
        unsafe { ffi::scn_animation_get_repeat_count(self.ptr) }
    }

    pub fn set_repeat_count(&self, repeat_count: f64) {
        unsafe { ffi::scn_animation_set_repeat_count(self.ptr, repeat_count) };
    }

    #[must_use]
    pub fn autoreverses(&self) -> bool {
        unsafe { ffi::scn_animation_get_autoreverses(self.ptr) }
    }

    pub fn set_autoreverses(&self, autoreverses: bool) {
        unsafe { ffi::scn_animation_set_autoreverses(self.ptr, autoreverses) };
    }

    #[must_use]
    pub fn uses_scene_time_base(&self) -> bool {
        unsafe { ffi::scn_animation_get_uses_scene_time_base(self.ptr) }
    }

    pub fn set_uses_scene_time_base(&self, uses_scene_time_base: bool) {
        unsafe {
            ffi::scn_animation_set_uses_scene_time_base(self.ptr, uses_scene_time_base);
        };
    }
}

impl AnimationPlayer {
    #[must_use]
    pub fn new(animation: &Animation) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_animation_player_new(animation.ptr)) }
    }

    #[must_use]
    pub fn animation(&self) -> Option<Animation> {
        unsafe { Animation::from_raw(ffi::scn_animation_player_animation(self.ptr)) }
    }

    #[must_use]
    pub fn speed(&self) -> f64 {
        unsafe { ffi::scn_animation_player_get_speed(self.ptr) }
    }

    pub fn set_speed(&self, speed: f64) {
        unsafe { ffi::scn_animation_player_set_speed(self.ptr, speed) };
    }

    #[must_use]
    pub fn is_paused(&self) -> bool {
        unsafe { ffi::scn_animation_player_get_paused(self.ptr) }
    }

    pub fn set_paused(&self, paused: bool) {
        unsafe { ffi::scn_animation_player_set_paused(self.ptr, paused) };
    }

    pub fn play(&self) {
        unsafe { ffi::scn_animation_player_play(self.ptr) };
    }

    pub fn stop(&self) {
        unsafe { ffi::scn_animation_player_stop(self.ptr) };
    }
}

impl Node {
    pub fn add_animation_player(&self, player: &AnimationPlayer, key: &str) {
        if let Some(key) = cstring_from_str(key) {
            unsafe { ffi::scn_node_add_animation_player(self.ptr, player.ptr, key.as_ptr()) };
        }
    }

    #[must_use]
    pub fn animation_player(&self, key: &str) -> Option<AnimationPlayer> {
        let key = cstring_from_str(key)?;
        unsafe { AnimationPlayer::from_raw(ffi::scn_node_animation_player(self.ptr, key.as_ptr())) }
    }
}
