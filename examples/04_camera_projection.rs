use scenekit::{Camera, Matrix4};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera = Camera::new().ok_or("missing camera")?;
    camera.set_field_of_view(55.0);
    camera.set_z_near(0.25);
    camera.set_z_far(250.0);
    camera.set_projection_matrix(Matrix4::identity());
    assert!((camera.field_of_view() - 55.0).abs() < f64::EPSILON);
    assert!((camera.z_near() - 0.25).abs() < f64::EPSILON);
    assert!((camera.z_far() - 250.0).abs() < f64::EPSILON);
    println!("✅ camera projection configured");
    Ok(())
}
