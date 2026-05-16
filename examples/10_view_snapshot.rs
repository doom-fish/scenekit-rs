use scenekit::{Camera, Color, Node, Scene, Vector3, View};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = Scene::new().ok_or("missing scene")?;
    let root = scene.root_node();
    let camera = Camera::new().ok_or("missing camera")?;
    let camera_node = Node::new().ok_or("missing camera node")?;
    camera_node.set_camera(Some(&camera));
    camera_node.set_position(Vector3::new(0.0, 0.0, 5.0));
    root.add_child_node(&camera_node);

    let view = View::new(96.0, 64.0).ok_or("missing view")?;
    view.set_scene(Some(&scene));
    view.set_point_of_view(Some(&camera_node));
    view.set_allows_camera_control(true);
    view.set_background_color(Color::rgba(0.2, 0.3, 0.4, 1.0));
    let dimensions = view.snapshot_dimensions().ok_or("missing snapshot")?;
    assert!((dimensions.0 - 96.0).abs() < f64::EPSILON);
    assert!((dimensions.1 - 64.0).abs() < f64::EPSILON);
    println!("✅ view snapshot captured");
    Ok(())
}
