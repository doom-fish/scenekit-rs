mod common;

#[test]
fn test_renderer_offline_smoke() {
    let pixels = common::renderer_smoke().expect("renderer smoke");
    assert_eq!(pixels.len(), 64 * 64 * 4);
    let center = (32 * 64 + 32) * 4;
    let (blue, green, red, alpha) = (
        pixels[center],
        pixels[center + 1],
        pixels[center + 2],
        pixels[center + 3],
    );
    assert!(
        green > 32,
        "the lit green cube should cover the center pixel"
    );
    assert!(
        green > red && green > blue,
        "center pixel {blue},{green},{red} is not green"
    );
    assert_eq!(alpha, 255);
    assert_eq!(
        &pixels[0..3],
        &[0, 0, 0],
        "the corner keeps the black clear color"
    );
}
