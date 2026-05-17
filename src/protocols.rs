use core::ffi::{c_char, c_void};
use core::ptr;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::action::Action;
use crate::animation::{Animation, AnimationPlayer};
use crate::camera::Camera;
use crate::constraint::Constraint;
use crate::error::take_string;
use crate::geometry::Geometry;
use crate::light::Light;
use crate::material::{Material, MaterialProperty};
use crate::math::Vector3;
use crate::node::Node;
use crate::particle_system::ParticleSystem;
use crate::private::{cstring_from_str, handle_type, Sealed};
use crate::renderer::Renderer;
use crate::technique::Technique;
use crate::view::View;

extern "C" {
    fn scn_actionable_run_action_for_key(
        object: *mut c_void,
        action: *mut c_void,
        key: *const c_char,
    );
    fn scn_actionable_has_actions(object: *mut c_void) -> bool;
    fn scn_actionable_action_for_key(object: *mut c_void, key: *const c_char) -> *mut c_void;
    fn scn_actionable_remove_action_for_key(object: *mut c_void, key: *const c_char);
    fn scn_actionable_remove_all_actions(object: *mut c_void);
    fn scn_actionable_action_keys(object: *mut c_void) -> *mut c_void;

    fn scn_animatable_add_animation(
        object: *mut c_void,
        animation: *mut c_void,
        key: *const c_char,
    );
    fn scn_animatable_add_animation_player(
        object: *mut c_void,
        player: *mut c_void,
        key: *const c_char,
    );
    fn scn_animatable_remove_all_animations(object: *mut c_void);
    fn scn_animatable_remove_animation_for_key(object: *mut c_void, key: *const c_char);
    fn scn_animatable_animation_player(object: *mut c_void, key: *const c_char) -> *mut c_void;
    fn scn_animatable_animation_keys(object: *mut c_void) -> *mut c_void;

    fn scn_bounding_volume_get_bounding_box(
        object: *mut c_void,
        min: *mut c_void,
        max: *mut c_void,
    ) -> bool;
    fn scn_bounding_volume_set_bounding_box(
        object: *mut c_void,
        min: *mut c_void,
        max: *mut c_void,
    );
    fn scn_bounding_volume_get_bounding_sphere(
        object: *mut c_void,
        center: *mut c_void,
        radius: *mut f64,
    ) -> bool;

    fn scn_technique_support_get_technique(object: *mut c_void) -> *mut c_void;
    fn scn_technique_support_set_technique(object: *mut c_void, technique: *mut c_void);

    fn scn_string_array_count(array: *mut c_void) -> usize;
    fn scn_string_array_get(array: *mut c_void, index: usize) -> *mut c_char;

    fn scn_timing_function_new_mode(mode: i32) -> *mut c_void;
    fn scn_animation_event_new(
        key_time: f32,
        context: *mut c_void,
        release_context: extern "C" fn(*mut c_void),
        callback: extern "C" fn(*mut c_void, bool),
    ) -> *mut c_void;
}

handle_type!(StringArray);
handle_type!(TimingFunction);
handle_type!(AnimationEvent);

impl StringArray {
    fn to_vec(&self) -> Vec<String> {
        (0..unsafe { scn_string_array_count(self.ptr) })
            .filter_map(|index| unsafe { take_string(scn_string_array_get(self.ptr, index)) })
            .collect()
    }
}

pub trait Actionable: Sealed {
    fn actionable_ptr(&self) -> *mut c_void;

    fn run_action(&self, action: &Action) {
        unsafe { crate::ffi::scn_node_run_action(self.actionable_ptr(), action.as_ptr()) };
    }

    fn run_action_for_key(&self, action: &Action, key: &str) {
        if let Some(key) = cstring_from_str(key) {
            unsafe {
                scn_actionable_run_action_for_key(
                    self.actionable_ptr(),
                    action.as_ptr(),
                    key.as_ptr(),
                );
            };
        }
    }

    #[must_use]
    fn has_actions(&self) -> bool {
        unsafe { scn_actionable_has_actions(self.actionable_ptr()) }
    }

    #[must_use]
    fn action_for_key(&self, key: &str) -> Option<Action> {
        let key = cstring_from_str(key)?;
        unsafe {
            Action::from_raw(scn_actionable_action_for_key(
                self.actionable_ptr(),
                key.as_ptr(),
            ))
        }
    }

    fn remove_action_for_key(&self, key: &str) {
        if let Some(key) = cstring_from_str(key) {
            unsafe { scn_actionable_remove_action_for_key(self.actionable_ptr(), key.as_ptr()) };
        }
    }

    fn remove_all_actions(&self) {
        unsafe { scn_actionable_remove_all_actions(self.actionable_ptr()) };
    }

    #[must_use]
    fn action_keys(&self) -> Vec<String> {
        unsafe { StringArray::from_raw(scn_actionable_action_keys(self.actionable_ptr())) }
            .map_or_else(Vec::new, |array| array.to_vec())
    }
}

pub trait Animatable: Sealed {
    fn animatable_ptr(&self) -> *mut c_void;

    fn add_animation(&self, animation: &Animation, key: Option<&str>) {
        let key = key.and_then(cstring_from_str);
        unsafe {
            scn_animatable_add_animation(
                self.animatable_ptr(),
                animation.as_ptr(),
                key.as_ref().map_or(ptr::null(), |key| key.as_ptr()),
            );
        };
    }

    fn add_animation_player(&self, player: &AnimationPlayer, key: Option<&str>) {
        let key = key.and_then(cstring_from_str);
        unsafe {
            scn_animatable_add_animation_player(
                self.animatable_ptr(),
                player.as_ptr(),
                key.as_ref().map_or(ptr::null(), |key| key.as_ptr()),
            );
        };
    }

    fn remove_all_animations(&self) {
        unsafe { scn_animatable_remove_all_animations(self.animatable_ptr()) };
    }

    fn remove_animation_for_key(&self, key: &str) {
        if let Some(key) = cstring_from_str(key) {
            unsafe { scn_animatable_remove_animation_for_key(self.animatable_ptr(), key.as_ptr()) };
        }
    }

    #[must_use]
    fn animation_player(&self, key: &str) -> Option<AnimationPlayer> {
        let key = cstring_from_str(key)?;
        unsafe {
            AnimationPlayer::from_raw(scn_animatable_animation_player(
                self.animatable_ptr(),
                key.as_ptr(),
            ))
        }
    }

    #[must_use]
    fn animation_keys(&self) -> Vec<String> {
        unsafe { StringArray::from_raw(scn_animatable_animation_keys(self.animatable_ptr())) }
            .map_or_else(Vec::new, |array| array.to_vec())
    }
}

pub trait BoundingVolume: Sealed {
    fn bounding_volume_ptr(&self) -> *mut c_void;

    #[must_use]
    fn bounding_box(&self) -> Option<(Vector3, Vector3)> {
        let mut min = Vector3::default();
        let mut max = Vector3::default();
        let ok = unsafe {
            scn_bounding_volume_get_bounding_box(
                self.bounding_volume_ptr(),
                min.as_mut_ptr().cast(),
                max.as_mut_ptr().cast(),
            )
        };
        ok.then_some((min, max))
    }

    fn set_bounding_box(&self, bounding_box: Option<(Vector3, Vector3)>) {
        match bounding_box {
            Some((mut min, mut max)) => unsafe {
                scn_bounding_volume_set_bounding_box(
                    self.bounding_volume_ptr(),
                    min.as_mut_ptr().cast(),
                    max.as_mut_ptr().cast(),
                );
            },
            None => unsafe {
                scn_bounding_volume_set_bounding_box(
                    self.bounding_volume_ptr(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
            },
        }
    }

    #[must_use]
    fn bounding_sphere(&self) -> Option<(Vector3, f64)> {
        let mut center = Vector3::default();
        let mut radius = 0.0_f64;
        let ok = unsafe {
            scn_bounding_volume_get_bounding_sphere(
                self.bounding_volume_ptr(),
                center.as_mut_ptr().cast(),
                &mut radius,
            )
        };
        ok.then_some((center, radius))
    }
}

pub trait TechniqueSupport: Sealed {
    fn technique_support_ptr(&self) -> *mut c_void;

    #[must_use]
    fn technique(&self) -> Option<Technique> {
        unsafe {
            Technique::from_raw(scn_technique_support_get_technique(
                self.technique_support_ptr(),
            ))
        }
    }

    fn set_technique(&self, technique: Option<&Technique>) {
        unsafe {
            scn_technique_support_set_technique(
                self.technique_support_ptr(),
                technique.map_or(ptr::null_mut(), Technique::as_ptr),
            );
        };
    }
}

impl TimingFunction {
    #[must_use]
    pub fn with_timing_mode(mode: crate::symbols::ActionTimingMode) -> Option<Self> {
        unsafe { Self::from_raw(scn_timing_function_new_mode(mode as i32)) }
    }
}

type AnimationEventCallback = Box<dyn FnMut(bool)>;

struct AnimationEventState {
    callback: AnimationEventCallback,
}

unsafe fn animation_event_state_from_context<'a>(
    context: *mut c_void,
) -> &'a mut AnimationEventState {
    &mut *context.cast::<AnimationEventState>()
}

extern "C" fn release_animation_event_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(context.cast::<AnimationEventState>()));
    }
}

extern "C" fn animation_event_trampoline(context: *mut c_void, playing_backward: bool) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if context.is_null() {
            return;
        }
        let state = unsafe { animation_event_state_from_context(context) };
        (state.callback)(playing_backward);
    }));
}

impl AnimationEvent {
    #[must_use]
    pub fn new<F>(key_time: f32, callback: F) -> Option<Self>
    where
        F: FnMut(bool) + 'static,
    {
        let state = Box::new(AnimationEventState {
            callback: Box::new(callback),
        });
        let context = Box::into_raw(state).cast::<c_void>();
        let ptr = unsafe {
            scn_animation_event_new(
                key_time,
                context,
                release_animation_event_context,
                animation_event_trampoline,
            )
        };
        if ptr.is_null() {
            release_animation_event_context(context);
            None
        } else {
            unsafe { Self::from_raw(ptr) }
        }
    }
}

impl Sealed for Node {}
impl Actionable for Node {
    fn actionable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Sealed for MaterialProperty {}
impl Sealed for Camera {}
impl Sealed for Light {}
impl Sealed for ParticleSystem {}
impl Sealed for Constraint {}

impl Animatable for Node {
    fn animatable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Animatable for Geometry {
    fn animatable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Animatable for Material {
    fn animatable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Animatable for MaterialProperty {
    fn animatable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Animatable for Camera {
    fn animatable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Animatable for Light {
    fn animatable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Animatable for ParticleSystem {
    fn animatable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Animatable for Constraint {
    fn animatable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl BoundingVolume for Node {
    fn bounding_volume_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl BoundingVolume for Geometry {
    fn bounding_volume_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl TechniqueSupport for View {
    fn technique_support_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl TechniqueSupport for Renderer {
    fn technique_support_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl TechniqueSupport for Camera {
    fn technique_support_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}
