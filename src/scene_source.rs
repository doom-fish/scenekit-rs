use std::path::Path;

use crate::error::{take_error, take_string, SceneKitError};
use crate::ffi;
use crate::private::{cstring_from_path, handle_type, lookup_string_constant};
use crate::scene::Scene;

handle_type!(SceneSource);

/// Mirrors `SCNSceneSourceStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SceneSourceStatus {
    /// Corresponds to the `SCNSceneSourceStatus::Error` case.
    Error = -1,
    /// Corresponds to the `SCNSceneSourceStatus::Parsing` case.
    Parsing = 4,
    /// Corresponds to the `SCNSceneSourceStatus::Validating` case.
    Validating = 8,
    /// Corresponds to the `SCNSceneSourceStatus::Processing` case.
    Processing = 12,
    /// Corresponds to the `SCNSceneSourceStatus::Complete` case.
    Complete = 16,
}

impl SceneSourceStatus {
    /// Mirrors `SCNSceneSourceStatus.fromRaw`.
    #[must_use]
    pub const fn from_raw(value: i32) -> Option<Self> {
        match value {
            -1 => Some(Self::Error),
            4 => Some(Self::Parsing),
            8 => Some(Self::Validating),
            12 => Some(Self::Processing),
            16 => Some(Self::Complete),
            _ => None,
        }
    }
}

/// Mirrors `SCNSceneSource entry classes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SceneSourceEntryClass {
    /// Corresponds to the `SCNSceneSource entry classes::Material` case.
    Material = 0,
    /// Corresponds to the `SCNSceneSource entry classes::Geometry` case.
    Geometry = 1,
    /// Corresponds to the `SCNSceneSource entry classes::Scene` case.
    Scene = 2,
    /// Corresponds to the `SCNSceneSource entry classes::Node` case.
    Node = 3,
    /// Corresponds to the `SCNSceneSource entry classes::Animation` case.
    Animation = 4,
    /// Corresponds to the `SCNSceneSource entry classes::Light` case.
    Light = 5,
    /// Corresponds to the `SCNSceneSource entry classes::Camera` case.
    Camera = 6,
    /// Corresponds to the `SCNSceneSource entry classes::Skinner` case.
    Skinner = 7,
    /// Corresponds to the `SCNSceneSource entry classes::Morpher` case.
    Morpher = 8,
    /// Corresponds to the `SCNSceneSource entry classes::Image` case.
    Image = 9,
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

string_constant_fn!(
    consistency_element_id_error_key,
    "SCNConsistencyElementIDErrorKey"
);
string_constant_fn!(
    consistency_element_type_error_key,
    "SCNConsistencyElementTypeErrorKey"
);
string_constant_fn!(
    consistency_line_number_error_key,
    "SCNConsistencyLineNumberErrorKey"
);
string_constant_fn!(detailed_errors_key, "SCNDetailedErrorsKey");
string_constant_fn!(
    scene_source_animation_import_policy_do_not_play,
    "SCNSceneSourceAnimationImportPolicyDoNotPlay"
);
string_constant_fn!(
    scene_source_animation_import_policy_key,
    "SCNSceneSourceAnimationImportPolicyKey"
);
string_constant_fn!(
    scene_source_animation_import_policy_play,
    "SCNSceneSourceAnimationImportPolicyPlay"
);
string_constant_fn!(
    scene_source_animation_import_policy_play_repeatedly,
    "SCNSceneSourceAnimationImportPolicyPlayRepeatedly"
);
string_constant_fn!(
    scene_source_animation_import_policy_play_using_scene_time_base,
    "SCNSceneSourceAnimationImportPolicyPlayUsingSceneTimeBase"
);
string_constant_fn!(
    scene_source_asset_author_key,
    "SCNSceneSourceAssetAuthorKey"
);
string_constant_fn!(
    scene_source_asset_authoring_tool_key,
    "SCNSceneSourceAssetAuthoringToolKey"
);
string_constant_fn!(
    scene_source_asset_contributors_key,
    "SCNSceneSourceAssetContributorsKey"
);
string_constant_fn!(
    scene_source_asset_created_date_key,
    "SCNSceneSourceAssetCreatedDateKey"
);
string_constant_fn!(
    scene_source_asset_directory_urls_key,
    "SCNSceneSourceAssetDirectoryURLsKey"
);
string_constant_fn!(
    scene_source_asset_modified_date_key,
    "SCNSceneSourceAssetModifiedDateKey"
);
string_constant_fn!(scene_source_asset_unit_key, "SCNSceneSourceAssetUnitKey");
string_constant_fn!(
    scene_source_asset_unit_meter_key,
    "SCNSceneSourceAssetUnitMeterKey"
);
string_constant_fn!(
    scene_source_asset_unit_name_key,
    "SCNSceneSourceAssetUnitNameKey"
);
string_constant_fn!(
    scene_source_asset_up_axis_key,
    "SCNSceneSourceAssetUpAxisKey"
);
string_constant_fn!(
    scene_source_check_consistency_key,
    "SCNSceneSourceCheckConsistencyKey"
);
string_constant_fn!(
    scene_source_convert_to_y_up_key,
    "SCNSceneSourceConvertToYUpKey"
);
string_constant_fn!(
    scene_source_convert_units_to_meters_key,
    "SCNSceneSourceConvertUnitsToMetersKey"
);
string_constant_fn!(
    scene_source_create_normals_if_absent_key,
    "SCNSceneSourceCreateNormalsIfAbsentKey"
);
string_constant_fn!(
    scene_source_flatten_scene_key,
    "SCNSceneSourceFlattenSceneKey"
);
string_constant_fn!(
    scene_source_loading_option_preserve_original_topology,
    "SCNSceneSourceLoadingOptionPreserveOriginalTopology"
);
string_constant_fn!(
    scene_source_override_asset_urls_key,
    "SCNSceneSourceOverrideAssetURLsKey"
);
string_constant_fn!(
    scene_source_strict_conformance_key,
    "SCNSceneSourceStrictConformanceKey"
);

impl SceneSource {
    /// Mirrors `SCNSceneSource.fromUrl`.
    pub fn from_url(path: impl AsRef<Path>) -> Result<Self, SceneKitError> {
        let path = cstring_from_path(path.as_ref())
            .ok_or_else(|| SceneKitError::new("path contains an interior NUL byte"))?;
        let mut error = core::ptr::null_mut();
        let ptr = unsafe { ffi::scn_scene_source_new_url(path.as_ptr(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { take_error(error, "SCNSceneSource(url:options:) returned nil") })
        } else {
            Ok(unsafe { Self::from_raw_unchecked(ptr) })
        }
    }

    /// Mirrors `SCNSceneSource.fromData`.
    pub fn from_data(data: &[u8]) -> Result<Self, SceneKitError> {
        let mut error = core::ptr::null_mut();
        let ptr =
            unsafe { ffi::scn_scene_source_new_data(data.as_ptr().cast(), data.len(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { take_error(error, "SCNSceneSource(data:options:) returned nil") })
        } else {
            Ok(unsafe { Self::from_raw_unchecked(ptr) })
        }
    }

    /// Mirrors `SCNSceneSource.url`.
    #[must_use]
    pub fn url(&self) -> Option<String> {
        unsafe { take_string(ffi::scn_scene_source_copy_url(self.ptr)) }
    }

    /// Mirrors `SCNSceneSource.scene`.
    pub fn scene(&self) -> Result<Scene, SceneKitError> {
        let mut error = core::ptr::null_mut();
        let ptr = unsafe { ffi::scn_scene_source_new_scene(self.ptr, &mut error) };
        if ptr.is_null() {
            Err(unsafe { take_error(error, "SCNSceneSource.scene(options:error:) returned nil") })
        } else {
            Ok(unsafe { Scene::from_raw_unchecked(ptr) })
        }
    }

    /// Mirrors `SCNSceneSource.propertyForKey`.
    #[must_use]
    pub fn property_for_key(&self, key: &str) -> Option<String> {
        let key = crate::private::cstring_from_str(key)?;
        unsafe {
            take_string(ffi::scn_scene_source_copy_property_for_key(
                self.ptr,
                key.as_ptr(),
            ))
        }
    }

    /// Mirrors `SCNSceneSource.identifiersOfEntries`.
    #[must_use]
    pub fn identifiers_of_entries(&self, entry_class: SceneSourceEntryClass) -> Vec<String> {
        let Some(raw) = (unsafe {
            take_string(ffi::scn_scene_source_copy_identifiers_of_entries(
                self.ptr,
                entry_class as i32,
            ))
        }) else {
            return Vec::new();
        };

        if raw.is_empty() {
            Vec::new()
        } else {
            raw.lines().map(ToOwned::to_owned).collect()
        }
    }
}
