use scenekit::{PhysicsBody, PhysicsBodyType, Vector3};

#[test]
fn test_physics_body_properties_round_trip() {
    let body = PhysicsBody::dynamic_body().expect("body");
    body.set_body_type(PhysicsBodyType::Dynamic);
    body.set_mass(4.0);
    body.set_restitution(0.3);
    body.set_friction(0.6);
    body.apply_force(Vector3::new(1.0, 0.0, 0.0), false);
    assert_eq!(body.body_type(), Some(PhysicsBodyType::Dynamic));
    assert!((body.mass() - 4.0).abs() < f64::EPSILON);
    assert!((body.restitution() - 0.3).abs() < f64::EPSILON);
    assert!((body.friction() - 0.6).abs() < f64::EPSILON);
}
