use scenekit::{Geometry, Node, Vector3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let geometry = Geometry::box_geometry(1.0, 2.0, 3.0, 0.0).ok_or("missing geometry")?;
    let parent = Node::new().ok_or("missing parent node")?;
    parent.set_name("parent");
    let child = Node::with_geometry(Some(&geometry)).ok_or("missing child node")?;
    child.set_position(Vector3::new(1.0, 2.0, 3.0));
    parent.add_child_node(&child);
    assert_eq!(parent.name().as_deref(), Some("parent"));
    assert_eq!(child.position(), Vector3::new(1.0, 2.0, 3.0));
    println!("✅ node hierarchy configured");
    Ok(())
}
