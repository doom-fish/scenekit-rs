use scenekit::{Color, Scene};

#[test]
fn test_scene_fog_color_round_trip() {
    let scene = Scene::new().expect("scene");
    scene.set_fog_color(Color::rgba(0.3, 0.2, 0.1, 0.8));
    assert_eq!(scene.fog_color(), Some(Color::rgba(0.3, 0.2, 0.1, 0.8)));
    assert!(scene.background().intensity() >= 0.0);
}
