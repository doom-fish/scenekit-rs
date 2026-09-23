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

fn write_two_box_scene(path: &Path) {
    let scene = scenekit::Scene::new().expect("scene");
    for x in [-1.0, 1.0] {
        let geometry = scenekit::Geometry::box_geometry(0.5, 0.5, 0.5, 0.0).expect("box");
        let node = scenekit::Node::with_geometry(Some(&geometry)).expect("node");
        node.set_position(scenekit::Vector3::new(x, 0.0, 0.0));
        scene.root_node().add_child_node(&node);
    }
    scene.write_to_url(path, None).expect("write scene");
}

#[test]
fn loading_options_reach_scene_source_and_scene_loaders() {
    let dir = std::path::PathBuf::from("target/test-output");
    std::fs::create_dir_all(&dir).expect("output dir");
    let path = dir.join(format!("two-boxes-{}.dae", std::process::id()));
    write_two_box_scene(&path);

    let plain = SceneSource::from_url(&path)
        .expect("source")
        .scene()
        .expect("scene");
    let flatten = scenekit::SceneSourceOptions {
        flatten_scene: Some(true),
        ..scenekit::SceneSourceOptions::default()
    };
    let flattened = SceneSource::from_url_with_options(&path, &flatten)
        .expect("source")
        .scene_with_options(&flatten)
        .expect("flattened scene");
    let loaded = scenekit::Scene::from_url_with_options(&path, &flatten).expect("scene");
    std::fs::remove_file(&path).expect("cleanup");

    let plain_children = plain.root_node().child_nodes().len();
    assert_eq!(plain_children, 2);
    assert_eq!(
        flattened.root_node().child_nodes().len(),
        1,
        "flatten_scene merges the two boxes"
    );
    assert_eq!(loaded.root_node().child_nodes().len(), 1);
}

#[test]
fn invalid_loading_options_are_rejected_before_loading() {
    let fixture = Path::new("tests/fixtures/triangle.obj");
    let negative_units = scenekit::SceneSourceOptions {
        convert_units_to_meters: Some(-1.0),
        ..scenekit::SceneSourceOptions::default()
    };
    assert!(SceneSource::from_url_with_options(fixture, &negative_units).is_err());
    let nul_directory = scenekit::SceneSourceOptions {
        asset_directory_urls: vec!["bad\0dir".into()],
        ..scenekit::SceneSourceOptions::default()
    };
    assert!(SceneSource::from_url_with_options(fixture, &nul_directory).is_err());
    let valid = scenekit::SceneSourceOptions {
        animation_import_policy: Some(scenekit::SceneSourceAnimationImportPolicy::DoNotPlay),
        asset_directory_urls: vec!["tests/fixtures".into()],
        check_consistency: Some(true),
        convert_to_y_up: Some(true),
        convert_units_to_meters: Some(1.0),
        create_normals_if_absent: Some(true),
        preserve_original_topology: Some(false),
        strict_conformance: Some(false),
        ..scenekit::SceneSourceOptions::default()
    };
    let source = SceneSource::from_url_with_options(fixture, &valid).expect("source");
    assert!(source.scene_with_options(&valid).is_ok());
}
