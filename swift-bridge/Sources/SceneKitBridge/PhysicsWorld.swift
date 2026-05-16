import SceneKit

public typealias PhysicsContactCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void

private final class PhysicsContactDelegateBox: NSObject, SCNPhysicsContactDelegate {
    let context: UnsafeMutableRawPointer?
    let releaseContext: ScnReleaseContextCallback?
    let didBeginContact: PhysicsContactCallback
    let didUpdateContact: PhysicsContactCallback
    let didEndContact: PhysicsContactCallback

    init(
        context: UnsafeMutableRawPointer?,
        releaseContext: ScnReleaseContextCallback?,
        didBeginContact: @escaping PhysicsContactCallback,
        didUpdateContact: @escaping PhysicsContactCallback,
        didEndContact: @escaping PhysicsContactCallback
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.didBeginContact = didBeginContact
        self.didUpdateContact = didUpdateContact
        self.didEndContact = didEndContact
    }

    deinit {
        releaseContext?(context)
    }

    func physicsWorld(_ world: SCNPhysicsWorld, didBegin contact: SCNPhysicsContact) {
        didBeginContact(context, Unmanaged.passUnretained(contact).toOpaque())
    }

    func physicsWorld(_ world: SCNPhysicsWorld, didUpdate contact: SCNPhysicsContact) {
        didUpdateContact(context, Unmanaged.passUnretained(contact).toOpaque())
    }

    func physicsWorld(_ world: SCNPhysicsWorld, didEnd contact: SCNPhysicsContact) {
        didEndContact(context, Unmanaged.passUnretained(contact).toOpaque())
    }

    func invokeDidBegin(contact: SCNPhysicsContact?) {
        didBeginContact(context, contact.map { Unmanaged.passUnretained($0).toOpaque() })
    }

    func invokeDidUpdate(contact: SCNPhysicsContact?) {
        didUpdateContact(context, contact.map { Unmanaged.passUnretained($0).toOpaque() })
    }

    func invokeDidEnd(contact: SCNPhysicsContact?) {
        didEndContact(context, contact.map { Unmanaged.passUnretained($0).toOpaque() })
    }
}

@_cdecl("scn_physics_contact_delegate_new")
public func scn_physics_contact_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: ScnReleaseContextCallback?,
    _ didBeginContact: @escaping PhysicsContactCallback,
    _ didUpdateContact: @escaping PhysicsContactCallback,
    _ didEndContact: @escaping PhysicsContactCallback
) -> UnsafeMutableRawPointer? {
    scnRetain(PhysicsContactDelegateBox(
        context: context,
        releaseContext: releaseContext,
        didBeginContact: didBeginContact,
        didUpdateContact: didUpdateContact,
        didEndContact: didEndContact
    ))
}

@_cdecl("scn_scene_physics_world")
public func scn_scene_physics_world(_ sceneHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let scene: SCNScene = scnBorrow(sceneHandle) else { return nil }
    return scnRetain(scene.physicsWorld)
}

@_cdecl("scn_physics_world_get_gravity")
public func scn_physics_world_get_gravity(_ worldHandle: UnsafeMutableRawPointer?, _ outVector: UnsafeMutableRawPointer?) -> Bool {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle) else { return false }
    return scnWriteVector3(world.gravity, out: outVector)
}

@_cdecl("scn_physics_world_set_gravity")
public func scn_physics_world_set_gravity(_ worldHandle: UnsafeMutableRawPointer?, _ gravity: UnsafeMutableRawPointer?) {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle),
          let gravity = scnReadVector3(gravity)
    else { return }
    world.gravity = gravity
}

@_cdecl("scn_physics_world_get_speed")
public func scn_physics_world_get_speed(_ worldHandle: UnsafeMutableRawPointer?) -> Double {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle) else { return 0 }
    return world.speed
}

@_cdecl("scn_physics_world_set_speed")
public func scn_physics_world_set_speed(_ worldHandle: UnsafeMutableRawPointer?, _ speed: Double) {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle) else { return }
    world.speed = speed
}

@_cdecl("scn_physics_world_get_time_step")
public func scn_physics_world_get_time_step(_ worldHandle: UnsafeMutableRawPointer?) -> Double {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle) else { return 0 }
    return world.timeStep
}

@_cdecl("scn_physics_world_set_time_step")
public func scn_physics_world_set_time_step(_ worldHandle: UnsafeMutableRawPointer?, _ timeStep: Double) {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle) else { return }
    world.timeStep = timeStep
}

@_cdecl("scn_physics_world_set_contact_delegate")
public func scn_physics_world_set_contact_delegate(_ worldHandle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle) else { return }
    world.contactDelegate = scnBorrow(delegateHandle)
}

@_cdecl("scn_physics_world_update_collision_pairs")
public func scn_physics_world_update_collision_pairs(_ worldHandle: UnsafeMutableRawPointer?) {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle) else { return }
    world.updateCollisionPairs()
}

@_cdecl("scn_physics_world_contact_test_with_body_count")
public func scn_physics_world_contact_test_with_body_count(_ worldHandle: UnsafeMutableRawPointer?, _ bodyHandle: UnsafeMutableRawPointer?) -> Int {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle),
          let body: SCNPhysicsBody = scnBorrow(bodyHandle)
    else { return 0 }
    return world.contactTest(with: body, options: nil).count
}

@_cdecl("scn_physics_world_contact_test_between_bodies_count")
public func scn_physics_world_contact_test_between_bodies_count(_ worldHandle: UnsafeMutableRawPointer?, _ bodyAHandle: UnsafeMutableRawPointer?, _ bodyBHandle: UnsafeMutableRawPointer?) -> Int {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle),
          let bodyA: SCNPhysicsBody = scnBorrow(bodyAHandle),
          let bodyB: SCNPhysicsBody = scnBorrow(bodyBHandle)
    else { return 0 }
    return world.contactTestBetween(bodyA, bodyB, options: nil).count
}

@_cdecl("scn_physics_contact_get_node_a")
public func scn_physics_contact_get_node_a(_ contactHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contact: SCNPhysicsContact = scnBorrow(contactHandle) else { return nil }
    return scnRetain(contact.nodeA)
}

@_cdecl("scn_physics_contact_get_node_b")
public func scn_physics_contact_get_node_b(_ contactHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let contact: SCNPhysicsContact = scnBorrow(contactHandle) else { return nil }
    return scnRetain(contact.nodeB)
}

@_cdecl("scn_physics_contact_get_contact_point")
public func scn_physics_contact_get_contact_point(_ contactHandle: UnsafeMutableRawPointer?, _ outVector: UnsafeMutableRawPointer?) -> Bool {
    guard let contact: SCNPhysicsContact = scnBorrow(contactHandle) else { return false }
    return scnWriteVector3(contact.contactPoint, out: outVector)
}

@_cdecl("scn_physics_contact_get_contact_normal")
public func scn_physics_contact_get_contact_normal(_ contactHandle: UnsafeMutableRawPointer?, _ outVector: UnsafeMutableRawPointer?) -> Bool {
    guard let contact: SCNPhysicsContact = scnBorrow(contactHandle) else { return false }
    return scnWriteVector3(contact.contactNormal, out: outVector)
}

@_cdecl("scn_physics_contact_get_collision_impulse")
public func scn_physics_contact_get_collision_impulse(_ contactHandle: UnsafeMutableRawPointer?) -> Double {
    guard let contact: SCNPhysicsContact = scnBorrow(contactHandle) else { return 0 }
    return contact.collisionImpulse
}

@_cdecl("scn_physics_contact_get_penetration_distance")
public func scn_physics_contact_get_penetration_distance(_ contactHandle: UnsafeMutableRawPointer?) -> Double {
    guard let contact: SCNPhysicsContact = scnBorrow(contactHandle) else { return 0 }
    return contact.penetrationDistance
}

@_cdecl("scn_physics_contact_get_sweep_test_fraction")
public func scn_physics_contact_get_sweep_test_fraction(_ contactHandle: UnsafeMutableRawPointer?) -> Double {
    guard let contact: SCNPhysicsContact = scnBorrow(contactHandle) else { return 0 }
    return contact.sweepTestFraction
}

@_cdecl("scn_physics_world_test_invoke_delegate_did_begin")
public func scn_physics_world_test_invoke_delegate_did_begin(_ worldHandle: UnsafeMutableRawPointer?) {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle),
          let delegate = world.contactDelegate as? PhysicsContactDelegateBox
    else { return }
    delegate.invokeDidBegin(contact: nil)
}

@_cdecl("scn_physics_world_test_invoke_delegate_did_update")
public func scn_physics_world_test_invoke_delegate_did_update(_ worldHandle: UnsafeMutableRawPointer?) {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle),
          let delegate = world.contactDelegate as? PhysicsContactDelegateBox
    else { return }
    delegate.invokeDidUpdate(contact: nil)
}

@_cdecl("scn_physics_world_test_invoke_delegate_did_end")
public func scn_physics_world_test_invoke_delegate_did_end(_ worldHandle: UnsafeMutableRawPointer?) {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle),
          let delegate = world.contactDelegate as? PhysicsContactDelegateBox
    else { return }
    delegate.invokeDidEnd(contact: nil)
}
