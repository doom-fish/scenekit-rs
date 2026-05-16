use scenekit::{CGPoint, Camera, Geometry, Node, Scene, Vector3, View};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = Scene::new().ok_or("missing scene")?;
    let root = scene.root_node();
    let sphere = Geometry::sphere(1.0).ok_or("missing sphere")?;
    let sphere_node = Node::with_geometry(Some(&sphere)).ok_or("missing sphere node")?;
    sphere_node.set_name("target");
    root.add_child_node(&sphere_node);

    let camera = Camera::new().ok_or("missing camera")?;
    let camera_node = Node::new().ok_or("missing camera node")?;
    camera_node.set_camera(Some(&camera));
    camera_node.set_position(Vector3::new(0.0, 0.0, 5.0));
    root.add_child_node(&camera_node);

    let view = View::new(100.0, 100.0).ok_or("missing view")?;
    view.set_scene(Some(&scene));
    view.set_point_of_view(Some(&camera_node));
    let hits = view
        .hit_test(CGPoint::new(50.0, 50.0))
        .ok_or("missing hit box")?;
    let first = hits.first().ok_or("missing first hit")?;
    let node = first.node().ok_or("missing hit node")?;
    assert_eq!(node.name().as_deref(), Some("target"));
    println!("✅ hit test succeeded");
    Ok(())
}
