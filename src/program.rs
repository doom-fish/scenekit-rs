use core::ffi::{c_char, c_void};
use core::ptr;

use apple_metal::MetalLibrary;

use crate::error::{take_error, take_string, SceneKitError};
use crate::ffi;
use crate::geometry::Geometry;
use crate::material::Material;
use crate::private::{
    cstring_from_str, handle_type, invoke_callback, lookup_string_constant, CallbackState,
    DelegateObject, Sealed,
};

handle_type!(BufferStream);
handle_type!(Program);

/// Mirrors `SCNBufferFrequency`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BufferFrequency {
    /// Corresponds to the `SCNBufferFrequency::PerFrame` case.
    PerFrame = 0,
    /// Corresponds to the `SCNBufferFrequency::PerNode` case.
    PerNode = 1,
    /// Corresponds to the `SCNBufferFrequency::PerShadable` case.
    PerShadable = 2,
}

impl BufferFrequency {
    /// Mirrors `SCNBufferFrequency.fromRaw`.
    #[must_use]
    pub const fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::PerFrame),
            1 => Some(Self::PerNode),
            2 => Some(Self::PerShadable),
            _ => None,
        }
    }
}

macro_rules! string_constant_fn {
    ($name:ident, $symbol:literal) => {
        #[doc = concat!("Returns the SceneKit constant `", $symbol, "`.")]
        #[must_use]
        pub fn $name() -> String {
            lookup_string_constant($symbol)
        }
    };
}

string_constant_fn!(program_mapping_channel_key, "SCNProgramMappingChannelKey");
string_constant_fn!(
    shader_modifier_entry_point_fragment,
    "SCNShaderModifierEntryPointFragment"
);
string_constant_fn!(
    shader_modifier_entry_point_geometry,
    "SCNShaderModifierEntryPointGeometry"
);
string_constant_fn!(
    shader_modifier_entry_point_lighting_model,
    "SCNShaderModifierEntryPointLightingModel"
);
string_constant_fn!(
    shader_modifier_entry_point_surface,
    "SCNShaderModifierEntryPointSurface"
);

type ProgramErrorCallback = Box<dyn FnMut(SceneKitError) + Send>;
type BufferBindingCallback = Box<dyn FnMut(&BufferStream) + Send>;

/// Wraps `SCNProgramDelegate`.
#[derive(Debug)]
pub struct ProgramDelegate {
    inner: DelegateObject<ProgramErrorCallback>,
}

/// Wraps `SCNBufferBindingBlock`.
#[derive(Debug)]
pub struct ProgramBufferBinding {
    inner: DelegateObject<BufferBindingCallback>,
}

unsafe extern "C" fn program_delegate_handle_error_trampoline(
    context: *mut c_void,
    message: *mut c_char,
) {
    let message = unsafe { take_string(message) }
        .unwrap_or_else(|| "SCNProgramDelegate.handleError invoked without a message".to_owned());
    unsafe {
        invoke_callback::<ProgramErrorCallback, _>(
            context,
            "scenekit::ProgramDelegate::handle_error",
            |callback| callback(SceneKitError::new(message)),
        );
    }
}

unsafe extern "C" fn program_buffer_binding_trampoline(
    context: *mut c_void,
    buffer_stream: *mut c_void,
) {
    if buffer_stream.is_null() {
        return;
    }
    let buffer_stream = unsafe { BufferStream::from_raw_borrowed(buffer_stream) };
    unsafe {
        invoke_callback::<BufferBindingCallback, _>(
            context,
            "scenekit::ProgramBufferBinding::bind",
            |callback| callback(&buffer_stream),
        );
    }
}

impl ProgramDelegate {
    /// Creates a wrapped `SCNProgramDelegate` instance.
    #[must_use]
    pub fn new<F>(callback: F) -> Option<Self>
    where
        F: FnMut(SceneKitError) + Send + 'static,
    {
        let callback: ProgramErrorCallback = Box::new(callback);
        DelegateObject::new(callback, |context| unsafe {
            ffi::scn_program_delegate_new(
                context,
                CallbackState::<ProgramErrorCallback>::RELEASE,
                program_delegate_handle_error_trampoline,
            )
        })
        .map(|inner| Self { inner })
    }

    /// Returns the Objective-C pointer backing this `SCNProgramDelegate` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.inner.as_ptr()
    }
}

impl ProgramBufferBinding {
    /// Creates a wrapped `SCNBufferBindingBlock` instance.
    #[must_use]
    pub fn new<F>(callback: F) -> Option<Self>
    where
        F: FnMut(&BufferStream) + Send + 'static,
    {
        let callback: BufferBindingCallback = Box::new(callback);
        DelegateObject::new(callback, |context| unsafe {
            ffi::scn_program_buffer_binding_new(
                context,
                CallbackState::<BufferBindingCallback>::RELEASE,
                program_buffer_binding_trampoline,
            )
        })
        .map(|inner| Self { inner })
    }

    /// Returns the Objective-C pointer backing this `SCNBufferBindingBlock` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.inner.as_ptr()
    }
}

/// Mirrors the `SCNShadable` protocol.
pub trait Shadable: Sealed {
    /// Mirrors the `SCNShadable.program` protocol requirement.
    #[must_use]
    fn program(&self) -> Option<Program>;

    /// Sets the `SCNShadable.program` member.
    fn set_program(&self, program: Option<&Program>);

    /// Mirrors the `SCNShadable.shaderModifier` protocol requirement.
    #[must_use]
    fn shader_modifier(&self, entry_point: &str) -> Option<String>;

    /// Sets the `SCNShadable.shaderModifier` member.
    fn set_shader_modifier(&self, entry_point: &str, shader_modifier: Option<&str>);
}

impl Sealed for Geometry {}

impl Shadable for Geometry {
    fn program(&self) -> Option<Program> {
        unsafe { Program::from_raw(ffi::scn_geometry_get_program(self.ptr)) }
    }

    fn set_program(&self, program: Option<&Program>) {
        unsafe {
            ffi::scn_geometry_set_program(
                self.ptr,
                program.map_or(ptr::null_mut(), Program::as_ptr),
            );
        };
    }

    fn shader_modifier(&self, entry_point: &str) -> Option<String> {
        let entry_point = cstring_from_str(entry_point)?;
        unsafe {
            take_string(ffi::scn_geometry_copy_shader_modifier(
                self.ptr,
                entry_point.as_ptr(),
            ))
        }
    }

    fn set_shader_modifier(&self, entry_point: &str, shader_modifier: Option<&str>) {
        let Some(entry_point) = cstring_from_str(entry_point) else {
            return;
        };
        let shader_modifier = shader_modifier.and_then(cstring_from_str);
        unsafe {
            ffi::scn_geometry_set_shader_modifier(
                self.ptr,
                entry_point.as_ptr(),
                shader_modifier
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
    }
}

impl Sealed for Material {}

impl Shadable for Material {
    fn program(&self) -> Option<Program> {
        unsafe { Program::from_raw(ffi::scn_material_get_program(self.ptr)) }
    }

    fn set_program(&self, program: Option<&Program>) {
        unsafe {
            ffi::scn_material_set_program(
                self.ptr,
                program.map_or(ptr::null_mut(), Program::as_ptr),
            );
        };
    }

    fn shader_modifier(&self, entry_point: &str) -> Option<String> {
        let entry_point = cstring_from_str(entry_point)?;
        unsafe {
            take_string(ffi::scn_material_copy_shader_modifier(
                self.ptr,
                entry_point.as_ptr(),
            ))
        }
    }

    fn set_shader_modifier(&self, entry_point: &str, shader_modifier: Option<&str>) {
        let Some(entry_point) = cstring_from_str(entry_point) else {
            return;
        };
        let shader_modifier = shader_modifier.and_then(cstring_from_str);
        unsafe {
            ffi::scn_material_set_shader_modifier(
                self.ptr,
                entry_point.as_ptr(),
                shader_modifier
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
    }
}

impl BufferStream {
    #[must_use]
    pub fn required_length(&self) -> Option<usize> {
        let mut length = 0;
        unsafe { ffi::scn_buffer_stream_required_length(self.ptr, &raw mut length) }
            .then_some(length)
    }

    #[must_use]
    pub fn maximum_length(&self) -> usize {
        unsafe { ffi::scn_buffer_stream_maximum_length(self.ptr) }
    }

    /// Mirrors `SCNBufferStream.writeBytes`.
    pub fn write_bytes(&self, bytes: &[u8]) -> Result<(), SceneKitError> {
        let mut error = ptr::null_mut();
        let written = unsafe {
            ffi::scn_buffer_stream_write_bytes(
                self.ptr,
                bytes.as_ptr().cast(),
                bytes.len(),
                true,
                &raw mut error,
            )
        };
        if written {
            Ok(())
        } else {
            Err(unsafe { take_error(error, "SCNBufferStream rejected the write") })
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn write_bytes_unchecked(&self, bytes: &[u8]) -> Result<(), SceneKitError> {
        let mut error = ptr::null_mut();
        let written = unsafe {
            ffi::scn_buffer_stream_write_bytes(
                self.ptr,
                bytes.as_ptr().cast(),
                bytes.len(),
                false,
                &raw mut error,
            )
        };
        if written {
            Ok(())
        } else {
            Err(unsafe { take_error(error, "SCNBufferStream rejected the write") })
        }
    }
}

impl Program {
    /// Creates a wrapped `SCNProgram` instance.
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(ffi::scn_program_new()) }
    }

    /// Mirrors `SCNProgram.vertexShader`.
    #[must_use]
    pub fn vertex_shader(&self) -> Option<String> {
        unsafe { take_string(ffi::scn_program_copy_vertex_shader(self.ptr)) }
    }

    /// Sets the `SCNProgram.vertexShader` member.
    pub fn set_vertex_shader(&self, vertex_shader: Option<&str>) {
        let vertex_shader = vertex_shader.and_then(cstring_from_str);
        unsafe {
            ffi::scn_program_set_vertex_shader(
                self.ptr,
                vertex_shader
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
    }

    /// Mirrors `SCNProgram.fragmentShader`.
    #[must_use]
    pub fn fragment_shader(&self) -> Option<String> {
        unsafe { take_string(ffi::scn_program_copy_fragment_shader(self.ptr)) }
    }

    /// Sets the `SCNProgram.fragmentShader` member.
    pub fn set_fragment_shader(&self, fragment_shader: Option<&str>) {
        let fragment_shader = fragment_shader.and_then(cstring_from_str);
        unsafe {
            ffi::scn_program_set_fragment_shader(
                self.ptr,
                fragment_shader
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
    }

    /// Mirrors `SCNProgram.geometryShader`.
    #[must_use]
    pub fn geometry_shader(&self) -> Option<String> {
        unsafe { take_string(ffi::scn_program_copy_geometry_shader(self.ptr)) }
    }

    /// Sets the `SCNProgram.geometryShader` member.
    pub fn set_geometry_shader(&self, geometry_shader: Option<&str>) {
        let geometry_shader = geometry_shader.and_then(cstring_from_str);
        unsafe {
            ffi::scn_program_set_geometry_shader(
                self.ptr,
                geometry_shader
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
    }

    /// Mirrors `SCNProgram.tessellationControlShader`.
    #[must_use]
    pub fn tessellation_control_shader(&self) -> Option<String> {
        unsafe { take_string(ffi::scn_program_copy_tessellation_control_shader(self.ptr)) }
    }

    /// Sets the `SCNProgram.tessellationControlShader` member.
    pub fn set_tessellation_control_shader(&self, tessellation_control_shader: Option<&str>) {
        let tessellation_control_shader = tessellation_control_shader.and_then(cstring_from_str);
        unsafe {
            ffi::scn_program_set_tessellation_control_shader(
                self.ptr,
                tessellation_control_shader
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
    }

    /// Mirrors `SCNProgram.tessellationEvaluationShader`.
    #[must_use]
    pub fn tessellation_evaluation_shader(&self) -> Option<String> {
        unsafe {
            take_string(ffi::scn_program_copy_tessellation_evaluation_shader(
                self.ptr,
            ))
        }
    }

    /// Sets the `SCNProgram.tessellationEvaluationShader` member.
    pub fn set_tessellation_evaluation_shader(&self, tessellation_evaluation_shader: Option<&str>) {
        let tessellation_evaluation_shader =
            tessellation_evaluation_shader.and_then(cstring_from_str);
        unsafe {
            ffi::scn_program_set_tessellation_evaluation_shader(
                self.ptr,
                tessellation_evaluation_shader
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
    }

    /// Mirrors `SCNProgram.vertexFunctionName`.
    #[must_use]
    pub fn vertex_function_name(&self) -> Option<String> {
        unsafe { take_string(ffi::scn_program_copy_vertex_function_name(self.ptr)) }
    }

    /// Sets the `SCNProgram.vertexFunctionName` member.
    pub fn set_vertex_function_name(&self, vertex_function_name: Option<&str>) {
        let vertex_function_name = vertex_function_name.and_then(cstring_from_str);
        unsafe {
            ffi::scn_program_set_vertex_function_name(
                self.ptr,
                vertex_function_name
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
    }

    /// Mirrors `SCNProgram.fragmentFunctionName`.
    #[must_use]
    pub fn fragment_function_name(&self) -> Option<String> {
        unsafe { take_string(ffi::scn_program_copy_fragment_function_name(self.ptr)) }
    }

    /// Sets the `SCNProgram.fragmentFunctionName` member.
    pub fn set_fragment_function_name(&self, fragment_function_name: Option<&str>) {
        let fragment_function_name = fragment_function_name.and_then(cstring_from_str);
        unsafe {
            ffi::scn_program_set_fragment_function_name(
                self.ptr,
                fragment_function_name
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
    }

    /// Mirrors `SCNProgram.opaque`.
    #[must_use]
    pub fn opaque(&self) -> bool {
        unsafe { ffi::scn_program_get_opaque(self.ptr) }
    }

    /// Sets the `SCNProgram.opaque` member.
    pub fn set_opaque(&self, opaque: bool) {
        unsafe { ffi::scn_program_set_opaque(self.ptr, opaque) };
    }

    /// Sets the `SCNProgram.semantic` member.
    pub fn set_semantic(
        &self,
        semantic: Option<&str>,
        symbol: &str,
        mapping_channel: Option<isize>,
    ) {
        let Some(symbol) = cstring_from_str(symbol) else {
            return;
        };
        let semantic = semantic.and_then(cstring_from_str);
        unsafe {
            ffi::scn_program_set_semantic(
                self.ptr,
                semantic
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                symbol.as_ptr(),
                mapping_channel.unwrap_or_default(),
                mapping_channel.is_some(),
            );
        };
    }

    /// Mirrors `SCNProgram.semanticForSymbol`.
    #[must_use]
    pub fn semantic_for_symbol(&self, symbol: &str) -> Option<String> {
        let symbol = cstring_from_str(symbol)?;
        unsafe {
            take_string(ffi::scn_program_copy_semantic_for_symbol(
                self.ptr,
                symbol.as_ptr(),
            ))
        }
    }

    /// Sets the `SCNProgram.delegate` member.
    pub fn set_delegate(&self, delegate: Option<&ProgramDelegate>) {
        unsafe {
            ffi::scn_program_set_delegate(
                self.ptr,
                delegate.map_or(ptr::null_mut(), ProgramDelegate::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn delegate(&self) -> Option<ProgramDelegate> {
        DelegateObject::from_retained(unsafe { ffi::scn_program_get_delegate(self.ptr) })
            .map(|inner| ProgramDelegate { inner })
    }

    pub fn set_library(&self, library: Option<&MetalLibrary>) {
        unsafe {
            ffi::scn_program_set_library(
                self.ptr,
                library.map_or(ptr::null_mut(), MetalLibrary::as_ptr),
            );
        };
    }

    /// Sets the `SCNProgram.bufferBinding` member.
    pub fn set_buffer_binding(
        &self,
        name: &str,
        frequency: BufferFrequency,
        binding: Option<&ProgramBufferBinding>,
    ) {
        let Some(name) = cstring_from_str(name) else {
            return;
        };
        unsafe {
            ffi::scn_program_set_buffer_binding(
                self.ptr,
                name.as_ptr(),
                frequency as i32,
                binding.map_or(ptr::null_mut(), ProgramBufferBinding::as_ptr),
            );
        };
    }
}
