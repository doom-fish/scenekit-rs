import SceneKit

@_cdecl("scn_physics_body_static")
public func scn_physics_body_static() -> UnsafeMutableRawPointer? {
    scnRetain(SCNPhysicsBody.static())
}

@_cdecl("scn_physics_body_dynamic")
public func scn_physics_body_dynamic() -> UnsafeMutableRawPointer? {
    scnRetain(SCNPhysicsBody.dynamic())
}

@_cdecl("scn_physics_body_kinematic")
public func scn_physics_body_kinematic() -> UnsafeMutableRawPointer? {
    scnRetain(SCNPhysicsBody.kinematic())
}

@_cdecl("scn_physics_body_get_type")
public func scn_physics_body_get_type(_ bodyHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return -1 }
    return Int32(clamping: body.type.rawValue)
}

@_cdecl("scn_physics_body_set_type")
public func scn_physics_body_set_type(_ bodyHandle: UnsafeMutableRawPointer?, _ bodyType: Int32) {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return }
    body.type = SCNPhysicsBodyType(rawValue: Int(bodyType)) ?? .dynamic
}

@_cdecl("scn_physics_body_get_mass")
public func scn_physics_body_get_mass(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return 0 }
    return body.mass
}

@_cdecl("scn_physics_body_set_mass")
public func scn_physics_body_set_mass(_ bodyHandle: UnsafeMutableRawPointer?, _ mass: Double) {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return }
    body.mass = mass
}

@_cdecl("scn_physics_body_get_restitution")
public func scn_physics_body_get_restitution(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return 0 }
    return body.restitution
}

@_cdecl("scn_physics_body_set_restitution")
public func scn_physics_body_set_restitution(_ bodyHandle: UnsafeMutableRawPointer?, _ restitution: Double) {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return }
    body.restitution = restitution
}

@_cdecl("scn_physics_body_get_friction")
public func scn_physics_body_get_friction(_ bodyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return 0 }
    return body.friction
}

@_cdecl("scn_physics_body_set_friction")
public func scn_physics_body_set_friction(_ bodyHandle: UnsafeMutableRawPointer?, _ friction: Double) {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return }
    body.friction = friction
}

@_cdecl("scn_physics_body_get_category_bit_mask")
public func scn_physics_body_get_category_bit_mask(_ bodyHandle: UnsafeMutableRawPointer?) -> UInt {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return 0 }
    return UInt(bitPattern: body.categoryBitMask)
}

@_cdecl("scn_physics_body_set_category_bit_mask")
public func scn_physics_body_set_category_bit_mask(_ bodyHandle: UnsafeMutableRawPointer?, _ mask: UInt) {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return }
    body.categoryBitMask = Int(bitPattern: mask)
}

@_cdecl("scn_physics_body_get_collision_bit_mask")
public func scn_physics_body_get_collision_bit_mask(_ bodyHandle: UnsafeMutableRawPointer?) -> UInt {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return 0 }
    return UInt(bitPattern: body.collisionBitMask)
}

@_cdecl("scn_physics_body_set_collision_bit_mask")
public func scn_physics_body_set_collision_bit_mask(_ bodyHandle: UnsafeMutableRawPointer?, _ mask: UInt) {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return }
    body.collisionBitMask = Int(bitPattern: mask)
}

@_cdecl("scn_physics_body_get_contact_test_bit_mask")
public func scn_physics_body_get_contact_test_bit_mask(_ bodyHandle: UnsafeMutableRawPointer?) -> UInt {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return 0 }
    return UInt(bitPattern: body.contactTestBitMask)
}

@_cdecl("scn_physics_body_set_contact_test_bit_mask")
public func scn_physics_body_set_contact_test_bit_mask(_ bodyHandle: UnsafeMutableRawPointer?, _ mask: UInt) {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return }
    body.contactTestBitMask = Int(bitPattern: mask)
}

@_cdecl("scn_physics_body_apply_force")
public func scn_physics_body_apply_force(
    _ bodyHandle: UnsafeMutableRawPointer?,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ impulse: Bool
) {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return }
    body.applyForce(
        SCNVector3(x: CGFloat(x), y: CGFloat(y), z: CGFloat(z)),
        asImpulse: impulse
    )
}
