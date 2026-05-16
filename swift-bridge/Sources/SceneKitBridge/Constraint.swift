import SceneKit

func scnConstraints(from rawConstraints: UnsafeMutableRawPointer?, count: Int) -> [SCNConstraint] {
    guard let rawConstraints else { return [] }
    let pointers = rawConstraints.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    var constraints: [SCNConstraint] = []
    constraints.reserveCapacity(count)
    for index in 0..<count {
        if let constraint: SCNConstraint = scnBorrow(pointers[index]) {
            constraints.append(constraint)
        }
    }
    return constraints
}

@_cdecl("scn_constraint_new_look_at")
public func scn_constraint_new_look_at(_ targetHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let target: SCNNode? = scnBorrow(targetHandle)
    return scnRetain(SCNLookAtConstraint(target: target))
}

@_cdecl("scn_constraint_new_distance")
public func scn_constraint_new_distance(_ targetHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let target: SCNNode? = scnBorrow(targetHandle)
    return scnRetain(SCNDistanceConstraint(target: target))
}

@_cdecl("scn_constraint_get_influence_factor")
public func scn_constraint_get_influence_factor(_ constraintHandle: UnsafeMutableRawPointer?) -> Double {
    guard let constraint: SCNConstraint = scnBorrow(constraintHandle) else { return 0 }
    return constraint.influenceFactor
}

@_cdecl("scn_constraint_set_influence_factor")
public func scn_constraint_set_influence_factor(_ constraintHandle: UnsafeMutableRawPointer?, _ influenceFactor: Double) {
    guard let constraint: SCNConstraint = scnBorrow(constraintHandle) else { return }
    constraint.influenceFactor = CGFloat(influenceFactor)
}

@_cdecl("scn_constraint_get_gimbal_lock_enabled")
public func scn_constraint_get_gimbal_lock_enabled(_ constraintHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let constraint: SCNLookAtConstraint = scnBorrow(constraintHandle) else { return false }
    return constraint.isGimbalLockEnabled
}

@_cdecl("scn_constraint_set_gimbal_lock_enabled")
public func scn_constraint_set_gimbal_lock_enabled(_ constraintHandle: UnsafeMutableRawPointer?, _ gimbalLockEnabled: Bool) {
    guard let constraint: SCNLookAtConstraint = scnBorrow(constraintHandle) else { return }
    constraint.isGimbalLockEnabled = gimbalLockEnabled
}

@_cdecl("scn_constraint_get_minimum_distance")
public func scn_constraint_get_minimum_distance(_ constraintHandle: UnsafeMutableRawPointer?) -> Double {
    guard let constraint: SCNDistanceConstraint = scnBorrow(constraintHandle) else { return 0 }
    return constraint.minimumDistance
}

@_cdecl("scn_constraint_set_minimum_distance")
public func scn_constraint_set_minimum_distance(_ constraintHandle: UnsafeMutableRawPointer?, _ minimumDistance: Double) {
    guard let constraint: SCNDistanceConstraint = scnBorrow(constraintHandle) else { return }
    constraint.minimumDistance = CGFloat(minimumDistance)
}

@_cdecl("scn_constraint_get_maximum_distance")
public func scn_constraint_get_maximum_distance(_ constraintHandle: UnsafeMutableRawPointer?) -> Double {
    guard let constraint: SCNDistanceConstraint = scnBorrow(constraintHandle) else { return 0 }
    return constraint.maximumDistance
}

@_cdecl("scn_constraint_set_maximum_distance")
public func scn_constraint_set_maximum_distance(_ constraintHandle: UnsafeMutableRawPointer?, _ maximumDistance: Double) {
    guard let constraint: SCNDistanceConstraint = scnBorrow(constraintHandle) else { return }
    constraint.maximumDistance = CGFloat(maximumDistance)
}

@_cdecl("scn_node_set_constraints")
public func scn_node_set_constraints(
    _ nodeHandle: UnsafeMutableRawPointer?,
    _ constraintsHandle: UnsafeMutableRawPointer?,
    _ count: Int
) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    node.constraints = scnConstraints(from: constraintsHandle, count: count)
}

@_cdecl("scn_node_constraints_count")
public func scn_node_constraints_count(_ nodeHandle: UnsafeMutableRawPointer?) -> Int {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return 0 }
    return node.constraints?.count ?? 0
}
