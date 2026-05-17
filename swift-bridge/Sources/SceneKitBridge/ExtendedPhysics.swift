import Foundation
import SceneKit

@_cdecl("scn_physics_shape_new_with_geometry")
public func scn_physics_shape_new_with_geometry(_ geometryHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle) else { return nil }
    return scnRetain(SCNPhysicsShape(geometry: geometry, options: nil))
}

@_cdecl("scn_physics_shape_new_with_node")
public func scn_physics_shape_new_with_node(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return nil }
    return scnRetain(SCNPhysicsShape(node: node, options: nil))
}

@_cdecl("scn_physics_body_get_shape")
public func scn_physics_body_get_shape(_ bodyHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle), let shape = body.physicsShape else { return nil }
    return scnRetain(shape)
}

@_cdecl("scn_physics_body_set_shape")
public func scn_physics_body_set_shape(_ bodyHandle: UnsafeMutableRawPointer?, _ shapeHandle: UnsafeMutableRawPointer?) {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle) else { return }
    let shape: SCNPhysicsShape? = scnBorrow(shapeHandle)
    body.physicsShape = shape
}

@_cdecl("scn_physics_field_drag")
public func scn_physics_field_drag() -> UnsafeMutableRawPointer? {
    scnRetain(SCNPhysicsField.drag())
}

@_cdecl("scn_physics_field_vortex")
public func scn_physics_field_vortex() -> UnsafeMutableRawPointer? {
    scnRetain(SCNPhysicsField.vortex())
}

@_cdecl("scn_physics_field_radial_gravity")
public func scn_physics_field_radial_gravity() -> UnsafeMutableRawPointer? {
    scnRetain(SCNPhysicsField.radialGravity())
}

@_cdecl("scn_physics_field_linear_gravity")
public func scn_physics_field_linear_gravity() -> UnsafeMutableRawPointer? {
    scnRetain(SCNPhysicsField.linearGravity())
}

@_cdecl("scn_physics_field_get_scope")
public func scn_physics_field_get_scope(_ fieldHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let field: SCNPhysicsField = scnBorrow(fieldHandle) else { return 0 }
    return Int32(field.scope.rawValue)
}

@_cdecl("scn_physics_field_set_scope")
public func scn_physics_field_set_scope(_ fieldHandle: UnsafeMutableRawPointer?, _ scope: Int32) {
    guard let field: SCNPhysicsField = scnBorrow(fieldHandle) else { return }
    field.scope = SCNPhysicsFieldScope(rawValue: Int(scope)) ?? .insideExtent
}

@_cdecl("scn_physics_world_add_behavior")
public func scn_physics_world_add_behavior(_ worldHandle: UnsafeMutableRawPointer?, _ behaviorHandle: UnsafeMutableRawPointer?) {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle), let behavior: SCNPhysicsBehavior = scnBorrow(behaviorHandle) else { return }
    world.addBehavior(behavior)
}

@_cdecl("scn_physics_world_remove_behavior")
public func scn_physics_world_remove_behavior(_ worldHandle: UnsafeMutableRawPointer?, _ behaviorHandle: UnsafeMutableRawPointer?) {
    guard let world: SCNPhysicsWorld = scnBorrow(worldHandle), let behavior: SCNPhysicsBehavior = scnBorrow(behaviorHandle) else { return }
    world.removeBehavior(behavior)
}

@_cdecl("scn_physics_world_remove_all_behaviors")
public func scn_physics_world_remove_all_behaviors(_ worldHandle: UnsafeMutableRawPointer?) {
    (scnBorrow(worldHandle) as SCNPhysicsWorld?)?.removeAllBehaviors()
}

@_cdecl("scn_node_get_physics_field")
public func scn_node_get_physics_field(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let field = node.physicsField else { return nil }
    return scnRetain(field)
}

@_cdecl("scn_node_set_physics_field")
public func scn_node_set_physics_field(_ nodeHandle: UnsafeMutableRawPointer?, _ fieldHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    let field: SCNPhysicsField? = scnBorrow(fieldHandle)
    node.physicsField = field
}

@_cdecl("scn_physics_ball_socket_joint_new")
public func scn_physics_ball_socket_joint_new(
    _ bodyAHandle: UnsafeMutableRawPointer?,
    _ anchorAHandle: UnsafeMutableRawPointer?,
    _ bodyBHandle: UnsafeMutableRawPointer?,
    _ anchorBHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let bodyA: SCNPhysicsBody = scnBorrow(bodyAHandle), let anchorA = scnReadVector3(anchorAHandle), let bodyB: SCNPhysicsBody = scnBorrow(bodyBHandle), let anchorB = scnReadVector3(anchorBHandle) else { return nil }
    return scnRetain(SCNPhysicsBallSocketJoint(bodyA: bodyA, anchorA: anchorA, bodyB: bodyB, anchorB: anchorB))
}

@_cdecl("scn_physics_ball_socket_joint_new_single")
public func scn_physics_ball_socket_joint_new_single(_ bodyHandle: UnsafeMutableRawPointer?, _ anchorHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle), let anchor = scnReadVector3(anchorHandle) else { return nil }
    return scnRetain(SCNPhysicsBallSocketJoint(body: body, anchor: anchor))
}

@_cdecl("scn_physics_hinge_joint_new")
public func scn_physics_hinge_joint_new(
    _ bodyAHandle: UnsafeMutableRawPointer?,
    _ axisAHandle: UnsafeMutableRawPointer?,
    _ anchorAHandle: UnsafeMutableRawPointer?,
    _ bodyBHandle: UnsafeMutableRawPointer?,
    _ axisBHandle: UnsafeMutableRawPointer?,
    _ anchorBHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let bodyA: SCNPhysicsBody = scnBorrow(bodyAHandle), let axisA = scnReadVector3(axisAHandle), let anchorA = scnReadVector3(anchorAHandle), let bodyB: SCNPhysicsBody = scnBorrow(bodyBHandle), let axisB = scnReadVector3(axisBHandle), let anchorB = scnReadVector3(anchorBHandle) else { return nil }
    return scnRetain(SCNPhysicsHingeJoint(bodyA: bodyA, axisA: axisA, anchorA: anchorA, bodyB: bodyB, axisB: axisB, anchorB: anchorB))
}

@_cdecl("scn_physics_hinge_joint_new_single")
public func scn_physics_hinge_joint_new_single(
    _ bodyHandle: UnsafeMutableRawPointer?,
    _ axisHandle: UnsafeMutableRawPointer?,
    _ anchorHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle), let axis = scnReadVector3(axisHandle), let anchor = scnReadVector3(anchorHandle) else { return nil }
    return scnRetain(SCNPhysicsHingeJoint(body: body, axis: axis, anchor: anchor))
}

@_cdecl("scn_physics_slider_joint_new")
public func scn_physics_slider_joint_new(
    _ bodyAHandle: UnsafeMutableRawPointer?,
    _ axisAHandle: UnsafeMutableRawPointer?,
    _ anchorAHandle: UnsafeMutableRawPointer?,
    _ bodyBHandle: UnsafeMutableRawPointer?,
    _ axisBHandle: UnsafeMutableRawPointer?,
    _ anchorBHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let bodyA: SCNPhysicsBody = scnBorrow(bodyAHandle), let axisA = scnReadVector3(axisAHandle), let anchorA = scnReadVector3(anchorAHandle), let bodyB: SCNPhysicsBody = scnBorrow(bodyBHandle), let axisB = scnReadVector3(axisBHandle), let anchorB = scnReadVector3(anchorBHandle) else { return nil }
    return scnRetain(SCNPhysicsSliderJoint(bodyA: bodyA, axisA: axisA, anchorA: anchorA, bodyB: bodyB, axisB: axisB, anchorB: anchorB))
}

@_cdecl("scn_physics_slider_joint_new_single")
public func scn_physics_slider_joint_new_single(
    _ bodyHandle: UnsafeMutableRawPointer?,
    _ axisHandle: UnsafeMutableRawPointer?,
    _ anchorHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle), let axis = scnReadVector3(axisHandle), let anchor = scnReadVector3(anchorHandle) else { return nil }
    return scnRetain(SCNPhysicsSliderJoint(body: body, axis: axis, anchor: anchor))
}

@_cdecl("scn_physics_cone_twist_joint_new")
public func scn_physics_cone_twist_joint_new(
    _ bodyAHandle: UnsafeMutableRawPointer?,
    _ frameAHandle: UnsafeMutableRawPointer?,
    _ bodyBHandle: UnsafeMutableRawPointer?,
    _ frameBHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let bodyA: SCNPhysicsBody = scnBorrow(bodyAHandle), let frameA = scnReadMatrix4(frameAHandle), let bodyB: SCNPhysicsBody = scnBorrow(bodyBHandle), let frameB = scnReadMatrix4(frameBHandle) else { return nil }
    return scnRetain(SCNPhysicsConeTwistJoint(bodyA: bodyA, frameA: frameA, bodyB: bodyB, frameB: frameB))
}

@_cdecl("scn_physics_cone_twist_joint_new_single")
public func scn_physics_cone_twist_joint_new_single(_ bodyHandle: UnsafeMutableRawPointer?, _ frameHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let body: SCNPhysicsBody = scnBorrow(bodyHandle), let frame = scnReadMatrix4(frameHandle) else { return nil }
    return scnRetain(SCNPhysicsConeTwistJoint(body: body, frame: frame))
}

@_cdecl("scn_physics_vehicle_wheel_new")
public func scn_physics_vehicle_wheel_new(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return nil }
    return scnRetain(SCNPhysicsVehicleWheel(node: node))
}

@_cdecl("scn_physics_vehicle_new")
public func scn_physics_vehicle_new(
    _ chassisBodyHandle: UnsafeMutableRawPointer?,
    _ wheelsHandle: UnsafeMutableRawPointer?,
    _ wheelCount: Int
) -> UnsafeMutableRawPointer? {
    guard let chassisBody: SCNPhysicsBody = scnBorrow(chassisBodyHandle) else { return nil }
    let ptr = wheelsHandle?.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    let wheels = (0..<wheelCount).compactMap { index -> SCNPhysicsVehicleWheel? in
        guard let ptr else { return nil }
        return scnBorrow(ptr[index])
    }
    return scnRetain(SCNPhysicsVehicle(chassisBody: chassisBody, wheels: wheels))
}

@_cdecl("scn_physics_vehicle_get_speed_in_kilometers_per_hour")
public func scn_physics_vehicle_get_speed_in_kilometers_per_hour(_ vehicleHandle: UnsafeMutableRawPointer?) -> Double {
    guard let vehicle: SCNPhysicsVehicle = scnBorrow(vehicleHandle) else { return 0 }
    return vehicle.speedInKilometersPerHour
}

@_cdecl("scn_physics_vehicle_apply_engine_force")
public func scn_physics_vehicle_apply_engine_force(_ vehicleHandle: UnsafeMutableRawPointer?, _ value: Double, _ wheelIndex: Int) {
    guard let vehicle: SCNPhysicsVehicle = scnBorrow(vehicleHandle) else { return }
    vehicle.applyEngineForce(value, forWheelAt: wheelIndex)
}

@_cdecl("scn_physics_vehicle_set_steering_angle")
public func scn_physics_vehicle_set_steering_angle(_ vehicleHandle: UnsafeMutableRawPointer?, _ value: Double, _ wheelIndex: Int) {
    guard let vehicle: SCNPhysicsVehicle = scnBorrow(vehicleHandle) else { return }
    vehicle.setSteeringAngle(value, forWheelAt: wheelIndex)
}

@_cdecl("scn_physics_vehicle_apply_braking_force")
public func scn_physics_vehicle_apply_braking_force(_ vehicleHandle: UnsafeMutableRawPointer?, _ value: Double, _ wheelIndex: Int) {
    guard let vehicle: SCNPhysicsVehicle = scnBorrow(vehicleHandle) else { return }
    vehicle.applyBrakingForce(value, forWheelAt: wheelIndex)
}
