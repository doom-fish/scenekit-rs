use scenekit::Color;

mod common;

#[test]
fn test_view_snapshot_and_properties() {
    let (view, scene, camera_node) = common::view_with_camera(80.0, 60.0).expect("view setup");
    view.set_scene(Some(&scene));
    view.set_point_of_view(Some(&camera_node));
    view.set_background_color(Color::rgba(0.1, 0.2, 0.3, 1.0));
    view.set_allows_camera_control(true);
    view.set_renders_continuously(true);
    view.set_preferred_frames_per_second(30);
    let dimensions = view.snapshot_dimensions().expect("dimensions");
    assert!((dimensions.0 - 80.0).abs() < f64::EPSILON);
    assert!((dimensions.1 - 60.0).abs() < f64::EPSILON);
    assert_eq!(
        view.background_color(),
        Some(Color::rgba(0.1, 0.2, 0.3, 1.0))
    );
    assert!(view.allows_camera_control());
    assert!(view.renders_continuously());
    assert_eq!(view.preferred_frames_per_second(), 30);
    assert!(view.scene().is_some());
    assert!(view.point_of_view().is_some());
}
