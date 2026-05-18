use core::ffi::c_char;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use crate::error::take_string;
use crate::math::{Matrix4, Vector3, Vector4};
use crate::private::{cstring_from_str, lookup_string_constant};

extern "C" {
    fn scn_constant_lookup_extra(name: *const c_char) -> *mut c_char;
    fn scn_extra_vector3_equal(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> bool;
    fn scn_extra_vector4_equal(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> bool;
    fn scn_extra_matrix4_equal(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> bool;
    fn scn_extra_matrix4_is_identity(matrix: *mut core::ffi::c_void) -> bool;
    fn scn_extra_matrix4_make_rotation(
        angle: f32,
        x: f32,
        y: f32,
        z: f32,
        out_matrix: *mut core::ffi::c_void,
    );
    fn scn_extra_matrix4_scale(
        matrix: *mut core::ffi::c_void,
        sx: f32,
        sy: f32,
        sz: f32,
        out_matrix: *mut core::ffi::c_void,
    );
    fn scn_extra_matrix4_rotate(
        matrix: *mut core::ffi::c_void,
        angle: f32,
        x: f32,
        y: f32,
        z: f32,
        out_matrix: *mut core::ffi::c_void,
    );
    fn scn_extra_matrix4_invert(matrix: *mut core::ffi::c_void, out_matrix: *mut core::ffi::c_void);
    fn scn_extra_matrix4_mult(
        a: *mut core::ffi::c_void,
        b: *mut core::ffi::c_void,
        out_matrix: *mut core::ffi::c_void,
    );
    fn scn_extra_matrix4_to_glk(matrix: *mut core::ffi::c_void, out_elements: *mut f32);
    fn scn_extra_matrix4_from_glk(elements: *const f32, out_matrix: *mut core::ffi::c_void);
}

fn lookup_extra_string_constant(symbol: &str) -> String {
    let c_string = cstring_from_str(symbol)
        .expect("SceneKit constant symbol names never contain interior NUL bytes");
    let extra = unsafe { take_string(scn_constant_lookup_extra(c_string.as_ptr())) };
    extra.unwrap_or_else(|| lookup_string_constant(symbol))
}

macro_rules! string_constant_fn {
    ($name:ident, $symbol:literal) => {
        #[doc = concat!("Returns the SceneKit constant `", $symbol, "`.")]
        #[must_use]
        pub fn $name() -> String {
            lookup_extra_string_constant($symbol)
        }
    };
}

macro_rules! option_set_type {
    ($(#[$meta:meta])* $name:ident { $($const_name:ident = $value:expr,)* }) => {
        $(#[$meta])*
        #[doc = concat!("Mirrors `SCN", stringify!($name), "`.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name(usize);

        impl $name {
            $(#[doc = concat!("Matches the `", stringify!($const_name), "` bit on this SceneKit option set.")]
            pub const $const_name: Self = Self($value);)*

            #[doc = "Returns the empty SceneKit option set value."]
            #[must_use]
            pub const fn empty() -> Self {
                Self(0)
            }

            #[doc = "Builds this SceneKit option set from raw bit values."]
            #[must_use]
            pub const fn from_bits(bits: usize) -> Self {
                Self(bits)
            }

            #[doc = "Returns the raw bit pattern stored by this SceneKit option set."]
            #[must_use]
            pub const fn bits(self) -> usize {
                self.0
            }

            #[doc = "Returns whether this SceneKit option set contains another flag set."]
            #[must_use]
            pub const fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }
        }

        impl BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }
    };
}

/// Mirrors `SCNActionTimingMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ActionTimingMode {
    /// Corresponds to the `SCNActionTimingMode::Linear` case.
    Linear = 0,
    /// Corresponds to the `SCNActionTimingMode::EaseIn` case.
    EaseIn = 1,
    /// Corresponds to the `SCNActionTimingMode::EaseOut` case.
    EaseOut = 2,
    /// Corresponds to the `SCNActionTimingMode::EaseInEaseOut` case.
    EaseInEaseOut = 3,
}

/// Mirrors `SCNBlendMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BlendMode {
    /// Corresponds to the `SCNBlendMode::Alpha` case.
    Alpha = 0,
    /// Corresponds to the `SCNBlendMode::Add` case.
    Add = 1,
    /// Corresponds to the `SCNBlendMode::Subtract` case.
    Subtract = 2,
    /// Corresponds to the `SCNBlendMode::Multiply` case.
    Multiply = 3,
    /// Corresponds to the `SCNBlendMode::Screen` case.
    Screen = 4,
    /// Corresponds to the `SCNBlendMode::Replace` case.
    Replace = 5,
    /// Corresponds to the `SCNBlendMode::Max` case.
    Max = 6,
}

/// Mirrors `SCNCameraProjectionDirection`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CameraProjectionDirection {
    /// Corresponds to the `SCNCameraProjectionDirection::Vertical` case.
    Vertical = 0,
    /// Corresponds to the `SCNCameraProjectionDirection::Horizontal` case.
    Horizontal = 1,
}

/// Mirrors `SCNChamferMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChamferMode {
    /// Corresponds to the `SCNChamferMode::Both` case.
    Both = 0,
    /// Corresponds to the `SCNChamferMode::Front` case.
    Front = 1,
    /// Corresponds to the `SCNChamferMode::Back` case.
    Back = 2,
}

option_set_type!(ColorMask {
    NONE = 0,
    RED = 0x1 << 3,
    GREEN = 0x1 << 2,
    BLUE = 0x1 << 1,
    ALPHA = 0x1 << 0,
    ALL = 0xF,
});

/// Mirrors `SCNCullMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CullMode {
    /// Corresponds to the `SCNCullMode::Back` case.
    Back = 0,
    /// Corresponds to the `SCNCullMode::Front` case.
    Front = 1,
}

/// Mirrors `SCNFillMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FillMode {
    /// Corresponds to the `SCNFillMode::Fill` case.
    Fill = 0,
    /// Corresponds to the `SCNFillMode::Lines` case.
    Lines = 1,
}

/// Mirrors `SCNFilterMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FilterMode {
    /// Corresponds to the `SCNFilterMode::None` case.
    None = 0,
    /// Corresponds to the `SCNFilterMode::Nearest` case.
    Nearest = 1,
    /// Corresponds to the `SCNFilterMode::Linear` case.
    Linear = 2,
}

/// Mirrors `SCNGeometryPrimitiveType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GeometryPrimitiveType {
    /// Corresponds to the `SCNGeometryPrimitiveType::Triangles` case.
    Triangles = 0,
    /// Corresponds to the `SCNGeometryPrimitiveType::TriangleStrip` case.
    TriangleStrip = 1,
    /// Corresponds to the `SCNGeometryPrimitiveType::Line` case.
    Line = 2,
    /// Corresponds to the `SCNGeometryPrimitiveType::Point` case.
    Point = 3,
    /// Corresponds to the `SCNGeometryPrimitiveType::Polygon` case.
    Polygon = 4,
}

/// Mirrors `SCNHitTestSearchMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HitTestSearchMode {
    /// Corresponds to the `SCNHitTestSearchMode::Closest` case.
    Closest = 0,
    /// Corresponds to the `SCNHitTestSearchMode::All` case.
    All = 1,
    /// Corresponds to the `SCNHitTestSearchMode::Any` case.
    Any = 2,
}

/// Mirrors `SCNLightAreaType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LightAreaType {
    /// Corresponds to the `SCNLightAreaType::Rectangle` case.
    Rectangle = 1,
    /// Corresponds to the `SCNLightAreaType::Polygon` case.
    Polygon = 4,
}

/// Mirrors `SCNLightProbeType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LightProbeType {
    /// Corresponds to the `SCNLightProbeType::Irradiance` case.
    Irradiance = 0,
    /// Corresponds to the `SCNLightProbeType::Radiance` case.
    Radiance = 1,
}

/// Mirrors `SCNLightProbeUpdateType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LightProbeUpdateType {
    /// Corresponds to the `SCNLightProbeUpdateType::Never` case.
    Never = 0,
    /// Corresponds to the `SCNLightProbeUpdateType::Realtime` case.
    Realtime = 1,
}

/// Mirrors `SCNMorpherCalculationMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MorpherCalculationMode {
    /// Corresponds to the `SCNMorpherCalculationMode::Normalized` case.
    Normalized = 0,
    /// Corresponds to the `SCNMorpherCalculationMode::Additive` case.
    Additive = 1,
}

/// Mirrors `SCNMovabilityHint`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MovabilityHint {
    /// Corresponds to the `SCNMovabilityHint::Fixed` case.
    Fixed = 0,
    /// Corresponds to the `SCNMovabilityHint::Movable` case.
    Movable = 1,
}

/// Mirrors `SCNNodeFocusBehavior`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NodeFocusBehavior {
    /// Corresponds to the `SCNNodeFocusBehavior::None` case.
    None = 0,
    /// Corresponds to the `SCNNodeFocusBehavior::Occluding` case.
    Occluding = 1,
    /// Corresponds to the `SCNNodeFocusBehavior::Focusable` case.
    Focusable = 2,
}

/// Mirrors `SCNParticleBirthDirection`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleBirthDirection {
    /// Corresponds to the `SCNParticleBirthDirection::Constant` case.
    Constant = 0,
    /// Corresponds to the `SCNParticleBirthDirection::SurfaceNormal` case.
    SurfaceNormal = 1,
    /// Corresponds to the `SCNParticleBirthDirection::Random` case.
    Random = 2,
}

/// Mirrors `SCNParticleBirthLocation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleBirthLocation {
    /// Corresponds to the `SCNParticleBirthLocation::Surface` case.
    Surface = 0,
    /// Corresponds to the `SCNParticleBirthLocation::Volume` case.
    Volume = 1,
    /// Corresponds to the `SCNParticleBirthLocation::Vertex` case.
    Vertex = 2,
}

/// Mirrors `SCNParticleBlendMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleBlendMode {
    /// Corresponds to the `SCNParticleBlendMode::Additive` case.
    Additive = 0,
    /// Corresponds to the `SCNParticleBlendMode::Subtract` case.
    Subtract = 1,
    /// Corresponds to the `SCNParticleBlendMode::Multiply` case.
    Multiply = 2,
    /// Corresponds to the `SCNParticleBlendMode::Screen` case.
    Screen = 3,
    /// Corresponds to the `SCNParticleBlendMode::Alpha` case.
    Alpha = 4,
    /// Corresponds to the `SCNParticleBlendMode::Replace` case.
    Replace = 5,
}

/// Mirrors `SCNParticleEvent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleEvent {
    /// Corresponds to the `SCNParticleEvent::Birth` case.
    Birth = 0,
    /// Corresponds to the `SCNParticleEvent::Death` case.
    Death = 1,
    /// Corresponds to the `SCNParticleEvent::Collision` case.
    Collision = 2,
}

/// Mirrors `SCNParticleImageSequenceAnimationMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleImageSequenceAnimationMode {
    /// Corresponds to the `SCNParticleImageSequenceAnimationMode::Repeat` case.
    Repeat = 0,
    /// Corresponds to the `SCNParticleImageSequenceAnimationMode::Clamp` case.
    Clamp = 1,
    /// Corresponds to the `SCNParticleImageSequenceAnimationMode::AutoReverse` case.
    AutoReverse = 2,
}

/// Mirrors `SCNParticleInputMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleInputMode {
    /// Corresponds to the `SCNParticleInputMode::OverLife` case.
    OverLife = 0,
    /// Corresponds to the `SCNParticleInputMode::OverDistance` case.
    OverDistance = 1,
    /// Corresponds to the `SCNParticleInputMode::OverOtherProperty` case.
    OverOtherProperty = 2,
}

/// Mirrors `SCNParticleModifierStage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleModifierStage {
    /// Corresponds to the `SCNParticleModifierStage::PreDynamics` case.
    PreDynamics = 0,
    /// Corresponds to the `SCNParticleModifierStage::PostDynamics` case.
    PostDynamics = 1,
    /// Corresponds to the `SCNParticleModifierStage::PreCollision` case.
    PreCollision = 2,
    /// Corresponds to the `SCNParticleModifierStage::PostCollision` case.
    PostCollision = 3,
}

/// Mirrors `SCNParticleOrientationMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleOrientationMode {
    /// Corresponds to the `SCNParticleOrientationMode::BillboardScreenAligned` case.
    BillboardScreenAligned = 0,
    /// Corresponds to the `SCNParticleOrientationMode::BillboardViewAligned` case.
    BillboardViewAligned = 1,
    /// Corresponds to the `SCNParticleOrientationMode::Free` case.
    Free = 2,
    /// Corresponds to the `SCNParticleOrientationMode::BillboardYAligned` case.
    BillboardYAligned = 3,
}

/// Mirrors `SCNParticleSortingMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleSortingMode {
    /// Corresponds to the `SCNParticleSortingMode::None` case.
    None = 0,
    /// Corresponds to the `SCNParticleSortingMode::ProjectedDepth` case.
    ProjectedDepth = 1,
    /// Corresponds to the `SCNParticleSortingMode::Distance` case.
    Distance = 2,
    /// Corresponds to the `SCNParticleSortingMode::OldestFirst` case.
    OldestFirst = 3,
    /// Corresponds to the `SCNParticleSortingMode::YoungestFirst` case.
    YoungestFirst = 4,
}

option_set_type!(PhysicsCollisionCategory {
    DEFAULT = 1 << 0,
    STATIC = 1 << 1,
    ALL = usize::MAX,
});

/// Mirrors `SCNPhysicsFieldScope`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PhysicsFieldScope {
    /// Corresponds to the `SCNPhysicsFieldScope::InsideExtent` case.
    InsideExtent = 0,
    /// Corresponds to the `SCNPhysicsFieldScope::OutsideExtent` case.
    OutsideExtent = 1,
}

/// Mirrors `SCNReferenceLoadingPolicy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReferenceLoadingPolicy {
    /// Corresponds to the `SCNReferenceLoadingPolicy::Immediate` case.
    Immediate = 0,
    /// Corresponds to the `SCNReferenceLoadingPolicy::OnDemand` case.
    OnDemand = 1,
}

/// Mirrors `SCNTessellationSmoothingMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TessellationSmoothingMode {
    /// Corresponds to the `SCNTessellationSmoothingMode::None` case.
    None = 0,
    /// Corresponds to the `SCNTessellationSmoothingMode::PnTriangles` case.
    PnTriangles = 1,
    /// Corresponds to the `SCNTessellationSmoothingMode::Phong` case.
    Phong = 2,
}

/// Mirrors `SCNTransparencyMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TransparencyMode {
    /// Corresponds to the `SCNTransparencyMode::AOne` case.
    AOne = 0,
    /// Corresponds to the `SCNTransparencyMode::RgbZero` case.
    RgbZero = 1,
    /// Corresponds to the `SCNTransparencyMode::SingleLayer` case.
    SingleLayer = 2,
    /// Corresponds to the `SCNTransparencyMode::DualLayer` case.
    DualLayer = 3,
}

impl TransparencyMode {
    /// Matches SceneKit's default `SCNTransparencyMode` value.
    pub const DEFAULT: Self = Self::AOne;
}

/// Mirrors `SCNWrapMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WrapMode {
    /// Corresponds to the `SCNWrapMode::Clamp` case.
    Clamp = 1,
    /// Corresponds to the `SCNWrapMode::Repeat` case.
    Repeat = 2,
    /// Corresponds to the `SCNWrapMode::ClampToBorder` case.
    ClampToBorder = 3,
    /// Corresponds to the `SCNWrapMode::Mirror` case.
    Mirror = 4,
}

option_set_type!(BillboardAxis {
    X = 1 << 0,
    Y = 1 << 1,
    Z = 1 << 2,
    ALL = (1 << 0) | (1 << 1) | (1 << 2),
});

/// Represents a `GLKMatrix4` value used alongside SceneKit transforms.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GlkMatrix4 {
    /// Column-major elements matching the `GLKMatrix4` memory layout.
    pub elements: [f32; 16],
}

/// Matches `SCNVector3Zero`.
pub const VECTOR3_ZERO: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
/// Matches the zero-valued `SCNVector4` constant.
pub const VECTOR4_ZERO: Vector4 = Vector4 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};
/// Matches `SCNMatrix4Identity`.
pub const MATRIX4_IDENTITY: Matrix4 = Matrix4 {
    elements: [
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    ],
};

/// Mirrors `SCNVector3EqualToVector3`.
#[must_use]
pub fn vector3_equal(a: Vector3, b: Vector3) -> bool {
    let mut a = a;
    let mut b = b;
    unsafe { scn_extra_vector3_equal(a.as_mut_ptr().cast(), b.as_mut_ptr().cast()) }
}

/// Mirrors `SCNVector4EqualToVector4`.
#[must_use]
pub fn vector4_equal(a: Vector4, b: Vector4) -> bool {
    let mut a = a;
    let mut b = b;
    unsafe { scn_extra_vector4_equal(a.as_mut_ptr().cast(), b.as_mut_ptr().cast()) }
}

/// Mirrors `SCNMatrix4EqualToMatrix4`.
#[must_use]
pub fn matrix4_equal(a: Matrix4, b: Matrix4) -> bool {
    let mut a = a;
    let mut b = b;
    unsafe { scn_extra_matrix4_equal(a.as_mut_ptr().cast(), b.as_mut_ptr().cast()) }
}

/// Mirrors `SCNMatrix4IsIdentity`.
#[must_use]
pub fn matrix4_is_identity(matrix: Matrix4) -> bool {
    let mut matrix = matrix;
    unsafe { scn_extra_matrix4_is_identity(matrix.as_mut_ptr().cast()) }
}

/// Mirrors `SCNMatrix4MakeRotation`.
#[must_use]
pub fn matrix4_make_rotation(angle: f32, x: f32, y: f32, z: f32) -> Matrix4 {
    let mut matrix = Matrix4::default();
    unsafe {
        scn_extra_matrix4_make_rotation(angle, x, y, z, matrix.as_mut_ptr().cast());
    }
    matrix
}

/// Mirrors `SCNMatrix4Scale`.
#[must_use]
pub fn matrix4_scale(matrix: Matrix4, sx: f32, sy: f32, sz: f32) -> Matrix4 {
    let mut matrix = matrix;
    let mut out = Matrix4::default();
    unsafe {
        scn_extra_matrix4_scale(
            matrix.as_mut_ptr().cast(),
            sx,
            sy,
            sz,
            out.as_mut_ptr().cast(),
        );
    }
    out
}

/// Mirrors `SCNMatrix4Rotate`.
#[must_use]
pub fn matrix4_rotate(matrix: Matrix4, angle: f32, x: f32, y: f32, z: f32) -> Matrix4 {
    let mut matrix = matrix;
    let mut out = Matrix4::default();
    unsafe {
        scn_extra_matrix4_rotate(
            matrix.as_mut_ptr().cast(),
            angle,
            x,
            y,
            z,
            out.as_mut_ptr().cast(),
        );
    }
    out
}

/// Mirrors `SCNMatrix4Invert`.
#[must_use]
pub fn matrix4_invert(matrix: Matrix4) -> Matrix4 {
    let mut matrix = matrix;
    let mut out = Matrix4::default();
    unsafe { scn_extra_matrix4_invert(matrix.as_mut_ptr().cast(), out.as_mut_ptr().cast()) };
    out
}

/// Mirrors `SCNMatrix4Mult`.
#[must_use]
pub fn matrix4_mult(a: Matrix4, b: Matrix4) -> Matrix4 {
    let mut a = a;
    let mut b = b;
    let mut out = Matrix4::default();
    unsafe {
        scn_extra_matrix4_mult(
            a.as_mut_ptr().cast(),
            b.as_mut_ptr().cast(),
            out.as_mut_ptr().cast(),
        );
    };
    out
}

/// Mirrors `SCNMatrix4ToGLKMatrix4`.
#[must_use]
pub fn matrix4_to_glk(matrix: Matrix4) -> GlkMatrix4 {
    let mut matrix = matrix;
    let mut glk = GlkMatrix4::default();
    unsafe { scn_extra_matrix4_to_glk(matrix.as_mut_ptr().cast(), glk.elements.as_mut_ptr()) };
    glk
}

/// Mirrors `SCNGLKMatrix4ToSCNMatrix4`.
#[must_use]
pub fn matrix4_from_glk(glk: GlkMatrix4) -> Matrix4 {
    let mut matrix = Matrix4::default();
    unsafe { scn_extra_matrix4_from_glk(glk.elements.as_ptr(), matrix.as_mut_ptr().cast()) };
    matrix
}

string_constant_fn!(error_domain, "SCNErrorDomain");
string_constant_fn!(
    geometry_source_semantic_bone_indices,
    "SCNGeometrySourceSemanticBoneIndices"
);
string_constant_fn!(
    geometry_source_semantic_bone_weights,
    "SCNGeometrySourceSemanticBoneWeights"
);
string_constant_fn!(
    geometry_source_semantic_color,
    "SCNGeometrySourceSemanticColor"
);
string_constant_fn!(
    geometry_source_semantic_edge_crease,
    "SCNGeometrySourceSemanticEdgeCrease"
);
string_constant_fn!(
    geometry_source_semantic_normal,
    "SCNGeometrySourceSemanticNormal"
);
string_constant_fn!(
    geometry_source_semantic_tangent,
    "SCNGeometrySourceSemanticTangent"
);
string_constant_fn!(
    geometry_source_semantic_texcoord,
    "SCNGeometrySourceSemanticTexcoord"
);
string_constant_fn!(
    geometry_source_semantic_vertex,
    "SCNGeometrySourceSemanticVertex"
);
string_constant_fn!(
    geometry_source_semantic_vertex_crease,
    "SCNGeometrySourceSemanticVertexCrease"
);
string_constant_fn!(
    hit_test_back_face_culling_key,
    "SCNHitTestBackFaceCullingKey"
);
string_constant_fn!(
    hit_test_bounding_box_only_key,
    "SCNHitTestBoundingBoxOnlyKey"
);
string_constant_fn!(hit_test_clip_to_z_range_key, "SCNHitTestClipToZRangeKey");
string_constant_fn!(hit_test_first_found_only_key, "SCNHitTestFirstFoundOnlyKey");
string_constant_fn!(
    hit_test_ignore_child_nodes_key,
    "SCNHitTestIgnoreChildNodesKey"
);
string_constant_fn!(
    hit_test_ignore_hidden_nodes_key,
    "SCNHitTestIgnoreHiddenNodesKey"
);
string_constant_fn!(
    hit_test_option_category_bit_mask,
    "SCNHitTestOptionCategoryBitMask"
);
string_constant_fn!(
    hit_test_option_ignore_light_area,
    "SCNHitTestOptionIgnoreLightArea"
);
string_constant_fn!(hit_test_option_search_mode, "SCNHitTestOptionSearchMode");
string_constant_fn!(hit_test_root_node_key, "SCNHitTestRootNodeKey");
string_constant_fn!(hit_test_sort_results_key, "SCNHitTestSortResultsKey");
string_constant_fn!(lighting_model_blinn, "SCNLightingModelBlinn");
string_constant_fn!(lighting_model_constant, "SCNLightingModelConstant");
string_constant_fn!(lighting_model_lambert, "SCNLightingModelLambert");
string_constant_fn!(lighting_model_phong, "SCNLightingModelPhong");
string_constant_fn!(
    lighting_model_physically_based,
    "SCNLightingModelPhysicallyBased"
);
string_constant_fn!(lighting_model_shadow_only, "SCNLightingModelShadowOnly");
string_constant_fn!(model_transform, "SCNModelTransform");
string_constant_fn!(
    model_view_projection_transform,
    "SCNModelViewProjectionTransform"
);
string_constant_fn!(model_view_transform, "SCNModelViewTransform");
string_constant_fn!(normal_transform, "SCNNormalTransform");
string_constant_fn!(projection_transform, "SCNProjectionTransform");
string_constant_fn!(view_transform, "SCNViewTransform");
string_constant_fn!(particle_property_angle, "SCNParticlePropertyAngle");
string_constant_fn!(
    particle_property_angular_velocity,
    "SCNParticlePropertyAngularVelocity"
);
string_constant_fn!(particle_property_bounce, "SCNParticlePropertyBounce");
string_constant_fn!(particle_property_charge, "SCNParticlePropertyCharge");
string_constant_fn!(particle_property_color, "SCNParticlePropertyColor");
string_constant_fn!(
    particle_property_contact_normal,
    "SCNParticlePropertyContactNormal"
);
string_constant_fn!(
    particle_property_contact_point,
    "SCNParticlePropertyContactPoint"
);
string_constant_fn!(particle_property_frame, "SCNParticlePropertyFrame");
string_constant_fn!(particle_property_frame_rate, "SCNParticlePropertyFrameRate");
string_constant_fn!(particle_property_friction, "SCNParticlePropertyFriction");
string_constant_fn!(particle_property_life, "SCNParticlePropertyLife");
string_constant_fn!(particle_property_opacity, "SCNParticlePropertyOpacity");
string_constant_fn!(particle_property_position, "SCNParticlePropertyPosition");
string_constant_fn!(
    particle_property_rotation_axis,
    "SCNParticlePropertyRotationAxis"
);
string_constant_fn!(particle_property_size, "SCNParticlePropertySize");
string_constant_fn!(particle_property_velocity, "SCNParticlePropertyVelocity");
string_constant_fn!(
    physics_shape_keep_as_compound_key,
    "SCNPhysicsShapeKeepAsCompoundKey"
);
string_constant_fn!(
    physics_shape_option_collision_margin,
    "SCNPhysicsShapeOptionCollisionMargin"
);
string_constant_fn!(physics_shape_scale_key, "SCNPhysicsShapeScaleKey");
string_constant_fn!(
    physics_shape_type_bounding_box,
    "SCNPhysicsShapeTypeBoundingBox"
);
string_constant_fn!(
    physics_shape_type_concave_polyhedron,
    "SCNPhysicsShapeTypeConcavePolyhedron"
);
string_constant_fn!(
    physics_shape_type_convex_hull,
    "SCNPhysicsShapeTypeConvexHull"
);
string_constant_fn!(physics_shape_type_key, "SCNPhysicsShapeTypeKey");
string_constant_fn!(prefer_low_power_device_key, "SCNPreferLowPowerDeviceKey");
string_constant_fn!(preferred_device_key, "SCNPreferredDeviceKey");
string_constant_fn!(preferred_rendering_api_key, "SCNPreferredRenderingAPIKey");
string_constant_fn!(scene_end_time_attribute_key, "SCNSceneEndTimeAttributeKey");
string_constant_fn!(scene_export_destination_url, "SCNSceneExportDestinationURL");
string_constant_fn!(
    scene_frame_rate_attribute_key,
    "SCNSceneFrameRateAttributeKey"
);
string_constant_fn!(
    scene_start_time_attribute_key,
    "SCNSceneStartTimeAttributeKey"
);
string_constant_fn!(scene_up_axis_attribute_key, "SCNSceneUpAxisAttributeKey");
