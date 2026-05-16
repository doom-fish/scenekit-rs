import SceneKit

@_cdecl("scn_particle_system_new")
public func scn_particle_system_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNParticleSystem())
}

@_cdecl("scn_particle_system_get_birth_rate")
public func scn_particle_system_get_birth_rate(_ systemHandle: UnsafeMutableRawPointer?) -> Double {
    guard let system: SCNParticleSystem = scnBorrow(systemHandle) else { return 0 }
    return system.birthRate
}

@_cdecl("scn_particle_system_set_birth_rate")
public func scn_particle_system_set_birth_rate(_ systemHandle: UnsafeMutableRawPointer?, _ birthRate: Double) {
    guard let system: SCNParticleSystem = scnBorrow(systemHandle) else { return }
    system.birthRate = CGFloat(birthRate)
}

@_cdecl("scn_particle_system_get_life_span")
public func scn_particle_system_get_life_span(_ systemHandle: UnsafeMutableRawPointer?) -> Double {
    guard let system: SCNParticleSystem = scnBorrow(systemHandle) else { return 0 }
    return system.particleLifeSpan
}

@_cdecl("scn_particle_system_set_life_span")
public func scn_particle_system_set_life_span(_ systemHandle: UnsafeMutableRawPointer?, _ lifeSpan: Double) {
    guard let system: SCNParticleSystem = scnBorrow(systemHandle) else { return }
    system.particleLifeSpan = CGFloat(lifeSpan)
}

@_cdecl("scn_particle_system_get_loops")
public func scn_particle_system_get_loops(_ systemHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let system: SCNParticleSystem = scnBorrow(systemHandle) else { return false }
    return system.loops
}

@_cdecl("scn_particle_system_set_loops")
public func scn_particle_system_set_loops(_ systemHandle: UnsafeMutableRawPointer?, _ loops: Bool) {
    guard let system: SCNParticleSystem = scnBorrow(systemHandle) else { return }
    system.loops = loops
}

@_cdecl("scn_node_add_particle_system")
public func scn_node_add_particle_system(_ nodeHandle: UnsafeMutableRawPointer?, _ systemHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle),
          let system: SCNParticleSystem = scnBorrow(systemHandle)
    else { return }
    node.addParticleSystem(system)
}

@_cdecl("scn_node_remove_all_particle_systems")
public func scn_node_remove_all_particle_systems(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    node.removeAllParticleSystems()
}

@_cdecl("scn_node_particle_system_count")
public func scn_node_particle_system_count(_ nodeHandle: UnsafeMutableRawPointer?) -> Int {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return 0 }
    return node.particleSystems?.count ?? 0
}
