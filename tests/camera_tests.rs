use scenekit::{Camera, Matrix4};

#[test]
fn test_camera_projection_properties_round_trip() {
    let camera = Camera::new().expect("camera");
    camera.set_field_of_view(42.0);
    camera.set_z_near(0.2);
    camera.set_z_far(200.0);
    camera.set_projection_matrix(Matrix4::identity());
    assert!((camera.field_of_view() - 42.0).abs() < f64::EPSILON);
    assert!((camera.z_near() - 0.2).abs() < f64::EPSILON);
    assert!((camera.z_far() - 200.0).abs() < f64::EPSILON);
    assert_eq!(camera.projection_matrix(), Matrix4::identity());
}
