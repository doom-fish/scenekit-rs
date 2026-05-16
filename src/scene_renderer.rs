use core::ffi::c_void;
use core::ptr;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use crate::ffi;
use crate::node::Node;
use crate::private::Sealed;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::view::View;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AntialiasingMode {
    None = 0,
    Multisampling2X = 1,
    Multisampling4X = 2,
    Multisampling8X = 3,
    Multisampling16X = 4,
}

impl AntialiasingMode {
    const fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::Multisampling2X),
            2 => Some(Self::Multisampling4X),
            3 => Some(Self::Multisampling8X),
            4 => Some(Self::Multisampling16X),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RenderingAPI {
    Metal = 0,
    OpenGlLegacy = 1,
    OpenGlCore32 = 2,
    OpenGlCore41 = 3,
}

impl RenderingAPI {
    const fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Metal),
            1 => Some(Self::OpenGlLegacy),
            2 => Some(Self::OpenGlCore32),
            3 => Some(Self::OpenGlCore41),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DebugOptions(usize);

impl DebugOptions {
    pub const NONE: Self = Self(0);
    pub const SHOW_PHYSICS_SHAPES: Self = Self(1 << 0);
    pub const SHOW_BOUNDING_BOXES: Self = Self(1 << 1);
    pub const SHOW_LIGHT_INFLUENCES: Self = Self(1 << 2);
    pub const SHOW_LIGHT_EXTENTS: Self = Self(1 << 3);
    pub const SHOW_PHYSICS_FIELDS: Self = Self(1 << 4);
    pub const SHOW_WIREFRAME: Self = Self(1 << 5);
    pub const RENDER_AS_WIREFRAME: Self = Self(1 << 6);
    pub const SHOW_SKELETONS: Self = Self(1 << 7);
    pub const SHOW_CREASES: Self = Self(1 << 8);
    pub const SHOW_CONSTRAINTS: Self = Self(1 << 9);
    pub const SHOW_CAMERAS: Self = Self(1 << 10);

    #[must_use]
    pub const fn empty() -> Self {
        Self::NONE
    }

    #[must_use]
    pub const fn from_bits(bits: usize) -> Self {
        Self(bits)
    }

    #[must_use]
    pub const fn bits(self) -> usize {
        self.0
    }

    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for DebugOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for DebugOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for DebugOptions {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for DebugOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

type TimeCallback = Box<dyn FnMut(f64)>;
type SceneCallback = Box<dyn FnMut(&Scene, f64)>;

#[derive(Default)]
pub struct SceneRendererDelegateCallbacks {
    update: Option<TimeCallback>,
    did_apply_animations: Option<TimeCallback>,
    did_simulate_physics: Option<TimeCallback>,
    did_apply_constraints: Option<TimeCallback>,
    will_render_scene: Option<SceneCallback>,
    did_render_scene: Option<SceneCallback>,
}

impl SceneRendererDelegateCallbacks {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            update: None,
            did_apply_animations: None,
            did_simulate_physics: None,
            did_apply_constraints: None,
            will_render_scene: None,
            did_render_scene: None,
        }
    }

    #[must_use]
    pub fn on_update<F>(mut self, callback: F) -> Self
    where
        F: FnMut(f64) + 'static,
    {
        self.update = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_did_apply_animations<F>(mut self, callback: F) -> Self
    where
        F: FnMut(f64) + 'static,
    {
        self.did_apply_animations = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_did_simulate_physics<F>(mut self, callback: F) -> Self
    where
        F: FnMut(f64) + 'static,
    {
        self.did_simulate_physics = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_did_apply_constraints<F>(mut self, callback: F) -> Self
    where
        F: FnMut(f64) + 'static,
    {
        self.did_apply_constraints = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_will_render_scene<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&Scene, f64) + 'static,
    {
        self.will_render_scene = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_did_render_scene<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&Scene, f64) + 'static,
    {
        self.did_render_scene = Some(Box::new(callback));
        self
    }
}

struct SceneRendererDelegateState {
    callbacks: SceneRendererDelegateCallbacks,
}

pub struct SceneRendererDelegate {
    ptr: *mut c_void,
}

impl core::fmt::Debug for SceneRendererDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SceneRendererDelegate")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl Drop for SceneRendererDelegate {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::scn_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

unsafe fn delegate_state_from_context<'a>(
    context: *mut c_void,
) -> &'a mut SceneRendererDelegateState {
    &mut *context.cast::<SceneRendererDelegateState>()
}

extern "C" fn release_scene_renderer_delegate_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(context.cast::<SceneRendererDelegateState>()));
    }
}

extern "C" fn scene_renderer_update_trampoline(context: *mut c_void, time: f64) {
    if context.is_null() {
        return;
    }
    let state = unsafe { delegate_state_from_context(context) };
    if let Some(callback) = state.callbacks.update.as_mut() {
        callback(time);
    }
}

extern "C" fn scene_renderer_did_apply_animations_trampoline(context: *mut c_void, time: f64) {
    if context.is_null() {
        return;
    }
    let state = unsafe { delegate_state_from_context(context) };
    if let Some(callback) = state.callbacks.did_apply_animations.as_mut() {
        callback(time);
    }
}

extern "C" fn scene_renderer_did_simulate_physics_trampoline(context: *mut c_void, time: f64) {
    if context.is_null() {
        return;
    }
    let state = unsafe { delegate_state_from_context(context) };
    if let Some(callback) = state.callbacks.did_simulate_physics.as_mut() {
        callback(time);
    }
}

extern "C" fn scene_renderer_did_apply_constraints_trampoline(context: *mut c_void, time: f64) {
    if context.is_null() {
        return;
    }
    let state = unsafe { delegate_state_from_context(context) };
    if let Some(callback) = state.callbacks.did_apply_constraints.as_mut() {
        callback(time);
    }
}

extern "C" fn scene_renderer_will_render_scene_trampoline(
    context: *mut c_void,
    scene: *mut c_void,
    time: f64,
) {
    if context.is_null() || scene.is_null() {
        return;
    }
    let state = unsafe { delegate_state_from_context(context) };
    if let Some(callback) = state.callbacks.will_render_scene.as_mut() {
        let scene = unsafe { Scene::from_raw_borrowed(scene) };
        callback(&scene, time);
    }
}

extern "C" fn scene_renderer_did_render_scene_trampoline(
    context: *mut c_void,
    scene: *mut c_void,
    time: f64,
) {
    if context.is_null() || scene.is_null() {
        return;
    }
    let state = unsafe { delegate_state_from_context(context) };
    if let Some(callback) = state.callbacks.did_render_scene.as_mut() {
        let scene = unsafe { Scene::from_raw_borrowed(scene) };
        callback(&scene, time);
    }
}

impl SceneRendererDelegate {
    #[must_use]
    pub fn new(callbacks: SceneRendererDelegateCallbacks) -> Option<Self> {
        let state = Box::new(SceneRendererDelegateState { callbacks });
        let context = Box::into_raw(state).cast::<c_void>();
        let ptr = unsafe {
            ffi::scn_scene_renderer_delegate_new(
                context,
                release_scene_renderer_delegate_context,
                scene_renderer_update_trampoline,
                scene_renderer_did_apply_animations_trampoline,
                scene_renderer_did_simulate_physics_trampoline,
                scene_renderer_did_apply_constraints_trampoline,
                scene_renderer_will_render_scene_trampoline,
                scene_renderer_did_render_scene_trampoline,
            )
        };
        if ptr.is_null() {
            release_scene_renderer_delegate_context(context);
            None
        } else {
            Some(Self { ptr })
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

pub trait SceneRenderer: Sealed {
    fn scene_renderer_ptr(&self) -> *mut c_void;

    #[must_use]
    fn scene(&self) -> Option<Scene> {
        unsafe { Scene::from_raw(ffi::scn_scene_renderer_get_scene(self.scene_renderer_ptr())) }
    }

    fn set_scene(&self, scene: Option<&Scene>) {
        unsafe {
            ffi::scn_scene_renderer_set_scene(
                self.scene_renderer_ptr(),
                scene.map_or(ptr::null_mut(), Scene::as_ptr),
            );
        };
    }

    #[must_use]
    fn scene_time(&self) -> f64 {
        unsafe { ffi::scn_scene_renderer_get_scene_time(self.scene_renderer_ptr()) }
    }

    fn set_scene_time(&self, scene_time: f64) {
        unsafe { ffi::scn_scene_renderer_set_scene_time(self.scene_renderer_ptr(), scene_time) };
    }

    #[must_use]
    fn point_of_view(&self) -> Option<Node> {
        unsafe {
            Node::from_raw(ffi::scn_scene_renderer_get_point_of_view(
                self.scene_renderer_ptr(),
            ))
        }
    }

    fn set_point_of_view(&self, point_of_view: Option<&Node>) {
        unsafe {
            ffi::scn_scene_renderer_set_point_of_view(
                self.scene_renderer_ptr(),
                point_of_view.map_or(ptr::null_mut(), Node::as_ptr),
            );
        };
    }

    #[must_use]
    fn is_playing(&self) -> bool {
        unsafe { ffi::scn_scene_renderer_get_playing(self.scene_renderer_ptr()) }
    }

    fn set_playing(&self, playing: bool) {
        unsafe { ffi::scn_scene_renderer_set_playing(self.scene_renderer_ptr(), playing) };
    }

    #[must_use]
    fn loops(&self) -> bool {
        unsafe { ffi::scn_scene_renderer_get_loops(self.scene_renderer_ptr()) }
    }

    fn set_loops(&self, loops: bool) {
        unsafe { ffi::scn_scene_renderer_set_loops(self.scene_renderer_ptr(), loops) };
    }

    #[must_use]
    fn autoenables_default_lighting(&self) -> bool {
        unsafe {
            ffi::scn_scene_renderer_get_autoenables_default_lighting(self.scene_renderer_ptr())
        }
    }

    fn set_autoenables_default_lighting(&self, autoenables_default_lighting: bool) {
        unsafe {
            ffi::scn_scene_renderer_set_autoenables_default_lighting(
                self.scene_renderer_ptr(),
                autoenables_default_lighting,
            );
        };
    }

    #[must_use]
    fn jittering_enabled(&self) -> bool {
        unsafe { ffi::scn_scene_renderer_get_jittering_enabled(self.scene_renderer_ptr()) }
    }

    fn set_jittering_enabled(&self, jittering_enabled: bool) {
        unsafe {
            ffi::scn_scene_renderer_set_jittering_enabled(
                self.scene_renderer_ptr(),
                jittering_enabled,
            );
        };
    }

    #[must_use]
    fn temporal_antialiasing_enabled(&self) -> bool {
        unsafe {
            ffi::scn_scene_renderer_get_temporal_antialiasing_enabled(self.scene_renderer_ptr())
        }
    }

    fn set_temporal_antialiasing_enabled(&self, temporal_antialiasing_enabled: bool) {
        unsafe {
            ffi::scn_scene_renderer_set_temporal_antialiasing_enabled(
                self.scene_renderer_ptr(),
                temporal_antialiasing_enabled,
            );
        };
    }

    #[must_use]
    fn shows_statistics(&self) -> bool {
        unsafe { ffi::scn_scene_renderer_get_shows_statistics(self.scene_renderer_ptr()) }
    }

    fn set_shows_statistics(&self, shows_statistics: bool) {
        unsafe {
            ffi::scn_scene_renderer_set_shows_statistics(
                self.scene_renderer_ptr(),
                shows_statistics,
            );
        };
    }

    #[must_use]
    fn debug_options(&self) -> DebugOptions {
        DebugOptions::from_bits(unsafe {
            ffi::scn_scene_renderer_get_debug_options(self.scene_renderer_ptr())
        })
    }

    fn set_debug_options(&self, debug_options: DebugOptions) {
        unsafe {
            ffi::scn_scene_renderer_set_debug_options(
                self.scene_renderer_ptr(),
                debug_options.bits(),
            );
        };
    }

    #[must_use]
    fn rendering_api(&self) -> Option<RenderingAPI> {
        RenderingAPI::from_raw(unsafe {
            ffi::scn_scene_renderer_get_rendering_api(self.scene_renderer_ptr())
        })
    }

    fn set_delegate(&self, delegate: Option<&SceneRendererDelegate>) {
        unsafe {
            ffi::scn_scene_renderer_set_delegate(
                self.scene_renderer_ptr(),
                delegate.map_or(ptr::null_mut(), SceneRendererDelegate::as_ptr),
            );
        };
    }
}

impl Sealed for Renderer {}

impl SceneRenderer for Renderer {
    fn scene_renderer_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Sealed for View {}

impl SceneRenderer for View {
    fn scene_renderer_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl View {
    #[must_use]
    pub fn antialiasing_mode(&self) -> Option<AntialiasingMode> {
        AntialiasingMode::from_raw(unsafe { ffi::scn_view_get_antialiasing_mode(self.ptr) })
    }

    pub fn set_antialiasing_mode(&self, antialiasing_mode: AntialiasingMode) {
        unsafe { ffi::scn_view_set_antialiasing_mode(self.ptr, antialiasing_mode as i32) };
    }
}
