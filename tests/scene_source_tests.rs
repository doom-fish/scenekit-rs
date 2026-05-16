use std::path::Path;

use scenekit::{scene_source, SceneSource, SceneSourceEntryClass, SceneSourceStatus};

#[test]
fn test_scene_source_loads_obj_from_url_and_data() {
    let fixture_path = Path::new("tests/fixtures/triangle.obj");
    let scene_source = SceneSource::from_url(fixture_path).expect("scene source from url");
    let scene = scene_source.scene().expect("scene from url source");
    let geometry_ids = scene_source.identifiers_of_entries(SceneSourceEntryClass::Geometry);

    assert!(scene_source.url().is_some());
    assert!(geometry_ids.iter().all(|identifier| !identifier.is_empty()));
    assert!(scene.root_node().geometry().is_none());
    assert_eq!(
        SceneSourceStatus::from_raw(16),
        Some(SceneSourceStatus::Complete)
    );
    assert!(!scene_source::scene_source_asset_up_axis_key().is_empty());
    assert!(!scene_source::scene_source_animation_import_policy_play().is_empty());
    assert!(!scene_source::detailed_errors_key().is_empty());

    let scene_source_from_data = SceneSource::from_data(include_bytes!("fixtures/triangle.obj"))
        .expect("scene source from data");
    assert!(scene_source_from_data.url().is_none());
    assert!(scene_source_from_data
        .property_for_key(&scene_source::scene_source_asset_contributors_key())
        .is_none());
}
