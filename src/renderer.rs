use apple_cf::cg::CGRect;
use apple_metal::{CommandBuffer, MetalDevice, MetalTexture};

use crate::color::Color;
use crate::error::SceneKitError;
use crate::ffi;
use crate::node::Node;
use crate::private::handle_type;
use crate::scene::Scene;

handle_type!(RenderPassDescriptor);
handle_type!(Renderer);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StoreAction {
    DontCare = 0,
    Store = 1,
    MultisampleResolve = 2,
}

impl RenderPassDescriptor {
    #[must_use]
    pub fn for_texture(texture: &MetalTexture, clear_color: Color) -> Option<Self> {
        Self::for_texture_with_actions(texture, clear_color, LoadAction::Clear, StoreAction::Store)
    }

    #[must_use]
    pub fn for_texture_with_actions(
        texture: &MetalTexture,
        clear_color: Color,
        load_action: LoadAction,
        store_action: StoreAction,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::scn_render_pass_descriptor_new_for_texture(
                texture.as_ptr(),
                clear_color.r.into(),
                clear_color.g.into(),
                clear_color.b.into(),
                clear_color.a.into(),
                load_action as i32,
                store_action as i32,
            ))
        }
    }
}

impl Renderer {
    #[must_use]
    pub fn new(device: Option<&MetalDevice>) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::scn_renderer_new(
                device.map_or(core::ptr::null_mut(), MetalDevice::as_ptr),
            ))
        }
    }

    pub fn set_scene(&self, scene: Option<&Scene>) {
        unsafe {
            ffi::scn_renderer_set_scene(
                self.ptr,
                scene.map_or(core::ptr::null_mut(), Scene::as_ptr),
            );
        };
    }

    pub fn set_point_of_view(&self, point_of_view: Option<&Node>) {
        unsafe {
            ffi::scn_renderer_set_point_of_view(
                self.ptr,
                point_of_view.map_or(core::ptr::null_mut(), Node::as_ptr),
            );
        };
    }

    pub fn render(
        &self,
        at_time: f64,
        viewport: CGRect,
        command_buffer: &CommandBuffer,
        pass_descriptor: &RenderPassDescriptor,
    ) {
        unsafe {
            ffi::scn_renderer_render(
                self.ptr,
                at_time,
                viewport.x,
                viewport.y,
                viewport.width,
                viewport.height,
                command_buffer.as_ptr(),
                pass_descriptor.as_ptr(),
            );
        };
    }
}

pub fn read_texture_bytes(texture: &MetalTexture) -> Result<Vec<u8>, SceneKitError> {
    let width = texture.width();
    let height = texture.height();
    let bytes_per_row = width
        .checked_mul(4)
        .ok_or_else(|| SceneKitError::new("texture row byte count overflowed usize"))?;
    let byte_len = bytes_per_row
        .checked_mul(height)
        .ok_or_else(|| SceneKitError::new("texture byte count overflowed usize"))?;
    let mut bytes = vec![0_u8; byte_len];
    let ok = unsafe {
        ffi::scn_texture_copy_bytes(texture.as_ptr(), bytes.as_mut_ptr().cast(), bytes_per_row)
    };
    if ok {
        Ok(bytes)
    } else {
        Err(SceneKitError::new(
            "failed to copy texture bytes from MTLTexture",
        ))
    }
}
