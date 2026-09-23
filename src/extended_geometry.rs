use core::ffi::{c_char, c_void};
use core::ops::Deref;
use core::ptr;
use std::path::Path;

use crate::animation::Animation;
use crate::error::{take_error, SceneKitError};
use crate::geometry::Geometry;
use crate::math::{Matrix4, Vector3};
use crate::node::Node;
use crate::private::{cstring_from_path, handle_type, Sealed};
use crate::protocols::Animatable;
use crate::symbols::{
    GeometryPrimitiveType, MorpherCalculationMode, ParticleInputMode, ReferenceLoadingPolicy,
};

extern "C" {
    fn scn_geometry_new_pyramid(width: f64, height: f64, length: f64) -> *mut c_void;
    fn scn_geometry_new_tube(inner_radius: f64, outer_radius: f64, height: f64) -> *mut c_void;
    fn scn_geometry_new_capsule(cap_radius: f64, height: f64) -> *mut c_void;
    fn scn_geometry_new_torus(ring_radius: f64, pipe_radius: f64) -> *mut c_void;
    fn scn_geometry_new_shape(extrusion_depth: f64) -> *mut c_void;

    fn scn_geometry_source_new_vertices(vertices: *const c_void, count: usize) -> *mut c_void;
    fn scn_geometry_source_new_normals(normals: *const c_void, count: usize) -> *mut c_void;
    fn scn_geometry_source_new_texcoords(texcoords: *const c_void, count: usize) -> *mut c_void;
    fn scn_geometry_source_new_data(
        data: *const c_void,
        length: usize,
        semantic: i32,
        layout: *const usize,
        uses_float_components: bool,
    ) -> *mut c_void;
    fn scn_geometry_source_get_vector_count(source: *mut c_void) -> isize;
    fn scn_geometry_element_new(
        data: *const c_void,
        length: usize,
        primitive_type: i32,
        primitive_count: usize,
        bytes_per_index: usize,
    ) -> *mut c_void;
    fn scn_geometry_new_with_sources_elements(
        sources: *mut c_void,
        source_count: usize,
        elements: *mut c_void,
        element_count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    fn scn_geometry_tessellator_new() -> *mut c_void;
    fn scn_geometry_get_tessellator(geometry: *mut c_void) -> *mut c_void;
    fn scn_geometry_set_tessellator(geometry: *mut c_void, tessellator: *mut c_void);
    fn scn_level_of_detail_new_screen_space(geometry: *mut c_void, radius: f64) -> *mut c_void;
    fn scn_level_of_detail_new_world_space(geometry: *mut c_void, distance: f64) -> *mut c_void;
    fn scn_geometry_set_levels_of_detail(geometry: *mut c_void, lods: *mut c_void, count: usize);
    fn scn_geometry_levels_of_detail_count(geometry: *mut c_void) -> usize;

    fn scn_morpher_new() -> *mut c_void;
    fn scn_morpher_get_calculation_mode(morpher: *mut c_void) -> i32;
    fn scn_morpher_set_calculation_mode(morpher: *mut c_void, mode: i32);
    fn scn_node_get_morpher(node: *mut c_void) -> *mut c_void;
    fn scn_node_set_morpher(node: *mut c_void, morpher: *mut c_void);

    fn scn_particle_property_controller_new_with_animation(animation: *mut c_void) -> *mut c_void;
    fn scn_particle_property_controller_get_input_mode(controller: *mut c_void) -> i32;
    fn scn_particle_property_controller_set_input_mode(controller: *mut c_void, mode: i32);

    fn scn_reference_node_new_url(path: *const i8) -> *mut c_void;
    fn scn_reference_node_get_loading_policy(node: *mut c_void) -> i32;
    fn scn_reference_node_set_loading_policy(node: *mut c_void, policy: i32);
    fn scn_reference_node_load(node: *mut c_void);
    fn scn_reference_node_unload(node: *mut c_void);
    fn scn_reference_node_get_loaded(node: *mut c_void) -> bool;

    fn scn_skinner_new(
        base_geometry: *mut c_void,
        bones: *mut c_void,
        bone_count: usize,
        inverse_bind_transforms: *const c_void,
        bone_weights: *mut c_void,
        bone_indices: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    fn scn_node_get_skinner(node: *mut c_void) -> *mut c_void;
    fn scn_node_set_skinner(node: *mut c_void, skinner: *mut c_void);
}

macro_rules! geometry_newtype {
    ($name:ident) => {
        #[doc = concat!("Wraps `SCN", stringify!($name), "`.")]
        pub struct $name(Geometry);

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl Deref for $name {
            type Target = Geometry;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$name> for Geometry {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl $name {
            unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
                Geometry::from_raw(ptr).map(Self)
            }
        }
    };
}

macro_rules! node_newtype {
    ($name:ident) => {
        #[doc = concat!("Wraps `SCN", stringify!($name), "`.")]
        pub struct $name(Node);

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl Deref for $name {
            type Target = Node;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$name> for Node {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl $name {
            unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
                Node::from_raw(ptr).map(Self)
            }
        }
    };
}

geometry_newtype!(Pyramid);
geometry_newtype!(Tube);
geometry_newtype!(Capsule);
geometry_newtype!(Torus);
geometry_newtype!(Shape);
node_newtype!(ReferenceNode);

handle_type!(GeometrySource);
handle_type!(GeometryElement);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GeometrySourceSemantic {
    Vertex = 0,
    Normal = 1,
    Color = 2,
    Texcoord = 3,
    Tangent = 4,
    VertexCrease = 5,
    EdgeCrease = 6,
    BoneWeights = 7,
    BoneIndices = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeometrySourceLayout {
    pub semantic: GeometrySourceSemantic,
    pub vector_count: usize,
    pub uses_float_components: bool,
    pub components_per_vector: usize,
    pub bytes_per_component: usize,
    pub data_offset: usize,
    pub data_stride: usize,
}

impl GeometrySourceLayout {
    #[must_use]
    pub const fn packed(
        semantic: GeometrySourceSemantic,
        vector_count: usize,
        uses_float_components: bool,
        components_per_vector: usize,
        bytes_per_component: usize,
    ) -> Self {
        Self {
            semantic,
            vector_count,
            uses_float_components,
            components_per_vector,
            bytes_per_component,
            data_offset: 0,
            data_stride: components_per_vector.saturating_mul(bytes_per_component),
        }
    }

    pub fn required_len(&self) -> Result<usize, SceneKitError> {
        if self.vector_count == 0 {
            return Err(SceneKitError::new(
                "a geometry source needs at least one vector",
            ));
        }
        if !(1..=4).contains(&self.components_per_vector) {
            return Err(SceneKitError::new(format!(
                "components_per_vector must be 1 to 4, got {}",
                self.components_per_vector
            )));
        }
        let valid_width = if self.uses_float_components {
            matches!(self.bytes_per_component, 4 | 8)
        } else {
            matches!(self.bytes_per_component, 1 | 2 | 4)
        };
        if !valid_width {
            return Err(SceneKitError::new(format!(
                "bytes_per_component {} is not valid for {} components",
                self.bytes_per_component,
                if self.uses_float_components {
                    "float"
                } else {
                    "integer"
                }
            )));
        }
        let vector_len = self.components_per_vector * self.bytes_per_component;
        if self.data_stride < vector_len {
            return Err(SceneKitError::new(format!(
                "data_stride {} is smaller than one vector ({vector_len} bytes)",
                self.data_stride
            )));
        }
        let required = (self.vector_count - 1)
            .checked_mul(self.data_stride)
            .and_then(|span| span.checked_add(self.data_offset))
            .and_then(|span| span.checked_add(vector_len))
            .filter(|required| isize::try_from(*required).is_ok())
            .ok_or_else(|| SceneKitError::new("geometry source layout overflows"))?;
        Ok(required)
    }
}

fn index_at(data: &[u8], position: usize, bytes_per_index: usize) -> Option<usize> {
    let start = position.checked_mul(bytes_per_index)?;
    let bytes = data.get(start..start.checked_add(bytes_per_index)?)?;
    Some(match bytes_per_index {
        1 => usize::from(bytes[0]),
        2 => usize::from(u16::from_ne_bytes([bytes[0], bytes[1]])),
        _ => usize::try_from(u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])).ok()?,
    })
}

fn element_index_count(
    data: Option<&[u8]>,
    primitive_type: GeometryPrimitiveType,
    primitive_count: usize,
    bytes_per_index: usize,
) -> Result<usize, SceneKitError> {
    let overflow = || SceneKitError::new("geometry element index count overflows");
    let count = match primitive_type {
        GeometryPrimitiveType::Triangles => primitive_count.checked_mul(3).ok_or_else(overflow)?,
        GeometryPrimitiveType::TriangleStrip if primitive_count == 0 => 0,
        GeometryPrimitiveType::TriangleStrip => {
            primitive_count.checked_add(2).ok_or_else(overflow)?
        }
        GeometryPrimitiveType::Line => primitive_count.checked_mul(2).ok_or_else(overflow)?,
        GeometryPrimitiveType::Point => primitive_count,
        GeometryPrimitiveType::Polygon => {
            let Some(data) = data else {
                return if primitive_count == 0 {
                    Ok(0)
                } else {
                    Err(SceneKitError::new(
                        "polygon elements need data that starts with the vertex count of each polygon",
                    ))
                };
            };
            let mut total = primitive_count;
            for polygon in 0..primitive_count {
                let vertices = index_at(data, polygon, bytes_per_index).ok_or_else(|| {
                    SceneKitError::new("polygon element data is shorter than its polygon counts")
                })?;
                if vertices < 3 {
                    return Err(SceneKitError::new(format!(
                        "polygon {polygon} has {vertices} vertices; at least 3 are required"
                    )));
                }
                total = total.checked_add(vertices).ok_or_else(overflow)?;
            }
            total
        }
    };
    Ok(count)
}
handle_type!(GeometryTessellator);
handle_type!(LevelOfDetail);
handle_type!(Morpher);
handle_type!(ParticlePropertyController);
handle_type!(Skinner);

impl Pyramid {
    /// Creates a wrapped `SCNPyramid` instance.
    #[must_use]
    pub fn new(width: f64, height: f64, length: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_new_pyramid(width, height, length)) }
    }
}

impl Tube {
    /// Creates a wrapped `SCNTube` instance.
    #[must_use]
    pub fn new(inner_radius: f64, outer_radius: f64, height: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_new_tube(inner_radius, outer_radius, height)) }
    }
}

impl Capsule {
    /// Creates a wrapped `SCNCapsule` instance.
    #[must_use]
    pub fn new(cap_radius: f64, height: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_new_capsule(cap_radius, height)) }
    }
}

impl Torus {
    /// Creates a wrapped `SCNTorus` instance.
    #[must_use]
    pub fn new(ring_radius: f64, pipe_radius: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_new_torus(ring_radius, pipe_radius)) }
    }
}

impl Shape {
    /// Mirrors `SCNShape.withExtrusionDepth`.
    #[must_use]
    pub fn with_extrusion_depth(extrusion_depth: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_new_shape(extrusion_depth)) }
    }
}

impl GeometrySource {
    /// Mirrors `SCNGeometrySource.withVertices`.
    #[must_use]
    pub fn with_vertices(vertices: &[Vector3]) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_geometry_source_new_vertices(
                vertices.as_ptr().cast(),
                vertices.len(),
            ))
        }
    }

    /// Mirrors `SCNGeometrySource.withNormals`.
    #[must_use]
    pub fn with_normals(normals: &[Vector3]) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_geometry_source_new_normals(
                normals.as_ptr().cast(),
                normals.len(),
            ))
        }
    }

    /// Mirrors `SCNGeometrySource.withTextureCoordinates`.
    #[must_use]
    pub fn with_texture_coordinates(texcoords: &[crate::CGPoint]) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_geometry_source_new_texcoords(
                texcoords.as_ptr().cast(),
                texcoords.len(),
            ))
        }
    }

    pub fn with_data(data: &[u8], layout: GeometrySourceLayout) -> Result<Self, SceneKitError> {
        let required = layout.required_len()?;
        if data.len() < required {
            return Err(SceneKitError::new(format!(
                "geometry source data has {} bytes; its layout needs {required}",
                data.len()
            )));
        }
        let numbers = [
            layout.vector_count,
            layout.components_per_vector,
            layout.bytes_per_component,
            layout.data_offset,
            layout.data_stride,
        ];
        unsafe {
            Self::from_raw(scn_geometry_source_new_data(
                data.as_ptr().cast(),
                data.len(),
                layout.semantic as i32,
                numbers.as_ptr(),
                layout.uses_float_components,
            ))
        }
        .ok_or_else(|| SceneKitError::new("SCNGeometrySource(data:semantic:...) returned nil"))
    }

    #[must_use]
    pub fn vector_count(&self) -> usize {
        usize::try_from(unsafe { scn_geometry_source_get_vector_count(self.as_ptr()) }).unwrap_or(0)
    }
}

impl GeometryElement {
    /// Mirrors `SCNGeometryElement.withData`.
    pub fn with_data(
        data: Option<&[u8]>,
        primitive_type: GeometryPrimitiveType,
        primitive_count: usize,
        bytes_per_index: usize,
    ) -> Result<Self, SceneKitError> {
        if !matches!(bytes_per_index, 1 | 2 | 4) {
            return Err(SceneKitError::new(format!(
                "bytes_per_index must be 1, 2 or 4, got {bytes_per_index}"
            )));
        }
        if isize::try_from(primitive_count).is_err() {
            return Err(SceneKitError::new("primitive_count exceeds isize::MAX"));
        }
        let index_count =
            element_index_count(data, primitive_type, primitive_count, bytes_per_index)?;
        if let Some(data) = data {
            let expected = index_count
                .checked_mul(bytes_per_index)
                .ok_or_else(|| SceneKitError::new("geometry element byte count overflows"))?;
            if data.len() != expected {
                return Err(SceneKitError::new(format!(
                    "geometry element data has {} bytes; {primitive_count} primitives of type \
                     {primitive_type:?} with {bytes_per_index}-byte indices need {expected}",
                    data.len()
                )));
            }
        }
        unsafe {
            Self::from_raw(scn_geometry_element_new(
                data.map_or(ptr::null(), <[u8]>::as_ptr).cast(),
                data.map_or(0, <[u8]>::len),
                primitive_type as i32,
                primitive_count,
                bytes_per_index,
            ))
        }
        .ok_or_else(|| SceneKitError::new("SCNGeometryElement(data:...) returned nil"))
    }
}

impl Geometry {
    /// Mirrors `SCNGeometry.withSourcesElements`.
    pub fn with_sources_elements(
        sources: &[&GeometrySource],
        elements: &[&GeometryElement],
    ) -> Result<Self, SceneKitError> {
        let mut source_ptrs: Vec<*mut c_void> =
            sources.iter().map(|source| source.as_ptr()).collect();
        let mut element_ptrs: Vec<*mut c_void> =
            elements.iter().map(|element| element.as_ptr()).collect();
        let mut error = ptr::null_mut();
        let geometry = unsafe {
            scn_geometry_new_with_sources_elements(
                if source_ptrs.is_empty() {
                    ptr::null_mut()
                } else {
                    source_ptrs.as_mut_ptr().cast()
                },
                source_ptrs.len(),
                if element_ptrs.is_empty() {
                    ptr::null_mut()
                } else {
                    element_ptrs.as_mut_ptr().cast()
                },
                element_ptrs.len(),
                &raw mut error,
            )
        };
        unsafe { Self::from_raw(geometry) }.ok_or_else(|| unsafe {
            take_error(error, "SCNGeometry(sources:elements:) returned nil")
        })
    }

    /// Mirrors `SCNGeometry.tessellator`.
    #[must_use]
    pub fn tessellator(&self) -> Option<GeometryTessellator> {
        unsafe { GeometryTessellator::from_raw(scn_geometry_get_tessellator(self.as_ptr())) }
    }

    /// Sets the `SCNGeometry.tessellator` member.
    pub fn set_tessellator(&self, tessellator: Option<&GeometryTessellator>) {
        unsafe {
            scn_geometry_set_tessellator(
                self.as_ptr(),
                tessellator.map_or(ptr::null_mut(), GeometryTessellator::as_ptr),
            );
        };
    }

    /// Sets the `SCNGeometry.levelsOfDetail` member.
    pub fn set_levels_of_detail(&self, levels_of_detail: &[&LevelOfDetail]) {
        let mut level_ptrs: Vec<*mut c_void> = levels_of_detail
            .iter()
            .map(|level_of_detail| level_of_detail.as_ptr())
            .collect();
        unsafe {
            scn_geometry_set_levels_of_detail(
                self.as_ptr(),
                if level_ptrs.is_empty() {
                    ptr::null_mut()
                } else {
                    level_ptrs.as_mut_ptr().cast()
                },
                level_ptrs.len(),
            );
        };
    }

    /// Mirrors `SCNGeometry.levelsOfDetailCount`.
    #[must_use]
    pub fn levels_of_detail_count(&self) -> usize {
        unsafe { scn_geometry_levels_of_detail_count(self.as_ptr()) }
    }
}

impl GeometryTessellator {
    /// Creates a wrapped `SCNGeometryTessellator` instance.
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_tessellator_new()) }
    }
}

impl LevelOfDetail {
    /// Mirrors `SCNLevelOfDetail.withScreenSpaceRadius`.
    #[must_use]
    pub fn with_screen_space_radius(geometry: Option<&Geometry>, radius: f64) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_level_of_detail_new_screen_space(
                geometry.map_or(ptr::null_mut(), Geometry::as_ptr),
                radius,
            ))
        }
    }

    /// Mirrors `SCNLevelOfDetail.withWorldSpaceDistance`.
    #[must_use]
    pub fn with_world_space_distance(geometry: Option<&Geometry>, distance: f64) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_level_of_detail_new_world_space(
                geometry.map_or(ptr::null_mut(), Geometry::as_ptr),
                distance,
            ))
        }
    }
}

impl Morpher {
    /// Creates a wrapped `SCNMorpher` instance.
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(scn_morpher_new()) }
    }

    /// Mirrors `SCNMorpher.calculationMode`.
    #[must_use]
    pub fn calculation_mode(&self) -> MorpherCalculationMode {
        match unsafe { scn_morpher_get_calculation_mode(self.as_ptr()) } {
            1 => MorpherCalculationMode::Additive,
            _ => MorpherCalculationMode::Normalized,
        }
    }

    /// Sets the `SCNMorpher.calculationMode` member.
    pub fn set_calculation_mode(&self, calculation_mode: MorpherCalculationMode) {
        unsafe { scn_morpher_set_calculation_mode(self.as_ptr(), calculation_mode as i32) };
    }
}

impl ParticlePropertyController {
    /// Mirrors `SCNParticlePropertyController.withAnimation`.
    #[must_use]
    pub fn with_animation(animation: &Animation) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_particle_property_controller_new_with_animation(
                animation.as_ptr(),
            ))
        }
    }

    /// Mirrors `SCNParticlePropertyController.inputMode`.
    #[must_use]
    pub fn input_mode(&self) -> ParticleInputMode {
        match unsafe { scn_particle_property_controller_get_input_mode(self.as_ptr()) } {
            1 => ParticleInputMode::OverDistance,
            2 => ParticleInputMode::OverOtherProperty,
            _ => ParticleInputMode::OverLife,
        }
    }

    /// Sets the `SCNParticlePropertyController.inputMode` member.
    pub fn set_input_mode(&self, input_mode: ParticleInputMode) {
        unsafe {
            scn_particle_property_controller_set_input_mode(self.as_ptr(), input_mode as i32);
        };
    }
}

impl ReferenceNode {
    /// Mirrors `SCNReferenceNode.withUrl`.
    #[must_use]
    pub fn with_url(path: impl AsRef<Path>) -> Option<Self> {
        let path = cstring_from_path(path.as_ref())?;
        unsafe { Self::from_raw(scn_reference_node_new_url(path.as_ptr())) }
    }

    /// Mirrors `SCNReferenceNode.loadingPolicy`.
    #[must_use]
    pub fn loading_policy(&self) -> ReferenceLoadingPolicy {
        match unsafe { scn_reference_node_get_loading_policy(self.as_ptr()) } {
            1 => ReferenceLoadingPolicy::OnDemand,
            _ => ReferenceLoadingPolicy::Immediate,
        }
    }

    /// Sets the `SCNReferenceNode.loadingPolicy` member.
    pub fn set_loading_policy(&self, loading_policy: ReferenceLoadingPolicy) {
        unsafe { scn_reference_node_set_loading_policy(self.as_ptr(), loading_policy as i32) };
    }

    /// Mirrors `SCNReferenceNode.loadReference`.
    pub fn load_reference(&self) {
        unsafe { scn_reference_node_load(self.as_ptr()) };
    }

    /// Mirrors `SCNReferenceNode.unloadReference`.
    pub fn unload_reference(&self) {
        unsafe { scn_reference_node_unload(self.as_ptr()) };
    }

    /// Returns the `SCNReferenceNode.isLoaded` value.
    #[must_use]
    pub fn is_loaded(&self) -> bool {
        unsafe { scn_reference_node_get_loaded(self.as_ptr()) }
    }
}

impl Skinner {
    /// Creates a wrapped `SCNSkinner` instance.
    pub fn new(
        base_geometry: &Geometry,
        bones: &[&Node],
        bone_inverse_bind_transforms: Option<&[Matrix4]>,
        bone_weights: &GeometrySource,
        bone_indices: &GeometrySource,
    ) -> Result<Self, SceneKitError> {
        if let Some(transforms) = bone_inverse_bind_transforms {
            if transforms.len() != bones.len() {
                return Err(SceneKitError::new(format!(
                    "{} inverse bind transforms were given for {} bones",
                    transforms.len(),
                    bones.len()
                )));
            }
        }
        let mut bone_ptrs: Vec<*mut c_void> = bones.iter().map(|bone| bone.as_ptr()).collect();
        let mut error = ptr::null_mut();
        let skinner = unsafe {
            scn_skinner_new(
                base_geometry.as_ptr(),
                if bone_ptrs.is_empty() {
                    ptr::null_mut()
                } else {
                    bone_ptrs.as_mut_ptr().cast()
                },
                bone_ptrs.len(),
                bone_inverse_bind_transforms
                    .map_or(ptr::null(), |transforms| transforms.as_ptr().cast()),
                bone_weights.as_ptr(),
                bone_indices.as_ptr(),
                &raw mut error,
            )
        };
        unsafe { Self::from_raw(skinner) }.ok_or_else(|| unsafe {
            take_error(error, "SCNSkinner(baseGeometry:...) returned nil")
        })
    }
}

impl Node {
    /// Mirrors `SCNNode.morpher`.
    #[must_use]
    pub fn morpher(&self) -> Option<Morpher> {
        unsafe { Morpher::from_raw(scn_node_get_morpher(self.as_ptr())) }
    }

    /// Sets the `SCNNode.morpher` member.
    pub fn set_morpher(&self, morpher: Option<&Morpher>) {
        unsafe {
            scn_node_set_morpher(
                self.as_ptr(),
                morpher.map_or(ptr::null_mut(), Morpher::as_ptr),
            );
        };
    }

    /// Mirrors `SCNNode.skinner`.
    #[must_use]
    pub fn skinner(&self) -> Option<Skinner> {
        unsafe { Skinner::from_raw(scn_node_get_skinner(self.as_ptr())) }
    }

    /// Sets the `SCNNode.skinner` member.
    pub fn set_skinner(&self, skinner: Option<&Skinner>) {
        unsafe {
            scn_node_set_skinner(
                self.as_ptr(),
                skinner.map_or(ptr::null_mut(), Skinner::as_ptr),
            );
        };
    }
}

impl Sealed for Morpher {}
impl Animatable for Morpher {
    fn animatable_ptr(&self) -> *mut c_void {
        self.as_ptr()
    }
}
