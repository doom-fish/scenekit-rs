mod common;

#[test]
fn test_renderer_offline_smoke() {
    common::renderer_smoke().expect("renderer smoke");
}
