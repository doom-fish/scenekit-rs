use scenekit::{PhysicsBody, PhysicsBodyType, Vector3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = PhysicsBody::dynamic_body().ok_or("missing physics body")?;
    body.set_body_type(PhysicsBodyType::Dynamic);
    body.set_mass(3.5);
    body.set_restitution(0.25);
    body.set_friction(0.75);
    body.apply_force(Vector3::new(0.0, 9.8, 0.0), false);
    assert_eq!(body.body_type(), Some(PhysicsBodyType::Dynamic));
    println!("✅ physics body configured");
    Ok(())
}
