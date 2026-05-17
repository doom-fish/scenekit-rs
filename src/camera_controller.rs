use core::ffi::c_void;
use core::ptr;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::ffi;
use crate::math::Vector3;
use crate::node::Node;
use crate::private::handle_type;
use crate::view::View;
use crate::{CGPoint, CGSize};

handle_type!(CameraController);
handle_type!(CameraControlConfiguration);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum InteractionMode {
    Fly = 0,
    OrbitTurntable = 1,
    OrbitAngleMapping = 2,
    OrbitCenteredArcball = 3,
    OrbitArcball = 4,
    Pan = 5,
    Truck = 6,
}

impl InteractionMode {
    const fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Fly),
            1 => Some(Self::OrbitTurntable),
            2 => Some(Self::OrbitAngleMapping),
            3 => Some(Self::OrbitCenteredArcball),
            4 => Some(Self::OrbitArcball),
            5 => Some(Self::Pan),
            6 => Some(Self::Truck),
            _ => None,
        }
    }
}

type CameraControllerCallback = Box<dyn FnMut()>;

#[derive(Default)]
pub struct CameraControllerDelegateCallbacks {
    inertia_will_start: Option<CameraControllerCallback>,
    inertia_did_end: Option<CameraControllerCallback>,
}

impl CameraControllerDelegateCallbacks {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inertia_will_start: None,
            inertia_did_end: None,
        }
    }

    #[must_use]
    pub fn on_inertia_will_start<F>(mut self, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.inertia_will_start = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_inertia_did_end<F>(mut self, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.inertia_did_end = Some(Box::new(callback));
        self
    }
}

struct CameraControllerDelegateState {
    callbacks: CameraControllerDelegateCallbacks,
}

pub struct CameraControllerDelegate {
    ptr: *mut c_void,
}

impl core::fmt::Debug for CameraControllerDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CameraControllerDelegate")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl Drop for CameraControllerDelegate {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::scn_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

unsafe fn delegate_state_from_context<'a>(
    context: *mut c_void,
) -> &'a mut CameraControllerDelegateState {
    &mut *context.cast::<CameraControllerDelegateState>()
}

extern "C" fn release_camera_controller_delegate_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(
            context.cast::<CameraControllerDelegateState>(),
        ));
    }
}

extern "C" fn camera_controller_inertia_will_start_trampoline(context: *mut c_void) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if context.is_null() {
            return;
        }
        let state = unsafe { delegate_state_from_context(context) };
        if let Some(callback) = state.callbacks.inertia_will_start.as_mut() {
            callback();
        }
    }));
}

extern "C" fn camera_controller_inertia_did_end_trampoline(context: *mut c_void) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if context.is_null() {
            return;
        }
        let state = unsafe { delegate_state_from_context(context) };
        if let Some(callback) = state.callbacks.inertia_did_end.as_mut() {
            callback();
        }
    }));
}

impl CameraControllerDelegate {
    #[must_use]
    pub fn new(callbacks: CameraControllerDelegateCallbacks) -> Option<Self> {
        let state = Box::new(CameraControllerDelegateState { callbacks });
        let context = Box::into_raw(state).cast::<c_void>();
        let ptr = unsafe {
            ffi::scn_camera_controller_delegate_new(
                context,
                release_camera_controller_delegate_context,
                camera_controller_inertia_will_start_trampoline,
                camera_controller_inertia_did_end_trampoline,
            )
        };
        if ptr.is_null() {
            release_camera_controller_delegate_context(context);
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

impl CameraControlConfiguration {
    #[must_use]
    pub fn auto_switch_to_free_camera(&self) -> bool {
        unsafe { ffi::scn_camera_control_configuration_get_auto_switch_to_free_camera(self.ptr) }
    }

    pub fn set_auto_switch_to_free_camera(&self, auto_switch_to_free_camera: bool) {
        unsafe {
            ffi::scn_camera_control_configuration_set_auto_switch_to_free_camera(
                self.ptr,
                auto_switch_to_free_camera,
            );
        };
    }

    #[must_use]
    pub fn allows_translation(&self) -> bool {
        unsafe { ffi::scn_camera_control_configuration_get_allows_translation(self.ptr) }
    }

    pub fn set_allows_translation(&self, allows_translation: bool) {
        unsafe {
            ffi::scn_camera_control_configuration_set_allows_translation(
                self.ptr,
                allows_translation,
            );
        };
    }

    #[must_use]
    pub fn fly_mode_velocity(&self) -> f64 {
        unsafe { ffi::scn_camera_control_configuration_get_fly_mode_velocity(self.ptr) }
    }

    pub fn set_fly_mode_velocity(&self, fly_mode_velocity: f64) {
        unsafe {
            ffi::scn_camera_control_configuration_set_fly_mode_velocity(
                self.ptr,
                fly_mode_velocity,
            );
        };
    }

    #[must_use]
    pub fn pan_sensitivity(&self) -> f64 {
        unsafe { ffi::scn_camera_control_configuration_get_pan_sensitivity(self.ptr) }
    }

    pub fn set_pan_sensitivity(&self, pan_sensitivity: f64) {
        unsafe {
            ffi::scn_camera_control_configuration_set_pan_sensitivity(self.ptr, pan_sensitivity);
        };
    }

    #[must_use]
    pub fn truck_sensitivity(&self) -> f64 {
        unsafe { ffi::scn_camera_control_configuration_get_truck_sensitivity(self.ptr) }
    }

    pub fn set_truck_sensitivity(&self, truck_sensitivity: f64) {
        unsafe {
            ffi::scn_camera_control_configuration_set_truck_sensitivity(
                self.ptr,
                truck_sensitivity,
            );
        };
    }

    #[must_use]
    pub fn rotation_sensitivity(&self) -> f64 {
        unsafe { ffi::scn_camera_control_configuration_get_rotation_sensitivity(self.ptr) }
    }

    pub fn set_rotation_sensitivity(&self, rotation_sensitivity: f64) {
        unsafe {
            ffi::scn_camera_control_configuration_set_rotation_sensitivity(
                self.ptr,
                rotation_sensitivity,
            );
        };
    }
}

impl CameraController {
    pub fn set_delegate(&self, delegate: Option<&CameraControllerDelegate>) {
        unsafe {
            ffi::scn_camera_controller_set_delegate(
                self.ptr,
                delegate.map_or(ptr::null_mut(), CameraControllerDelegate::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn point_of_view(&self) -> Option<Node> {
        unsafe { Node::from_raw(ffi::scn_camera_controller_get_point_of_view(self.ptr)) }
    }

    pub fn set_point_of_view(&self, point_of_view: Option<&Node>) {
        unsafe {
            ffi::scn_camera_controller_set_point_of_view(
                self.ptr,
                point_of_view.map_or(ptr::null_mut(), Node::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn interaction_mode(&self) -> Option<InteractionMode> {
        InteractionMode::from_raw(unsafe {
            ffi::scn_camera_controller_get_interaction_mode(self.ptr)
        })
    }

    pub fn set_interaction_mode(&self, interaction_mode: InteractionMode) {
        unsafe {
            ffi::scn_camera_controller_set_interaction_mode(self.ptr, interaction_mode as i32);
        };
    }

    #[must_use]
    pub fn target(&self) -> Vector3 {
        let mut target = Vector3::default();
        let _ =
            unsafe { ffi::scn_camera_controller_get_target(self.ptr, target.as_mut_ptr().cast()) };
        target
    }

    pub fn set_target(&self, target: Vector3) {
        unsafe {
            ffi::scn_camera_controller_set_target(self.ptr, target.as_ptr().cast_mut().cast());
        };
    }

    #[must_use]
    pub fn automatic_target(&self) -> bool {
        unsafe { ffi::scn_camera_controller_get_automatic_target(self.ptr) }
    }

    pub fn set_automatic_target(&self, automatic_target: bool) {
        unsafe { ffi::scn_camera_controller_set_automatic_target(self.ptr, automatic_target) };
    }

    #[must_use]
    pub fn world_up(&self) -> Vector3 {
        let mut world_up = Vector3::default();
        let _ = unsafe {
            ffi::scn_camera_controller_get_world_up(self.ptr, world_up.as_mut_ptr().cast())
        };
        world_up
    }

    pub fn set_world_up(&self, world_up: Vector3) {
        unsafe {
            ffi::scn_camera_controller_set_world_up(self.ptr, world_up.as_ptr().cast_mut().cast());
        };
    }

    #[must_use]
    pub fn inertia_enabled(&self) -> bool {
        unsafe { ffi::scn_camera_controller_get_inertia_enabled(self.ptr) }
    }

    pub fn set_inertia_enabled(&self, inertia_enabled: bool) {
        unsafe { ffi::scn_camera_controller_set_inertia_enabled(self.ptr, inertia_enabled) };
    }

    #[must_use]
    pub fn inertia_friction(&self) -> f32 {
        unsafe { ffi::scn_camera_controller_get_inertia_friction(self.ptr) }
    }

    pub fn set_inertia_friction(&self, inertia_friction: f32) {
        unsafe { ffi::scn_camera_controller_set_inertia_friction(self.ptr, inertia_friction) };
    }

    #[must_use]
    pub fn is_inertia_running(&self) -> bool {
        unsafe { ffi::scn_camera_controller_get_inertia_running(self.ptr) }
    }

    #[must_use]
    pub fn minimum_vertical_angle(&self) -> f32 {
        unsafe { ffi::scn_camera_controller_get_minimum_vertical_angle(self.ptr) }
    }

    pub fn set_minimum_vertical_angle(&self, minimum_vertical_angle: f32) {
        unsafe {
            ffi::scn_camera_controller_set_minimum_vertical_angle(self.ptr, minimum_vertical_angle);
        };
    }

    #[must_use]
    pub fn maximum_vertical_angle(&self) -> f32 {
        unsafe { ffi::scn_camera_controller_get_maximum_vertical_angle(self.ptr) }
    }

    pub fn set_maximum_vertical_angle(&self, maximum_vertical_angle: f32) {
        unsafe {
            ffi::scn_camera_controller_set_maximum_vertical_angle(self.ptr, maximum_vertical_angle);
        };
    }

    #[must_use]
    pub fn minimum_horizontal_angle(&self) -> f32 {
        unsafe { ffi::scn_camera_controller_get_minimum_horizontal_angle(self.ptr) }
    }

    pub fn set_minimum_horizontal_angle(&self, minimum_horizontal_angle: f32) {
        unsafe {
            ffi::scn_camera_controller_set_minimum_horizontal_angle(
                self.ptr,
                minimum_horizontal_angle,
            );
        };
    }

    #[must_use]
    pub fn maximum_horizontal_angle(&self) -> f32 {
        unsafe { ffi::scn_camera_controller_get_maximum_horizontal_angle(self.ptr) }
    }

    pub fn set_maximum_horizontal_angle(&self, maximum_horizontal_angle: f32) {
        unsafe {
            ffi::scn_camera_controller_set_maximum_horizontal_angle(
                self.ptr,
                maximum_horizontal_angle,
            );
        };
    }

    pub fn translate_in_camera_space(&self, delta: Vector3) {
        unsafe {
            ffi::scn_camera_controller_translate_in_camera_space(
                self.ptr, delta.x, delta.y, delta.z,
            );
        };
    }

    pub fn frame_nodes(&self, nodes: &[Node]) {
        let raw_nodes = nodes.iter().map(Node::as_ptr).collect::<Vec<_>>();
        unsafe {
            ffi::scn_camera_controller_frame_nodes(
                self.ptr,
                raw_nodes.as_ptr().cast::<c_void>().cast_mut(),
                raw_nodes.len(),
            );
        };
    }

    pub fn rotate_by(&self, delta_x: f32, delta_y: f32) {
        unsafe { ffi::scn_camera_controller_rotate_by(self.ptr, delta_x, delta_y) };
    }

    pub fn roll_by(&self, delta: f32, around_screen_point: CGPoint, viewport: CGSize) {
        unsafe {
            ffi::scn_camera_controller_roll_by(
                self.ptr,
                delta,
                around_screen_point.x,
                around_screen_point.y,
                viewport.width,
                viewport.height,
            );
        };
    }

    pub fn dolly_by(&self, delta: f32, on_screen_point: CGPoint, viewport: CGSize) {
        unsafe {
            ffi::scn_camera_controller_dolly_by(
                self.ptr,
                delta,
                on_screen_point.x,
                on_screen_point.y,
                viewport.width,
                viewport.height,
            );
        };
    }

    pub fn roll_around_target(&self, delta: f32) {
        unsafe { ffi::scn_camera_controller_roll_around_target(self.ptr, delta) };
    }

    pub fn dolly_to_target(&self, delta: f32) {
        unsafe { ffi::scn_camera_controller_dolly_to_target(self.ptr, delta) };
    }

    pub fn clear_roll(&self) {
        unsafe { ffi::scn_camera_controller_clear_roll(self.ptr) };
    }

    pub fn stop_inertia(&self) {
        unsafe { ffi::scn_camera_controller_stop_inertia(self.ptr) };
    }

    pub fn begin_interaction(&self, location: CGPoint, viewport: CGSize) {
        unsafe {
            ffi::scn_camera_controller_begin_interaction(
                self.ptr,
                location.x,
                location.y,
                viewport.width,
                viewport.height,
            );
        };
    }

    pub fn continue_interaction(&self, location: CGPoint, viewport: CGSize, sensitivity: f64) {
        unsafe {
            ffi::scn_camera_controller_continue_interaction(
                self.ptr,
                location.x,
                location.y,
                viewport.width,
                viewport.height,
                sensitivity,
            );
        };
    }

    pub fn end_interaction(&self, location: CGPoint, viewport: CGSize, velocity: CGPoint) {
        unsafe {
            ffi::scn_camera_controller_end_interaction(
                self.ptr,
                location.x,
                location.y,
                viewport.width,
                viewport.height,
                velocity.x,
                velocity.y,
            );
        };
    }
}

impl View {
    #[must_use]
    pub fn camera_control_configuration(&self) -> Option<CameraControlConfiguration> {
        unsafe {
            CameraControlConfiguration::from_raw(ffi::scn_view_camera_control_configuration(
                self.ptr,
            ))
        }
    }

    #[must_use]
    pub fn default_camera_controller(&self) -> Option<CameraController> {
        unsafe { CameraController::from_raw(ffi::scn_view_default_camera_controller(self.ptr)) }
    }
}
