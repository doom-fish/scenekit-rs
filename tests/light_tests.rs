use scenekit::{Color, Light, LightType, ShadowMode};

#[test]
fn test_light_properties_round_trip() {
    let light = Light::new().expect("light");
    light.set_light_type(LightType::Directional);
    light.set_color(Color::white());
    light.set_intensity(750.0);
    light.set_shadow_mode(ShadowMode::Forward);
    light.set_casts_shadow(true);
    assert_eq!(light.light_type(), Some(LightType::Directional));
    assert_eq!(light.color(), Some(Color::white()));
    assert!(light.casts_shadow());
}
