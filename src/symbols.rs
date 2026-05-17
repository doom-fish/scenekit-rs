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
        #[must_use]
        pub fn $name() -> String {
            lookup_extra_string_constant($symbol)
        }
    };
}

macro_rules! option_set_type {
    ($name:ident { $($const_name:ident = $value:expr,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name(usize);

        impl $name {
            $(pub const $const_name: Self = Self($value);)*

            #[must_use]
            pub const fn empty() -> Self {
                Self(0)
            }

            #[must_use]
            pub const fn from_bits(bits: usize) -> Self {
                Self(bits)
            }

            #[must_use]
            pub const fn bits(self) -> usize {
                self.0
            }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ActionTimingMode {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BlendMode {
    Alpha = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Screen = 4,
    Replace = 5,
    Max = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CameraProjectionDirection {
    Vertical = 0,
    Horizontal = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChamferMode {
    Both = 0,
    Front = 1,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CullMode {
    Back = 0,
    Front = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FillMode {
    Fill = 0,
    Lines = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FilterMode {
    None = 0,
    Nearest = 1,
    Linear = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GeometryPrimitiveType {
    Triangles = 0,
    TriangleStrip = 1,
    Line = 2,
    Point = 3,
    Polygon = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HitTestSearchMode {
    Closest = 0,
    All = 1,
    Any = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LightAreaType {
    Rectangle = 1,
    Polygon = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LightProbeType {
    Irradiance = 0,
    Radiance = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LightProbeUpdateType {
    Never = 0,
    Realtime = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MorpherCalculationMode {
    Normalized = 0,
    Additive = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MovabilityHint {
    Fixed = 0,
    Movable = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NodeFocusBehavior {
    None = 0,
    Occluding = 1,
    Focusable = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleBirthDirection {
    Constant = 0,
    SurfaceNormal = 1,
    Random = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleBirthLocation {
    Surface = 0,
    Volume = 1,
    Vertex = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleBlendMode {
    Additive = 0,
    Subtract = 1,
    Multiply = 2,
    Screen = 3,
    Alpha = 4,
    Replace = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleEvent {
    Birth = 0,
    Death = 1,
    Collision = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleImageSequenceAnimationMode {
    Repeat = 0,
    Clamp = 1,
    AutoReverse = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleInputMode {
    OverLife = 0,
    OverDistance = 1,
    OverOtherProperty = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleModifierStage {
    PreDynamics = 0,
    PostDynamics = 1,
    PreCollision = 2,
    PostCollision = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleOrientationMode {
    BillboardScreenAligned = 0,
    BillboardViewAligned = 1,
    Free = 2,
    BillboardYAligned = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ParticleSortingMode {
    None = 0,
    ProjectedDepth = 1,
    Distance = 2,
    OldestFirst = 3,
    YoungestFirst = 4,
}

option_set_type!(PhysicsCollisionCategory {
    DEFAULT = 1 << 0,
    STATIC = 1 << 1,
    ALL = usize::MAX,
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PhysicsFieldScope {
    InsideExtent = 0,
    OutsideExtent = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReferenceLoadingPolicy {
    Immediate = 0,
    OnDemand = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TessellationSmoothingMode {
    None = 0,
    PnTriangles = 1,
    Phong = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TransparencyMode {
    AOne = 0,
    RgbZero = 1,
    SingleLayer = 2,
    DualLayer = 3,
}

impl TransparencyMode {
    pub const DEFAULT: Self = Self::AOne;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WrapMode {
    Clamp = 1,
    Repeat = 2,
    ClampToBorder = 3,
    Mirror = 4,
}

option_set_type!(BillboardAxis {
    X = 1 << 0,
    Y = 1 << 1,
    Z = 1 << 2,
    ALL = (1 << 0) | (1 << 1) | (1 << 2),
});

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GlkMatrix4 {
    pub elements: [f32; 16],
}

pub const VECTOR3_ZERO: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
pub const VECTOR4_ZERO: Vector4 = Vector4 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};
pub const MATRIX4_IDENTITY: Matrix4 = Matrix4 {
    elements: [
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    ],
};

#[must_use]
pub fn vector3_equal(a: Vector3, b: Vector3) -> bool {
    let mut a = a;
    let mut b = b;
    unsafe { scn_extra_vector3_equal(a.as_mut_ptr().cast(), b.as_mut_ptr().cast()) }
}

#[must_use]
pub fn vector4_equal(a: Vector4, b: Vector4) -> bool {
    let mut a = a;
    let mut b = b;
    unsafe { scn_extra_vector4_equal(a.as_mut_ptr().cast(), b.as_mut_ptr().cast()) }
}

#[must_use]
pub fn matrix4_equal(a: Matrix4, b: Matrix4) -> bool {
    let mut a = a;
    let mut b = b;
    unsafe { scn_extra_matrix4_equal(a.as_mut_ptr().cast(), b.as_mut_ptr().cast()) }
}

#[must_use]
pub fn matrix4_is_identity(matrix: Matrix4) -> bool {
    let mut matrix = matrix;
    unsafe { scn_extra_matrix4_is_identity(matrix.as_mut_ptr().cast()) }
}

#[must_use]
pub fn matrix4_make_rotation(angle: f32, x: f32, y: f32, z: f32) -> Matrix4 {
    let mut matrix = Matrix4::default();
    unsafe {
        scn_extra_matrix4_make_rotation(angle, x, y, z, matrix.as_mut_ptr().cast());
    }
    matrix
}

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

#[must_use]
pub fn matrix4_invert(matrix: Matrix4) -> Matrix4 {
    let mut matrix = matrix;
    let mut out = Matrix4::default();
    unsafe { scn_extra_matrix4_invert(matrix.as_mut_ptr().cast(), out.as_mut_ptr().cast()) };
    out
}

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
        )
    };
    out
}

#[must_use]
pub fn matrix4_to_glk(matrix: Matrix4) -> GlkMatrix4 {
    let mut matrix = matrix;
    let mut glk = GlkMatrix4::default();
    unsafe { scn_extra_matrix4_to_glk(matrix.as_mut_ptr().cast(), glk.elements.as_mut_ptr()) };
    glk
}

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
