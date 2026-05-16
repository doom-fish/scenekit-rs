use std::path::Path;

use scenekit::{scene_source, SceneSource, SceneSourceEntryClass, SceneSourceStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = SceneSource::from_url(Path::new("tests/fixtures/triangle.obj"))?;
    let geometry_ids = source.identifiers_of_entries(SceneSourceEntryClass::Geometry);
    let scene = source.scene()?;

    assert!(source.url().is_some());
    assert_eq!(
        SceneSourceStatus::from_raw(16),
        Some(SceneSourceStatus::Complete)
    );
    assert!(!scene_source::scene_source_animation_import_policy_play().is_empty());
    println!(
        "✅ loaded scene source ({} geometry identifiers)",
        geometry_ids.len()
    );
    drop(scene);
    Ok(())
}
