use scenekit::{Constraint, Node};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = Node::new().ok_or("missing target")?;
    let constrained = Node::new().ok_or("missing constrained node")?;
    let constraint = Constraint::look_at(Some(&target)).ok_or("missing constraint")?;
    constraint.set_influence_factor(0.8);
    constraint.set_gimbal_lock_enabled(true);
    constrained.set_constraints(&[&constraint]);
    assert_eq!(constrained.constraints_count(), 1);
    println!("✅ constraint attached");
    Ok(())
}
