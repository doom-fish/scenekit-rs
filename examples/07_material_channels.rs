use scenekit::{Color, Material};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let material = Material::new().ok_or("missing material")?;
    material.diffuse().set_color(Color::red());
    material.normal().set_intensity(0.5);
    material.specular().set_color(Color::white());
    material.emission().set_color(Color::blue());
    material.transparent().clear_contents();
    assert_eq!(material.diffuse().color(), Some(Color::red()));
    assert!((material.normal().intensity() - 0.5).abs() < f64::EPSILON);
    println!("✅ material channels configured");
    Ok(())
}
