use apple_cf::cg::CGColorSpace;
use core::ffi::c_void;
use core::ptr;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use crate::ffi;
use crate::geometry::Geometry;
use crate::hit_test::HitTestResults;
use crate::material::Material;
use crate::node::Node;
use crate::private::{handle_type, Sealed};
use crate::renderer::{RenderPassDescriptor, Renderer};
use crate::scene::Scene;
use crate::spritekit::{SpriteScene, SpriteTransition};
use crate::view::View;

extern "C" {
    fn scn_scene_renderer_present_scene(
        renderer: *mut c_void,
        scene: *mut c_void,
        transition: *mut c_void,
        point_of_view: *mut c_void,
    );
    fn scn_scene_renderer_get_delegate(renderer: *mut c_void) -> *mut c_void;
    fn scn_scene_renderer_hit_test(renderer: *mut c_void, x: f64, y: f64) -> *mut c_void;
    fn scn_scene_renderer_is_node_inside_frustum(
        renderer: *mut c_void,
        node: *mut c_void,
        point_of_view: *mut c_void,
    ) -> bool;
    fn scn_scene_renderer_nodes_inside_frustum(
        renderer: *mut c_void,
        point_of_view: *mut c_void,
    ) -> *mut c_void;
    fn scn_array_count(array: *mut c_void) -> usize;
    fn scn_array_get(array: *mut c_void, index: usize) -> *mut c_void;
    fn scn_scene_renderer_project_point(
        renderer: *mut c_void,
        point: *mut c_void,
        out_point: *mut c_void,
    ) -> bool;
    fn scn_scene_renderer_unproject_point(
        renderer: *mut c_void,
        point: *mut c_void,
        out_point: *mut c_void,
    ) -> bool;
    fn scn_scene_renderer_prepare_object(renderer: *mut c_void, object: *mut c_void) -> bool;
    fn scn_scene_renderer_prepare_objects(
        renderer: *mut c_void,
        objects: *mut c_void,
        count: usize,
    ) -> bool;
    fn scn_scene_renderer_get_overlay_sk_scene(renderer: *mut c_void) -> *mut c_void;
    fn scn_scene_renderer_set_overlay_sk_scene(renderer: *mut c_void, scene: *mut c_void);
    fn scn_scene_renderer_get_working_color_space(renderer: *mut c_void) -> *mut c_void;
    fn scn_scene_renderer_get_current_render_command_encoder(renderer: *mut c_void) -> *mut c_void;
    fn scn_scene_renderer_get_current_render_pass_descriptor(renderer: *mut c_void) -> *mut c_void;
    fn scn_scene_renderer_get_device(renderer: *mut c_void) -> *mut c_void;
    fn scn_scene_renderer_get_color_pixel_format(renderer: *mut c_void) -> usize;
    fn scn_scene_renderer_get_depth_pixel_format(renderer: *mut c_void) -> usize;
    fn scn_scene_renderer_get_stencil_pixel_format(renderer: *mut c_void) -> usize;
    fn scn_scene_renderer_get_command_queue(renderer: *mut c_void) -> *mut c_void;
    fn scn_scene_renderer_get_audio_engine(renderer: *mut c_void) -> *mut c_void;
    fn scn_scene_renderer_get_audio_environment_node(renderer: *mut c_void) -> *mut c_void;
    fn scn_scene_renderer_get_audio_listener(renderer: *mut c_void) -> *mut c_void;
    fn scn_scene_renderer_set_audio_listener(renderer: *mut c_void, listener: *mut c_void);
    fn scn_scene_renderer_get_current_viewport(
        renderer: *mut c_void,
        out_rect: *mut c_void,
    ) -> bool;
    fn scn_scene_renderer_get_current_time(renderer: *mut c_void) -> f64;
    fn scn_scene_renderer_set_current_time(renderer: *mut c_void, value: f64);
    fn scn_scene_renderer_get_uses_reverse_z(renderer: *mut c_void) -> bool;
    fn scn_scene_renderer_set_uses_reverse_z(renderer: *mut c_void, value: bool);
}

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

handle_type!(NodeArray);
handle_type!(AudioEngine);
handle_type!(AudioEnvironmentNode);
handle_type!(MetalDeviceHandle);
handle_type!(MetalCommandQueue);
handle_type!(MetalRenderCommandEncoder);

impl NodeArray {
    fn nodes(&self) -> Vec<Node> {
        (0..unsafe { scn_array_count(self.ptr) })
            .filter_map(|index| unsafe { Node::from_raw(scn_array_get(self.ptr, index)) })
            .collect()
    }
}

pub trait Prepareable: Sealed {
    fn as_prepareable_ptr(&self) -> *mut c_void;
}

impl Sealed for Scene {}

impl Prepareable for Scene {
    fn as_prepareable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Prepareable for Node {
    fn as_prepareable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Prepareable for Geometry {
    fn as_prepareable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Prepareable for Material {
    fn as_prepareable_ptr(&self) -> *mut c_void {
        self.as_ptr()
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

    fn present_scene(
        &self,
        scene: &Scene,
        transition: Option<&SpriteTransition>,
        point_of_view: Option<&Node>,
    ) {
        unsafe {
            scn_scene_renderer_present_scene(
                self.scene_renderer_ptr(),
                scene.as_ptr(),
                transition.map_or(ptr::null_mut(), SpriteTransition::as_ptr),
                point_of_view.map_or(ptr::null_mut(), Node::as_ptr),
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
    fn hit_test(&self, point: crate::CGPoint) -> Option<HitTestResults> {
        unsafe {
            HitTestResults::from_raw(scn_scene_renderer_hit_test(
                self.scene_renderer_ptr(),
                point.x,
                point.y,
            ))
        }
    }

    #[must_use]
    fn is_node_inside_frustum(&self, node: &Node, point_of_view: &Node) -> bool {
        unsafe {
            scn_scene_renderer_is_node_inside_frustum(
                self.scene_renderer_ptr(),
                node.as_ptr(),
                point_of_view.as_ptr(),
            )
        }
    }

    #[must_use]
    fn nodes_inside_frustum(&self, point_of_view: &Node) -> Vec<Node> {
        unsafe {
            NodeArray::from_raw(scn_scene_renderer_nodes_inside_frustum(
                self.scene_renderer_ptr(),
                point_of_view.as_ptr(),
            ))
            .map_or_else(Vec::new, |nodes| nodes.nodes())
        }
    }

    #[must_use]
    fn project_point(&self, point: crate::Vector3) -> Option<crate::Vector3> {
        let mut point = point;
        let mut projected = crate::Vector3::default();
        unsafe {
            scn_scene_renderer_project_point(
                self.scene_renderer_ptr(),
                point.as_mut_ptr().cast(),
                projected.as_mut_ptr().cast(),
            )
        }
        .then_some(projected)
    }

    #[must_use]
    fn unproject_point(&self, point: crate::Vector3) -> Option<crate::Vector3> {
        let mut point = point;
        let mut unprojected = crate::Vector3::default();
        unsafe {
            scn_scene_renderer_unproject_point(
                self.scene_renderer_ptr(),
                point.as_mut_ptr().cast(),
                unprojected.as_mut_ptr().cast(),
            )
        }
        .then_some(unprojected)
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

    #[must_use]
    fn delegate(&self) -> Option<SceneRendererDelegate> {
        unsafe {
            Some(SceneRendererDelegate {
                ptr: scn_scene_renderer_get_delegate(self.scene_renderer_ptr()),
            })
            .filter(|delegate| !delegate.ptr.is_null())
        }
    }

    #[must_use]
    fn prepare_object(&self, object: &dyn Prepareable) -> bool {
        unsafe {
            scn_scene_renderer_prepare_object(
                self.scene_renderer_ptr(),
                object.as_prepareable_ptr(),
            )
        }
    }

    #[must_use]
    fn prepare_objects(&self, objects: &[&dyn Prepareable]) -> bool {
        let mut object_ptrs: Vec<*mut c_void> = objects
            .iter()
            .map(|object| object.as_prepareable_ptr())
            .collect();
        unsafe {
            scn_scene_renderer_prepare_objects(
                self.scene_renderer_ptr(),
                if object_ptrs.is_empty() {
                    ptr::null_mut()
                } else {
                    object_ptrs.as_mut_ptr().cast()
                },
                object_ptrs.len(),
            )
        }
    }

    #[must_use]
    fn overlay_scene(&self) -> Option<SpriteScene> {
        unsafe {
            SpriteScene::from_raw(scn_scene_renderer_get_overlay_sk_scene(
                self.scene_renderer_ptr(),
            ))
        }
    }

    fn set_overlay_scene(&self, scene: Option<&SpriteScene>) {
        unsafe {
            scn_scene_renderer_set_overlay_sk_scene(
                self.scene_renderer_ptr(),
                scene.map_or(ptr::null_mut(), SpriteScene::as_ptr),
            );
        };
    }

    #[must_use]
    fn working_color_space(&self) -> Option<CGColorSpace> {
        let ptr = unsafe { scn_scene_renderer_get_working_color_space(self.scene_renderer_ptr()) };
        (!ptr.is_null()).then(|| unsafe { CGColorSpace::from_raw(ptr) })
    }

    #[must_use]
    fn current_render_command_encoder(&self) -> Option<MetalRenderCommandEncoder> {
        unsafe {
            MetalRenderCommandEncoder::from_raw(
                scn_scene_renderer_get_current_render_command_encoder(self.scene_renderer_ptr()),
            )
        }
    }

    #[must_use]
    fn current_render_pass_descriptor(&self) -> Option<RenderPassDescriptor> {
        unsafe {
            RenderPassDescriptor::from_raw(scn_scene_renderer_get_current_render_pass_descriptor(
                self.scene_renderer_ptr(),
            ))
        }
    }

    #[must_use]
    fn device(&self) -> Option<MetalDeviceHandle> {
        unsafe {
            MetalDeviceHandle::from_raw(scn_scene_renderer_get_device(self.scene_renderer_ptr()))
        }
    }

    #[must_use]
    fn color_pixel_format(&self) -> usize {
        unsafe { scn_scene_renderer_get_color_pixel_format(self.scene_renderer_ptr()) }
    }

    #[must_use]
    fn depth_pixel_format(&self) -> usize {
        unsafe { scn_scene_renderer_get_depth_pixel_format(self.scene_renderer_ptr()) }
    }

    #[must_use]
    fn stencil_pixel_format(&self) -> usize {
        unsafe { scn_scene_renderer_get_stencil_pixel_format(self.scene_renderer_ptr()) }
    }

    #[must_use]
    fn command_queue(&self) -> Option<MetalCommandQueue> {
        unsafe {
            MetalCommandQueue::from_raw(scn_scene_renderer_get_command_queue(
                self.scene_renderer_ptr(),
            ))
        }
    }

    #[must_use]
    fn audio_engine(&self) -> Option<AudioEngine> {
        unsafe {
            AudioEngine::from_raw(scn_scene_renderer_get_audio_engine(
                self.scene_renderer_ptr(),
            ))
        }
    }

    #[must_use]
    fn audio_environment_node(&self) -> Option<AudioEnvironmentNode> {
        unsafe {
            AudioEnvironmentNode::from_raw(scn_scene_renderer_get_audio_environment_node(
                self.scene_renderer_ptr(),
            ))
        }
    }

    #[must_use]
    fn audio_listener(&self) -> Option<Node> {
        unsafe {
            Node::from_raw(scn_scene_renderer_get_audio_listener(
                self.scene_renderer_ptr(),
            ))
        }
    }

    fn set_audio_listener(&self, listener: Option<&Node>) {
        unsafe {
            scn_scene_renderer_set_audio_listener(
                self.scene_renderer_ptr(),
                listener.map_or(ptr::null_mut(), Node::as_ptr),
            );
        };
    }

    #[must_use]
    fn current_viewport(&self) -> Option<crate::CGRect> {
        let mut rect = crate::CGRect::new(0.0, 0.0, 0.0, 0.0);
        unsafe {
            scn_scene_renderer_get_current_viewport(
                self.scene_renderer_ptr(),
                std::ptr::addr_of_mut!(rect).cast::<c_void>(),
            )
        }
        .then_some(rect)
    }

    #[must_use]
    fn current_time(&self) -> f64 {
        unsafe { scn_scene_renderer_get_current_time(self.scene_renderer_ptr()) }
    }

    fn set_current_time(&self, value: f64) {
        unsafe { scn_scene_renderer_set_current_time(self.scene_renderer_ptr(), value) };
    }

    #[must_use]
    fn uses_reverse_z(&self) -> bool {
        unsafe { scn_scene_renderer_get_uses_reverse_z(self.scene_renderer_ptr()) }
    }

    fn set_uses_reverse_z(&self, value: bool) {
        unsafe { scn_scene_renderer_set_uses_reverse_z(self.scene_renderer_ptr(), value) };
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
