use core::ffi::c_void;
use core::ops::Deref;
use core::ptr;
use std::path::Path;

use crate::animation::Animation;
use crate::geometry::Geometry;
use crate::math::Vector3;
use crate::node::Node;
use crate::private::{cstring_from_path, handle_type, Sealed};
use crate::protocols::Animatable;
use crate::symbols::{MorpherCalculationMode, ParticleInputMode, ReferenceLoadingPolicy};

extern "C" {
    fn scn_geometry_new_pyramid(width: f64, height: f64, length: f64) -> *mut c_void;
    fn scn_geometry_new_tube(inner_radius: f64, outer_radius: f64, height: f64) -> *mut c_void;
    fn scn_geometry_new_capsule(cap_radius: f64, height: f64) -> *mut c_void;
    fn scn_geometry_new_torus(ring_radius: f64, pipe_radius: f64) -> *mut c_void;
    fn scn_geometry_new_shape(extrusion_depth: f64) -> *mut c_void;

    fn scn_geometry_source_new_vertices(vertices: *const c_void, count: usize) -> *mut c_void;
    fn scn_geometry_source_new_normals(normals: *const c_void, count: usize) -> *mut c_void;
    fn scn_geometry_source_new_texcoords(texcoords: *const c_void, count: usize) -> *mut c_void;
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
        bone_weights: *mut c_void,
        bone_indices: *mut c_void,
    ) -> *mut c_void;
    fn scn_node_get_skinner(node: *mut c_void) -> *mut c_void;
    fn scn_node_set_skinner(node: *mut c_void, skinner: *mut c_void);
}

macro_rules! geometry_newtype {
    ($name:ident) => {
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
handle_type!(GeometryTessellator);
handle_type!(LevelOfDetail);
handle_type!(Morpher);
handle_type!(ParticlePropertyController);
handle_type!(Skinner);

impl Pyramid {
    #[must_use]
    pub fn new(width: f64, height: f64, length: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_new_pyramid(width, height, length)) }
    }
}

impl Tube {
    #[must_use]
    pub fn new(inner_radius: f64, outer_radius: f64, height: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_new_tube(inner_radius, outer_radius, height)) }
    }
}

impl Capsule {
    #[must_use]
    pub fn new(cap_radius: f64, height: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_new_capsule(cap_radius, height)) }
    }
}

impl Torus {
    #[must_use]
    pub fn new(ring_radius: f64, pipe_radius: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_new_torus(ring_radius, pipe_radius)) }
    }
}

impl Shape {
    #[must_use]
    pub fn with_extrusion_depth(extrusion_depth: f64) -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_new_shape(extrusion_depth)) }
    }
}

impl GeometrySource {
    #[must_use]
    pub fn with_vertices(vertices: &[Vector3]) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_geometry_source_new_vertices(
                vertices.as_ptr().cast(),
                vertices.len(),
            ))
        }
    }

    #[must_use]
    pub fn with_normals(normals: &[Vector3]) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_geometry_source_new_normals(
                normals.as_ptr().cast(),
                normals.len(),
            ))
        }
    }

    #[must_use]
    pub fn with_texture_coordinates(texcoords: &[crate::CGPoint]) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_geometry_source_new_texcoords(
                texcoords.as_ptr().cast(),
                texcoords.len(),
            ))
        }
    }
}

impl GeometryElement {
    #[must_use]
    pub fn with_data(
        data: Option<&[u8]>,
        primitive_type: crate::symbols::GeometryPrimitiveType,
        primitive_count: usize,
        bytes_per_index: usize,
    ) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_geometry_element_new(
                data.map_or(ptr::null(), <[u8]>::as_ptr).cast(),
                data.map_or(0, <[u8]>::len),
                primitive_type as i32,
                primitive_count,
                bytes_per_index,
            ))
        }
    }
}

impl Geometry {
    #[must_use]
    pub fn with_sources_elements(
        sources: &[&GeometrySource],
        elements: &[&GeometryElement],
    ) -> Option<Self> {
        let mut source_ptrs: Vec<*mut c_void> =
            sources.iter().map(|source| source.as_ptr()).collect();
        let mut element_ptrs: Vec<*mut c_void> =
            elements.iter().map(|element| element.as_ptr()).collect();
        unsafe {
            Self::from_raw(scn_geometry_new_with_sources_elements(
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
            ))
        }
    }

    #[must_use]
    pub fn tessellator(&self) -> Option<GeometryTessellator> {
        unsafe { GeometryTessellator::from_raw(scn_geometry_get_tessellator(self.as_ptr())) }
    }

    pub fn set_tessellator(&self, tessellator: Option<&GeometryTessellator>) {
        unsafe {
            scn_geometry_set_tessellator(
                self.as_ptr(),
                tessellator.map_or(ptr::null_mut(), GeometryTessellator::as_ptr),
            );
        };
    }

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

    #[must_use]
    pub fn levels_of_detail_count(&self) -> usize {
        unsafe { scn_geometry_levels_of_detail_count(self.as_ptr()) }
    }
}

impl GeometryTessellator {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(scn_geometry_tessellator_new()) }
    }
}

impl LevelOfDetail {
    #[must_use]
    pub fn with_screen_space_radius(geometry: Option<&Geometry>, radius: f64) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_level_of_detail_new_screen_space(
                geometry.map_or(ptr::null_mut(), Geometry::as_ptr),
                radius,
            ))
        }
    }

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
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(scn_morpher_new()) }
    }

    #[must_use]
    pub fn calculation_mode(&self) -> MorpherCalculationMode {
        match unsafe { scn_morpher_get_calculation_mode(self.as_ptr()) } {
            1 => MorpherCalculationMode::Additive,
            _ => MorpherCalculationMode::Normalized,
        }
    }

    pub fn set_calculation_mode(&self, calculation_mode: MorpherCalculationMode) {
        unsafe { scn_morpher_set_calculation_mode(self.as_ptr(), calculation_mode as i32) };
    }
}

impl ParticlePropertyController {
    #[must_use]
    pub fn with_animation(animation: &Animation) -> Option<Self> {
        unsafe {
            Self::from_raw(scn_particle_property_controller_new_with_animation(
                animation.as_ptr(),
            ))
        }
    }

    #[must_use]
    pub fn input_mode(&self) -> ParticleInputMode {
        match unsafe { scn_particle_property_controller_get_input_mode(self.as_ptr()) } {
            1 => ParticleInputMode::OverDistance,
            2 => ParticleInputMode::OverOtherProperty,
            _ => ParticleInputMode::OverLife,
        }
    }

    pub fn set_input_mode(&self, input_mode: ParticleInputMode) {
        unsafe {
            scn_particle_property_controller_set_input_mode(self.as_ptr(), input_mode as i32)
        };
    }
}

impl ReferenceNode {
    #[must_use]
    pub fn with_url(path: impl AsRef<Path>) -> Option<Self> {
        let path = cstring_from_path(path.as_ref())?;
        unsafe { Self::from_raw(scn_reference_node_new_url(path.as_ptr())) }
    }

    #[must_use]
    pub fn loading_policy(&self) -> ReferenceLoadingPolicy {
        match unsafe { scn_reference_node_get_loading_policy(self.as_ptr()) } {
            1 => ReferenceLoadingPolicy::OnDemand,
            _ => ReferenceLoadingPolicy::Immediate,
        }
    }

    pub fn set_loading_policy(&self, loading_policy: ReferenceLoadingPolicy) {
        unsafe { scn_reference_node_set_loading_policy(self.as_ptr(), loading_policy as i32) };
    }

    pub fn load_reference(&self) {
        unsafe { scn_reference_node_load(self.as_ptr()) };
    }

    pub fn unload_reference(&self) {
        unsafe { scn_reference_node_unload(self.as_ptr()) };
    }

    #[must_use]
    pub fn is_loaded(&self) -> bool {
        unsafe { scn_reference_node_get_loaded(self.as_ptr()) }
    }
}

impl Skinner {
    #[must_use]
    pub fn new(
        base_geometry: Option<&Geometry>,
        bones: &[&Node],
        bone_weights: &GeometrySource,
        bone_indices: &GeometrySource,
    ) -> Option<Self> {
        let mut bone_ptrs: Vec<*mut c_void> = bones.iter().map(|bone| bone.as_ptr()).collect();
        unsafe {
            Self::from_raw(scn_skinner_new(
                base_geometry.map_or(ptr::null_mut(), Geometry::as_ptr),
                if bone_ptrs.is_empty() {
                    ptr::null_mut()
                } else {
                    bone_ptrs.as_mut_ptr().cast()
                },
                bone_ptrs.len(),
                bone_weights.as_ptr(),
                bone_indices.as_ptr(),
            ))
        }
    }
}

impl Node {
    #[must_use]
    pub fn morpher(&self) -> Option<Morpher> {
        unsafe { Morpher::from_raw(scn_node_get_morpher(self.as_ptr())) }
    }

    pub fn set_morpher(&self, morpher: Option<&Morpher>) {
        unsafe {
            scn_node_set_morpher(
                self.as_ptr(),
                morpher.map_or(ptr::null_mut(), Morpher::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn skinner(&self) -> Option<Skinner> {
        unsafe { Skinner::from_raw(scn_node_get_skinner(self.as_ptr())) }
    }

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
