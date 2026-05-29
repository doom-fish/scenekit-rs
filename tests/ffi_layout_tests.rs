//! ABI layout assertions for the `#[repr(C)]` math/color types shared with the
//! Swift bridge.
//!
//! `Vector3`, `Vector4`, `Matrix4` and `Color` are marshalled across the Rust
//! <-> Swift `@_cdecl` FFI boundary as contiguous `f32` buffers. If their size
//! or alignment ever drifts from what the Swift side expects, the data
//! marshalling silently corrupts. These tests pin the layout so accidental
//! field reordering / type changes are caught at `cargo test` time rather than
//! as runtime garbage.

use std::mem::{align_of, size_of};

use scenekit::{ffi, Color, Matrix4, Vector3, Vector4};

#[test]
fn vector3_layout() {
    // 3 x f32
    assert_eq!(size_of::<Vector3>(), 12, "Vector3 size drifted");
    assert_eq!(align_of::<Vector3>(), 4, "Vector3 alignment drifted");
}

#[test]
fn vector4_layout() {
    // 4 x f32
    assert_eq!(size_of::<Vector4>(), 16, "Vector4 size drifted");
    assert_eq!(align_of::<Vector4>(), 4, "Vector4 alignment drifted");
}

#[test]
fn matrix4_layout() {
    // [f32; 16]
    assert_eq!(size_of::<Matrix4>(), 64, "Matrix4 size drifted");
    assert_eq!(align_of::<Matrix4>(), 4, "Matrix4 alignment drifted");
}

#[test]
fn color_layout() {
    // 4 x f32 (RGBA)
    assert_eq!(size_of::<Color>(), 16, "Color size drifted");
    assert_eq!(align_of::<Color>(), 4, "Color alignment drifted");
}

/// Cross-language ABI check: asks the Swift bridge to verify that *its* `Float`
/// `MemoryLayout` (size/stride/alignment) matches the 4-byte `f32` element the
/// Rust side marshals these structs as. A `false` return means the Rust and
/// Swift layouts genuinely disagree, which is a real ABI bug.
#[test]
fn ffi_layout_matches_swift() {
    // SAFETY: `scn_verify_ffi_layout` takes no arguments and only reads
    // compile-time `MemoryLayout` constants in the Swift bridge.
    let matches = unsafe { ffi::scn_verify_ffi_layout() };
    assert!(
        matches,
        "Swift FFI element layout disagrees with Rust layout (ABI mismatch)"
    );
}
