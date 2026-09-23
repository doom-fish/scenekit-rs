/// Three `f32` components exchanged with SceneKit's `SCNVector3`.
///
/// On macOS `SCNVector3` stores `CGFloat` (64-bit) components, so this is not its
/// memory layout: the bridge converts each component, rounding to `f32` on the way out.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector3 {
    /// X component, converted to and from the `CGFloat` in `SCNVector3`.
    pub x: f32,
    /// Y component, converted to and from the `CGFloat` in `SCNVector3`.
    pub y: f32,
    /// Z component, converted to and from the `CGFloat` in `SCNVector3`.
    pub z: f32,
}

impl Vector3 {
    /// Creates a wrapped `SCNVector3` instance.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns the zero-valued `SCNVector3` constant.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Returns a pointer to the first of the three `f32` components.
    #[must_use]
    pub const fn as_ptr(&self) -> *const f32 {
        &raw const self.x
    }

    /// Returns a mutable pointer to the first of the three `f32` components.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        &raw mut self.x
    }
}

/// Four `f32` components exchanged with SceneKit's `CGFloat`-based `SCNVector4`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector4 {
    /// X component, converted to and from the `CGFloat` in `SCNVector4`.
    pub x: f32,
    /// Y component, converted to and from the `CGFloat` in `SCNVector4`.
    pub y: f32,
    /// Z component, converted to and from the `CGFloat` in `SCNVector4`.
    pub z: f32,
    /// W component, converted to and from the `CGFloat` in `SCNVector4`.
    pub w: f32,
}

impl Vector4 {
    /// Creates a wrapped `SCNVector4` instance.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Returns a pointer to the first of the four `f32` components.
    #[must_use]
    pub const fn as_ptr(&self) -> *const f32 {
        &raw const self.x
    }

    /// Returns a mutable pointer to the first of the four `f32` components.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        &raw mut self.x
    }
}

/// Sixteen `f32` elements exchanged with SceneKit's `CGFloat`-based `SCNMatrix4`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    /// Elements in `SCNMatrix4` order (`m11`, `m12`, … `m44`), each converted from `CGFloat`.
    pub elements: [f32; 16],
}

impl Matrix4 {
    /// Returns the `SCNMatrix4Identity` transform.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            elements: [
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    /// Builds a `SCNMatrix4` value from raw matrix elements.
    #[must_use]
    pub const fn from_elements(elements: [f32; 16]) -> Self {
        Self { elements }
    }

    /// Returns a pointer to the first of the sixteen `f32` elements.
    #[must_use]
    pub const fn as_ptr(&self) -> *const f32 {
        self.elements.as_ptr()
    }

    /// Returns a mutable pointer to the first of the sixteen `f32` elements.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self.elements.as_mut_ptr()
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self::identity()
    }
}

// MARK: - ABI Layout Assertions
//
// `Vector3`, `Vector4` and `Matrix4` are `#[repr(C)]` and are marshalled across
// the Rust <-> Swift `@_cdecl` FFI boundary as contiguous `f32` buffers (the
// Swift bridge in `swift-bridge/Sources/SceneKitBridge/Core.swift` reads/writes
// them via `assumingMemoryBound(to: Float.self)`). These compile-time
// assertions pin the exact size and alignment shared with Swift so any
// accidental field reordering / type change fails the build immediately instead
// of silently corrupting marshalled data at runtime. The cross-language
// `scn_verify_ffi_layout` check in `tests/ffi_layout_tests.rs` guards the Swift
// side's `f32` element size too.
use core::mem::{align_of, size_of};

const _: () = assert!(size_of::<Vector3>() == 12);
const _: () = assert!(align_of::<Vector3>() == 4);

const _: () = assert!(size_of::<Vector4>() == 16);
const _: () = assert!(align_of::<Vector4>() == 4);

const _: () = assert!(size_of::<Matrix4>() == 64);
const _: () = assert!(align_of::<Matrix4>() == 4);
