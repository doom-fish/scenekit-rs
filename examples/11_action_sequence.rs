use scenekit::{Action, Node, Vector3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::new().ok_or("missing node")?;
    let move_to = Action::move_to(Vector3::new(1.0, 2.0, 3.0), 0.1).ok_or("move_to")?;
    let rotate = Action::rotate_by(Vector3::new(0.0, 1.0, 0.0), 0.1).ok_or("rotate")?;
    let sequence = Action::sequence(&[&move_to, &rotate]).ok_or("sequence")?;
    let repeated = Action::repeat_count(&sequence, 2).ok_or("repeat")?;
    node.run_action(&repeated);
    println!("✅ action sequence created");
    Ok(())
}
