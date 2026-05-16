use scenekit::{Node, ParticleSystem};

#[test]
fn test_particle_system_attachment() {
    let node = Node::new().expect("node");
    let system = ParticleSystem::new().expect("particle system");
    system.set_birth_rate(32.0);
    system.set_life_span(1.5);
    system.set_loops(false);
    node.add_particle_system(&system);
    assert_eq!(node.particle_system_count(), 1);
    assert!((system.birth_rate() - 32.0).abs() < f64::EPSILON);
    assert!((system.life_span() - 1.5).abs() < f64::EPSILON);
    assert!(!system.loops());
    node.remove_all_particle_systems();
    assert_eq!(node.particle_system_count(), 0);
}
