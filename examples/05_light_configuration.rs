use scenekit::{Color, Light, LightType, ShadowMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let light = Light::new().ok_or("missing light")?;
    light.set_light_type(LightType::Spot);
    light.set_color(Color::white());
    light.set_intensity(900.0);
    light.set_shadow_mode(ShadowMode::Deferred);
    light.set_casts_shadow(true);
    assert_eq!(light.light_type(), Some(LightType::Spot));
    assert!(light.casts_shadow());
    println!("✅ light configured");
    Ok(())
}
