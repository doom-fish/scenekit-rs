use scenekit::{CGPoint, Camera, Geometry, Node, Scene, Vector3, View};

#[test]
fn test_view_hit_test_returns_named_node() {
    let scene = Scene::new().expect("scene");
    let root = scene.root_node();
    let sphere = Geometry::sphere(1.0).expect("sphere");
    let sphere_node = Node::with_geometry(Some(&sphere)).expect("sphere node");
    sphere_node.set_name("target");
    root.add_child_node(&sphere_node);

    let camera = Camera::new().expect("camera");
    let camera_node = Node::new().expect("camera node");
    camera_node.set_camera(Some(&camera));
    camera_node.set_position(Vector3::new(0.0, 0.0, 5.0));
    root.add_child_node(&camera_node);

    let view = View::new(100.0, 100.0).expect("view");
    view.set_scene(Some(&scene));
    view.set_point_of_view(Some(&camera_node));
    let hits = view
        .hit_test(CGPoint::new(50.0, 50.0))
        .expect("hit results");
    assert!(!hits.is_empty());
    let first = hits.first().expect("first hit");
    assert_eq!(
        first.node().expect("node").name().as_deref(),
        Some("target")
    );
    assert!(first.world_coordinates().is_some());
}
