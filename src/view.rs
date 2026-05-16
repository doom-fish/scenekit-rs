use core::ptr;

use crate::color::Color;
use crate::ffi;
use crate::node::Node;
use crate::private::handle_type;
use crate::scene::Scene;

handle_type!(View);

impl View {
    #[must_use]
    pub fn new(width: f64, height: f64) -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_view_new(width, height)) }
    }

    pub fn set_scene(&self, scene: Option<&Scene>) {
        unsafe {
            ffi::scn_view_set_scene(self.ptr, scene.map_or(ptr::null_mut(), Scene::as_ptr));
        };
    }

    #[must_use]
    pub fn scene(&self) -> Option<Scene> {
        unsafe { Scene::from_raw(ffi::scn_view_scene(self.ptr)) }
    }

    pub fn set_point_of_view(&self, point_of_view: Option<&Node>) {
        unsafe {
            ffi::scn_view_set_point_of_view(
                self.ptr,
                point_of_view.map_or(ptr::null_mut(), Node::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn point_of_view(&self) -> Option<Node> {
        unsafe { Node::from_raw(ffi::scn_view_point_of_view(self.ptr)) }
    }

    #[must_use]
    pub fn allows_camera_control(&self) -> bool {
        unsafe { ffi::scn_view_get_allows_camera_control(self.ptr) }
    }

    pub fn set_allows_camera_control(&self, allows_camera_control: bool) {
        unsafe { ffi::scn_view_set_allows_camera_control(self.ptr, allows_camera_control) };
    }

    #[must_use]
    pub fn renders_continuously(&self) -> bool {
        unsafe { ffi::scn_view_get_renders_continuously(self.ptr) }
    }

    pub fn set_renders_continuously(&self, renders_continuously: bool) {
        unsafe { ffi::scn_view_set_renders_continuously(self.ptr, renders_continuously) };
    }

    #[must_use]
    pub fn background_color(&self) -> Option<Color> {
        let mut rgba = [0.0_f32; 4];
        let ok = unsafe { ffi::scn_view_copy_background_color(self.ptr, rgba.as_mut_ptr().cast()) };
        ok.then(|| Color::rgba(rgba[0], rgba[1], rgba[2], rgba[3]))
    }

    pub fn set_background_color(&self, color: Color) {
        unsafe { ffi::scn_view_set_background_color(self.ptr, color.r, color.g, color.b, color.a) };
    }

    #[must_use]
    pub fn snapshot_dimensions(&self) -> Option<(f64, f64)> {
        let mut size = [0.0_f64; 2];
        let ok = unsafe { ffi::scn_view_snapshot_size(self.ptr, size.as_mut_ptr()) };
        ok.then_some(size.into())
    }

    #[must_use]
    pub fn preferred_frames_per_second(&self) -> isize {
        unsafe { ffi::scn_view_get_preferred_frames_per_second(self.ptr) }
    }

    pub fn set_preferred_frames_per_second(&self, preferred_frames_per_second: isize) {
        unsafe {
            ffi::scn_view_set_preferred_frames_per_second(self.ptr, preferred_frames_per_second);
        };
    }
}
