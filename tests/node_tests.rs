use scenekit::{Geometry, Node, Vector3};

#[test]
fn test_node_hierarchy_and_transform_round_trip() {
    let geometry = Geometry::sphere(1.0).expect("geometry");
    let parent = Node::new().expect("parent");
    let child = Node::with_geometry(Some(&geometry)).expect("child");
    child.set_name("child");
    child.set_position(Vector3::new(1.0, 2.0, 3.0));
    parent.add_child_node(&child);
    assert_eq!(child.name().as_deref(), Some("child"));
    assert_eq!(child.position(), Vector3::new(1.0, 2.0, 3.0));
    child.remove_from_parent();
}
