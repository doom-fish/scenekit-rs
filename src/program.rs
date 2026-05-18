use core::ffi::{c_char, c_void};
use core::ptr;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::error::{take_string, SceneKitError};
use crate::ffi;
use crate::geometry::Geometry;
use crate::material::Material;
use crate::private::{cstring_from_str, handle_type, lookup_string_constant, Sealed};

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

type ProgramErrorCallback = Box<dyn FnMut(SceneKitError)>;
type BufferBindingCallback = Box<dyn FnMut(&BufferStream)>;

struct ProgramDelegateState {
    handle_error: ProgramErrorCallback,
}

struct ProgramBufferBindingState {
    callback: BufferBindingCallback,
}

/// Wraps `SCNProgramDelegate`.
pub struct ProgramDelegate {
    ptr: *mut c_void,
}

/// Wraps `SCNBufferBindingBlock`.
pub struct ProgramBufferBinding {
    ptr: *mut c_void,
}

impl core::fmt::Debug for ProgramDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ProgramDelegate")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl core::fmt::Debug for ProgramBufferBinding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ProgramBufferBinding")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl Drop for ProgramDelegate {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::scn_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Drop for ProgramBufferBinding {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::scn_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

unsafe fn program_delegate_state_from_context<'a>(
    context: *mut c_void,
) -> &'a mut ProgramDelegateState {
    &mut *context.cast::<ProgramDelegateState>()
}

unsafe fn program_buffer_binding_state_from_context<'a>(
    context: *mut c_void,
) -> &'a mut ProgramBufferBindingState {
    &mut *context.cast::<ProgramBufferBindingState>()
}

extern "C" fn release_program_delegate_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(context.cast::<ProgramDelegateState>()));
    }
}

extern "C" fn release_program_buffer_binding_context(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(context.cast::<ProgramBufferBindingState>()));
    }
}

extern "C" fn program_delegate_handle_error_trampoline(context: *mut c_void, message: *mut c_char) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if context.is_null() {
            if !message.is_null() {
                let _ = unsafe { take_string(message) };
            }
            return;
        }

        let state = unsafe { program_delegate_state_from_context(context) };
        let message = unsafe { take_string(message) }.unwrap_or_else(|| {
            "SCNProgramDelegate.handleError invoked without a message".to_owned()
        });
        (state.handle_error)(SceneKitError::new(message));
    }));
}

extern "C" fn program_buffer_binding_trampoline(context: *mut c_void, buffer_stream: *mut c_void) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if context.is_null() || buffer_stream.is_null() {
            return;
        }
        let state = unsafe { program_buffer_binding_state_from_context(context) };
        let buffer_stream = unsafe { BufferStream::from_raw_borrowed(buffer_stream) };
        (state.callback)(&buffer_stream);
    }));
}

impl ProgramDelegate {
    /// Creates a wrapped `SCNProgramDelegate` instance.
    #[must_use]
    pub fn new<F>(callback: F) -> Option<Self>
    where
        F: FnMut(SceneKitError) + 'static,
    {
        let state = Box::new(ProgramDelegateState {
            handle_error: Box::new(callback),
        });
        let context = Box::into_raw(state).cast::<c_void>();
        let ptr = unsafe {
            ffi::scn_program_delegate_new(
                context,
                release_program_delegate_context,
                program_delegate_handle_error_trampoline,
            )
        };
        if ptr.is_null() {
            release_program_delegate_context(context);
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Returns the Objective-C pointer backing this `SCNProgramDelegate` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl ProgramBufferBinding {
    /// Creates a wrapped `SCNBufferBindingBlock` instance.
    #[must_use]
    pub fn new<F>(callback: F) -> Option<Self>
    where
        F: FnMut(&BufferStream) + 'static,
    {
        let state = Box::new(ProgramBufferBindingState {
            callback: Box::new(callback),
        });
        let context = Box::into_raw(state).cast::<c_void>();
        let ptr = unsafe {
            ffi::scn_program_buffer_binding_new(
                context,
                release_program_buffer_binding_context,
                program_buffer_binding_trampoline,
            )
        };
        if ptr.is_null() {
            release_program_buffer_binding_context(context);
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Returns the Objective-C pointer backing this `SCNBufferBindingBlock` wrapper.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
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
    /// Mirrors `SCNBufferStream.writeBytes`.
    pub fn write_bytes(&self, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }
        unsafe { ffi::scn_buffer_stream_write_bytes(self.ptr, bytes.as_ptr().cast(), bytes.len()) };
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
