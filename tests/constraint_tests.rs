use scenekit::{Constraint, Node};

#[test]
fn test_constraint_properties_and_attachment() {
    let target = Node::new().expect("target");
    let constrained = Node::new().expect("constrained");
    let look = Constraint::look_at(Some(&target)).expect("look constraint");
    look.set_influence_factor(0.75);
    look.set_gimbal_lock_enabled(true);
    let distance = Constraint::distance(Some(&target)).expect("distance constraint");
    distance.set_minimum_distance(1.0);
    distance.set_maximum_distance(5.0);
    constrained.set_constraints(&[&look, &distance]);
    assert_eq!(constrained.constraints_count(), 2);
    assert!(look.gimbal_lock_enabled());
    assert!((distance.minimum_distance() - 1.0).abs() < f64::EPSILON);
    assert!((distance.maximum_distance() - 5.0).abs() < f64::EPSILON);
}
