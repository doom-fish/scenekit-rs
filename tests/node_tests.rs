use scenekit::{Geometry, Matrix4, Node, Vector3};

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
    assert_eq!(parent.child_nodes().len(), 1);
    assert_eq!(
        child.parent().and_then(|node| node.name()),
        parent.name(),
        "the child reports its parent"
    );
    child.remove_from_parent();
    assert!(child.parent().is_none());
    assert!(parent.child_nodes().is_empty());
}

#[test]
fn child_lookup_honours_recursion() {
    let root = Node::new().expect("root");
    let middle = Node::new().expect("middle");
    let leaf = Node::new().expect("leaf");
    middle.set_name("middle");
    leaf.set_name("leaf");
    root.add_child_node(&middle);
    middle.add_child_node(&leaf);

    assert!(root.child_node_with_name("middle", false).is_some());
    assert!(root.child_node_with_name("leaf", false).is_none());
    let found = root
        .child_node_with_name("leaf", true)
        .expect("recursive lookup");
    assert_eq!(found.name().as_deref(), Some("leaf"));
    assert!(root.child_node_with_name("missing", true).is_none());
    assert!(root.child_node_with_name("bad\0name", true).is_none());
}

#[test]
fn clones_copy_the_subtree() {
    let root = Node::new().expect("root");
    let child = Node::new().expect("child");
    child.set_name("child");
    child.set_position(Vector3::new(0.0, 4.0, 0.0));
    root.add_child_node(&child);

    let clone = root.clone_node().expect("clone");
    let cloned_child = clone
        .child_node_with_name("child", false)
        .expect("cloned child");
    assert_eq!(cloned_child.position(), Vector3::new(0.0, 4.0, 0.0));
    assert_ne!(cloned_child.as_ptr(), child.as_ptr());
    assert_eq!(root.child_nodes().len(), 1);
}

#[test]
fn world_transform_includes_the_parent_transform() {
    let parent = Node::new().expect("parent");
    let child = Node::new().expect("child");
    parent.add_child_node(&child);
    parent.set_position(Vector3::new(10.0, 0.0, 0.0));
    child.set_position(Vector3::new(0.0, 2.0, 0.0));

    let world = child.world_transform();
    assert!((world.elements[12] - 10.0).abs() < 1e-4, "{world:?}");
    assert!((world.elements[13] - 2.0).abs() < 1e-4, "{world:?}");

    let mut target = Matrix4::identity();
    target.elements[14] = 7.0;
    child.set_world_transform(target);
    assert!((child.position().x + 10.0).abs() < 1e-4);
    assert!((child.position().z - 7.0).abs() < 1e-4);
}

#[test]
fn opacity_and_category_mask_round_trip() {
    let node = Node::new().expect("node");
    assert!((node.opacity() - 1.0).abs() < f64::EPSILON);
    node.set_opacity(0.25);
    assert!((node.opacity() - 0.25).abs() < f64::EPSILON);

    assert_eq!(node.category_bit_mask(), 1);
    node.set_category_bit_mask(0b1010);
    assert_eq!(node.category_bit_mask(), 0b1010);
    node.set_category_bit_mask(usize::MAX);
    assert_eq!(node.category_bit_mask(), usize::MAX);
}
