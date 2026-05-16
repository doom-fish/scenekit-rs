use scenekit::{Color, Material};

#[test]
fn test_material_property_round_trip() {
    let material = Material::new().expect("material");
    material.diffuse().set_color(Color::green());
    material.normal().set_intensity(0.75);
    material.transparent().clear_contents();
    assert_eq!(material.diffuse().color(), Some(Color::green()));
    assert!((material.normal().intensity() - 0.75).abs() < f64::EPSILON);
}
