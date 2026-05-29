/// Represents RGBA color components consumed by SceneKit APIs.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red channel consumed by SceneKit color APIs.
    pub r: f32,
    /// Green channel consumed by SceneKit color APIs.
    pub g: f32,
    /// Blue channel consumed by SceneKit color APIs.
    pub b: f32,
    /// Alpha channel consumed by SceneKit color APIs.
    pub a: f32,
}

impl Color {
    /// Builds a color value consumed by SceneKit RGBA APIs.
    #[must_use]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Builds an opaque color value consumed by SceneKit RGB APIs.
    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Returns the `black` color commonly used with SceneKit materials and views.
    #[must_use]
    pub const fn black() -> Self {
        Self::rgb(0.0, 0.0, 0.0)
    }

    /// Returns the `white` color commonly used with SceneKit materials and views.
    #[must_use]
    pub const fn white() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }

    /// Returns the `red` color commonly used with SceneKit materials and views.
    #[must_use]
    pub const fn red() -> Self {
        Self::rgb(1.0, 0.0, 0.0)
    }

    /// Returns the `green` color commonly used with SceneKit materials and views.
    #[must_use]
    pub const fn green() -> Self {
        Self::rgb(0.0, 1.0, 0.0)
    }

    /// Returns the `blue` color commonly used with SceneKit materials and views.
    #[must_use]
    pub const fn blue() -> Self {
        Self::rgb(0.0, 0.0, 1.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

// MARK: - ABI Layout Assertions
//
// `Color` is `#[repr(C)]` and is marshalled across the Rust <-> Swift `@_cdecl`
// FFI boundary as a contiguous 4 x `f32` RGBA buffer (the Swift bridge in
// `swift-bridge/Sources/SceneKitBridge/Core.swift` reads/writes it via
// `assumingMemoryBound(to: Float.self)`). These compile-time assertions pin the
// exact size and alignment shared with Swift so any accidental field reordering
// / type change fails the build immediately instead of silently corrupting
// marshalled data at runtime. The cross-language `scn_verify_ffi_layout` check
// in `tests/ffi_layout_tests.rs` guards the Swift side's `f32` element size too.
// (`offset_of!` is intentionally not used: the crate MSRV is 1.76, below the
// 1.77 that stabilized it.)
use core::mem::{align_of, size_of};

const _: () = assert!(size_of::<Color>() == 16);
const _: () = assert!(align_of::<Color>() == 4);
