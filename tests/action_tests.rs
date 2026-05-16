use scenekit::{Action, Node, Vector3};

#[test]
fn test_action_builders_attach_to_node() {
    let node = Node::new().expect("node");
    let move_to = Action::move_to(Vector3::new(1.0, 2.0, 3.0), 0.1).expect("move_to");
    let move_by = Action::move_by(Vector3::new(0.0, 1.0, 0.0), 0.1).expect("move_by");
    let sequence = Action::sequence(&[&move_to, &move_by]).expect("sequence");
    let repeat = Action::repeat_count(&sequence, 2).expect("repeat");
    node.run_action(&repeat);
}
