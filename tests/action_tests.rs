use scenekit::{Action, Actionable, Node, Vector3};

#[test]
fn test_action_builders_attach_to_node() {
    let node = Node::new().expect("node");
    let move_to = Action::move_to(Vector3::new(1.0, 2.0, 3.0), 0.1).expect("move_to");
    let move_by = Action::move_by(Vector3::new(0.0, 1.0, 0.0), 0.1).expect("move_by");
    let sequence = Action::sequence(&[&move_to, &move_by]).expect("sequence");
    let repeat = Action::repeat_count(&sequence, 2).expect("repeat");
    assert!(!node.has_actions());
    node.run_action(&repeat);
    assert!(node.has_actions());

    let spin = Action::rotate_by(Vector3::new(0.0, 1.0, 0.0), 1.0).expect("rotate_by");
    node.run_action_for_key(&Action::repeat_forever(&spin).expect("forever"), "spin");
    assert_eq!(node.action_keys().len(), 2);
    assert!(node.action_keys().iter().any(|key| key == "spin"));
    assert!(node.action_for_key("spin").is_some());
    node.remove_action_for_key("spin");
    assert!(node.action_for_key("spin").is_none());
    node.remove_all_actions();
    assert!(!node.has_actions());
}

#[test]
fn custom_actions_can_be_built_and_attached() {
    let node = Node::new().expect("node");
    let action = Action::custom(0.5, |node, elapsed| {
        let _ = (node.name(), elapsed);
    })
    .expect("custom action");
    node.run_action_for_key(&action, "custom");
    assert!(node.action_for_key("custom").is_some());
}
