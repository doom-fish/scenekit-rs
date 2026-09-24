use apple_cf::cg::CGRect;
use apple_metal::{
    command_buffer_status, storage_mode, texture_type, CommandBuffer, CommandQueue, MetalDevice,
    MetalTexture,
};

use crate::color::Color;
use crate::error::{take_error, take_string, SceneKitError};
use crate::ffi;
use crate::node::Node;
use crate::private::handle_type;
use crate::scene::Scene;

handle_type!(RenderPassDescriptor, "MTLRenderPassDescriptor");
handle_type!(Renderer);

/// Mirrors the Metal load actions used by `SCNRenderer` render passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LoadAction {
    /// Corresponds to the `MTLLoadAction::DontCare` case.
    DontCare = 0,
    /// Corresponds to the `MTLLoadAction::Load` case.
    Load = 1,
    /// Corresponds to the `MTLLoadAction::Clear` case.
    Clear = 2,
}

/// Mirrors the Metal store actions used by `SCNRenderer` render passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StoreAction {
    /// Corresponds to the `MTLStoreAction::DontCare` case.
    DontCare = 0,
    /// Corresponds to the `MTLStoreAction::Store` case.
    Store = 1,
    /// Corresponds to the `MTLStoreAction::MultisampleResolve` case.
    MultisampleResolve = 2,
}

impl RenderPassDescriptor {
    /// Mirrors `SCNRenderer render-pass setup.forTexture`.
    #[must_use]
    pub fn for_texture(texture: &MetalTexture, clear_color: Color) -> Option<Self> {
        Self::for_texture_with_actions(texture, clear_color, LoadAction::Clear, StoreAction::Store)
    }

    /// Mirrors `SCNRenderer render-pass setup.forTextureWithActions`.
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
    /// Creates a wrapped `SCNRenderer` instance.
    #[must_use]
    pub fn new(device: Option<&MetalDevice>) -> Option<Self> {
        unsafe {
            Self::from_raw(ffi::scn_renderer_new(
                device.map_or(core::ptr::null_mut(), MetalDevice::as_ptr),
            ))
        }
    }

    /// Sets the `SCNRenderer.scene` member.
    pub fn set_scene(&self, scene: Option<&Scene>) {
        unsafe {
            ffi::scn_renderer_set_scene(
                self.ptr,
                scene.map_or(core::ptr::null_mut(), Scene::as_ptr),
            );
        };
    }

    /// Sets the `SCNRenderer.pointOfView` member.
    pub fn set_point_of_view(&self, point_of_view: Option<&Node>) {
        unsafe {
            ffi::scn_renderer_set_point_of_view(
                self.ptr,
                point_of_view.map_or(core::ptr::null_mut(), Node::as_ptr),
            );
        };
    }

    /// Mirrors `SCNRenderer.render`.
    pub fn render(
        &self,
        at_time: f64,
        viewport: CGRect,
        queue: &CommandQueue,
        pass_descriptor: &RenderPassDescriptor,
    ) -> Result<CommandBuffer, SceneKitError> {
        let command_buffer = queue.new_command_buffer().ok_or_else(|| {
            SceneKitError::new("MTLCommandQueue could not create a command buffer")
        })?;
        unsafe { self.render_into(at_time, viewport, &command_buffer, pass_descriptor) }?;
        Ok(command_buffer)
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn render_into(
        &self,
        at_time: f64,
        viewport: CGRect,
        command_buffer: &CommandBuffer,
        pass_descriptor: &RenderPassDescriptor,
    ) -> Result<(), SceneKitError> {
        let status = command_buffer.status();
        if status == command_buffer_status::ERROR {
            return Err(SceneKitError::new(format!(
                "the command buffer failed: {}",
                command_buffer
                    .error()
                    .unwrap_or_else(|| "Metal reported no error message".to_owned())
            )));
        }
        if !matches!(
            status,
            command_buffer_status::NOT_ENQUEUED | command_buffer_status::ENQUEUED
        ) {
            return Err(SceneKitError::new(format!(
                "the command buffer was already committed (status {status})"
            )));
        }
        let mut error = core::ptr::null_mut();
        let rendered = unsafe {
            ffi::scn_renderer_render(
                self.ptr,
                at_time,
                viewport.origin.x,
                viewport.origin.y,
                viewport.size.width,
                viewport.size.height,
                command_buffer.as_ptr(),
                pass_descriptor.as_ptr(),
                &raw mut error,
            )
        };
        if rendered {
            drop(unsafe { take_string(error) });
            Ok(())
        } else {
            Err(unsafe { take_error(error, "SCNRenderer refused to render") })
        }
    }
}

/// Reads mipmap level 0 of a 2D texture filled by `SCNRenderer`, sized by its pixel format.
///
/// Compressed, depth, stencil and framebuffer-only textures, and textures without
/// shared or managed storage, are rejected.
pub fn read_texture_bytes(texture: &MetalTexture) -> Result<Vec<u8>, SceneKitError> {
    use apple_metal::pixel_format::{
        DEPTH16UNORM, DEPTH32FLOAT, STENCIL8, X24_STENCIL8, X32_STENCIL8,
    };

    let format = texture.pixel_format();
    if matches!(
        format,
        DEPTH16UNORM | DEPTH32FLOAT | STENCIL8 | X24_STENCIL8 | X32_STENCIL8
    ) {
        return Err(SceneKitError::new(format!(
            "pixel format {format} is a depth or stencil format"
        )));
    }
    let bytes_per_pixel = apple_metal::bytes_per_pixel(format).ok_or_else(|| {
        SceneKitError::new(format!(
            "pixel format {format} is compressed or has no per-pixel layout"
        ))
    })?;
    let mode = texture.storage_mode();
    if !matches!(mode, storage_mode::SHARED | storage_mode::MANAGED) {
        return Err(SceneKitError::new(format!(
            "texture storage mode {mode} is not readable by the CPU"
        )));
    }
    if texture.texture_type() != texture_type::TYPE_2D {
        return Err(SceneKitError::new(format!(
            "texture type {} is not a 2D texture",
            texture.texture_type()
        )));
    }
    let width = texture.width();
    let height = texture.height();
    let bytes_per_row = width
        .checked_mul(bytes_per_pixel)
        .ok_or_else(|| SceneKitError::new("texture row byte count overflowed usize"))?;
    let byte_len = bytes_per_row
        .checked_mul(height)
        .ok_or_else(|| SceneKitError::new("texture byte count overflowed usize"))?;
    let mut bytes = vec![0_u8; byte_len];
    let copied = unsafe {
        ffi::scn_texture_copy_bytes(
            texture.as_ptr(),
            bytes.as_mut_ptr().cast(),
            bytes.len(),
            bytes_per_row,
            bytes_per_pixel,
        )
    };
    if copied {
        Ok(bytes)
    } else {
        Err(SceneKitError::new(
            "MTLTexture refused the CPU copy (framebuffer-only, wrong layout or storage)",
        ))
    }
}
