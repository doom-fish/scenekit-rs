use scenekit::{Color, Scene};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = Scene::new().ok_or("failed to create scene")?;
    scene.set_fog_color(Color::rgba(0.1, 0.2, 0.3, 0.9));
    let fog = scene.fog_color().ok_or("missing fog color")?;
    assert!((fog.g - 0.2).abs() < f32::EPSILON);
    println!("✅ scene fog configured");
    Ok(())
}
