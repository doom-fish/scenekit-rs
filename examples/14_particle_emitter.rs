use scenekit::{Node, ParticleSystem};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::new().ok_or("missing node")?;
    let system = ParticleSystem::new().ok_or("missing particle system")?;
    system.set_birth_rate(64.0);
    system.set_life_span(2.0);
    system.set_loops(false);
    node.add_particle_system(&system);
    assert_eq!(node.particle_system_count(), 1);
    println!("✅ particle system attached");
    Ok(())
}
