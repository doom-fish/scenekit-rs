use crate::ffi;
use crate::node::Node;
use crate::private::{cstring_from_str, handle_type};

handle_type!(Animation);
handle_type!(AnimationPlayer);

impl Animation {
    /// Mirrors `SCNAnimation.opacity`.
    #[must_use]
    pub fn opacity(from: f32, to: f32, duration: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_animation_new_opacity(from, to, duration)) }
    }

    /// Mirrors `SCNAnimation.duration`.
    #[must_use]
    pub fn duration(&self) -> f64 {
        unsafe { ffi::scn_animation_get_duration(self.ptr) }
    }

    /// Sets the `SCNAnimation.duration` member.
    pub fn set_duration(&self, duration: f64) {
        unsafe { ffi::scn_animation_set_duration(self.ptr, duration) };
    }

    /// Mirrors `SCNAnimation.repeatCount`.
    #[must_use]
    pub fn repeat_count(&self) -> f64 {
        unsafe { ffi::scn_animation_get_repeat_count(self.ptr) }
    }

    /// Sets the `SCNAnimation.repeatCount` member.
    pub fn set_repeat_count(&self, repeat_count: f64) {
        unsafe { ffi::scn_animation_set_repeat_count(self.ptr, repeat_count) };
    }

    /// Mirrors `SCNAnimation.autoreverses`.
    #[must_use]
    pub fn autoreverses(&self) -> bool {
        unsafe { ffi::scn_animation_get_autoreverses(self.ptr) }
    }

    /// Sets the `SCNAnimation.autoreverses` member.
    pub fn set_autoreverses(&self, autoreverses: bool) {
        unsafe { ffi::scn_animation_set_autoreverses(self.ptr, autoreverses) };
    }

    /// Mirrors `SCNAnimation.usesSceneTimeBase`.
    #[must_use]
    pub fn uses_scene_time_base(&self) -> bool {
        unsafe { ffi::scn_animation_get_uses_scene_time_base(self.ptr) }
    }

    /// Sets the `SCNAnimation.usesSceneTimeBase` member.
    pub fn set_uses_scene_time_base(&self, uses_scene_time_base: bool) {
        unsafe {
            ffi::scn_animation_set_uses_scene_time_base(self.ptr, uses_scene_time_base);
        };
    }
}

impl AnimationPlayer {
    /// Creates a wrapped `SCNAnimationPlayer` instance.
    #[must_use]
    pub fn new(animation: &Animation) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_animation_player_new(animation.ptr)) }
    }

    /// Mirrors `SCNAnimationPlayer.animation`.
    #[must_use]
    pub fn animation(&self) -> Option<Animation> {
        unsafe { Animation::from_raw(ffi::scn_animation_player_animation(self.ptr)) }
    }

    /// Mirrors `SCNAnimationPlayer.speed`.
    #[must_use]
    pub fn speed(&self) -> f64 {
        unsafe { ffi::scn_animation_player_get_speed(self.ptr) }
    }

    /// Sets the `SCNAnimationPlayer.speed` member.
    pub fn set_speed(&self, speed: f64) {
        unsafe { ffi::scn_animation_player_set_speed(self.ptr, speed) };
    }

    /// Returns the `SCNAnimationPlayer.isPaused` value.
    #[must_use]
    pub fn is_paused(&self) -> bool {
        unsafe { ffi::scn_animation_player_get_paused(self.ptr) }
    }

    /// Sets the `SCNAnimationPlayer.paused` member.
    pub fn set_paused(&self, paused: bool) {
        unsafe { ffi::scn_animation_player_set_paused(self.ptr, paused) };
    }

    /// Mirrors `SCNAnimationPlayer.play`.
    pub fn play(&self) {
        unsafe { ffi::scn_animation_player_play(self.ptr) };
    }

    /// Mirrors `SCNAnimationPlayer.stop`.
    pub fn stop(&self) {
        unsafe { ffi::scn_animation_player_stop(self.ptr) };
    }
}

impl Node {
    /// Mirrors `SCNNode.addAnimationPlayer`.
    pub fn add_animation_player(&self, player: &AnimationPlayer, key: &str) {
        if let Some(key) = cstring_from_str(key) {
            unsafe { ffi::scn_node_add_animation_player(self.ptr, player.ptr, key.as_ptr()) };
        }
    }

    /// Mirrors `SCNNode.animationPlayer`.
    #[must_use]
    pub fn animation_player(&self, key: &str) -> Option<AnimationPlayer> {
        let key = cstring_from_str(key)?;
        unsafe { AnimationPlayer::from_raw(ffi::scn_node_animation_player(self.ptr, key.as_ptr())) }
    }
}
