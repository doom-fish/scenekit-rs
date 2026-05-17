import Foundation
import SceneKit

@_cdecl("scn_billboard_constraint_new")
public func scn_billboard_constraint_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNBillboardConstraint())
}

@_cdecl("scn_billboard_constraint_get_free_axes")
public func scn_billboard_constraint_get_free_axes(_ constraintHandle: UnsafeMutableRawPointer?) -> UInt {
    guard let constraint: SCNBillboardConstraint = scnBorrow(constraintHandle) else { return 0 }
    return constraint.freeAxes.rawValue
}

@_cdecl("scn_billboard_constraint_set_free_axes")
public func scn_billboard_constraint_set_free_axes(_ constraintHandle: UnsafeMutableRawPointer?, _ freeAxes: UInt) {
    guard let constraint: SCNBillboardConstraint = scnBorrow(constraintHandle) else { return }
    constraint.freeAxes = SCNBillboardAxis(rawValue: freeAxes)
}

@_cdecl("scn_transform_constraint_new_passthrough")
public func scn_transform_constraint_new_passthrough(_ worldSpace: Bool) -> UnsafeMutableRawPointer? {
    scnRetain(SCNTransformConstraint(inWorldSpace: worldSpace) { _, transform in transform })
}

@_cdecl("scn_ik_constraint_new")
public func scn_ik_constraint_new(_ chainRootNodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let chainRootNode: SCNNode = scnBorrow(chainRootNodeHandle) else { return nil }
    return scnRetain(SCNIKConstraint(chainRootNode: chainRootNode))
}

@_cdecl("scn_ik_constraint_get_target_position")
public func scn_ik_constraint_get_target_position(_ constraintHandle: UnsafeMutableRawPointer?, _ outVector: UnsafeMutableRawPointer?) -> Bool {
    guard let constraint: SCNIKConstraint = scnBorrow(constraintHandle) else { return false }
    return scnWriteVector3(constraint.targetPosition, out: outVector)
}

@_cdecl("scn_ik_constraint_set_target_position")
public func scn_ik_constraint_set_target_position(_ constraintHandle: UnsafeMutableRawPointer?, _ targetPositionHandle: UnsafeMutableRawPointer?) {
    guard let constraint: SCNIKConstraint = scnBorrow(constraintHandle), let targetPosition = scnReadVector3(targetPositionHandle) else { return }
    constraint.targetPosition = targetPosition
}

@_cdecl("scn_replicator_constraint_new")
public func scn_replicator_constraint_new(_ targetHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let target: SCNNode? = scnBorrow(targetHandle)
    return scnRetain(SCNReplicatorConstraint(target: target))
}

@_cdecl("scn_replicator_constraint_get_replicates_position")
public func scn_replicator_constraint_get_replicates_position(_ constraintHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let constraint: SCNReplicatorConstraint = scnBorrow(constraintHandle) else { return false }
    return constraint.replicatesPosition
}

@_cdecl("scn_replicator_constraint_set_replicates_position")
public func scn_replicator_constraint_set_replicates_position(_ constraintHandle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let constraint: SCNReplicatorConstraint = scnBorrow(constraintHandle) else { return }
    constraint.replicatesPosition = value
}

@_cdecl("scn_replicator_constraint_get_replicates_orientation")
public func scn_replicator_constraint_get_replicates_orientation(_ constraintHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let constraint: SCNReplicatorConstraint = scnBorrow(constraintHandle) else { return false }
    return constraint.replicatesOrientation
}

@_cdecl("scn_replicator_constraint_set_replicates_orientation")
public func scn_replicator_constraint_set_replicates_orientation(_ constraintHandle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let constraint: SCNReplicatorConstraint = scnBorrow(constraintHandle) else { return }
    constraint.replicatesOrientation = value
}

@_cdecl("scn_replicator_constraint_get_replicates_scale")
public func scn_replicator_constraint_get_replicates_scale(_ constraintHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let constraint: SCNReplicatorConstraint = scnBorrow(constraintHandle) else { return false }
    return constraint.replicatesScale
}

@_cdecl("scn_replicator_constraint_set_replicates_scale")
public func scn_replicator_constraint_set_replicates_scale(_ constraintHandle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let constraint: SCNReplicatorConstraint = scnBorrow(constraintHandle) else { return }
    constraint.replicatesScale = value
}

@_cdecl("scn_acceleration_constraint_new")
public func scn_acceleration_constraint_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNAccelerationConstraint())
}

@_cdecl("scn_acceleration_constraint_get_maximum_linear_acceleration")
public func scn_acceleration_constraint_get_maximum_linear_acceleration(_ constraintHandle: UnsafeMutableRawPointer?) -> Double {
    guard let constraint: SCNAccelerationConstraint = scnBorrow(constraintHandle) else { return 0 }
    return constraint.maximumLinearAcceleration
}

@_cdecl("scn_acceleration_constraint_set_maximum_linear_acceleration")
public func scn_acceleration_constraint_set_maximum_linear_acceleration(_ constraintHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let constraint: SCNAccelerationConstraint = scnBorrow(constraintHandle) else { return }
    constraint.maximumLinearAcceleration = value
}

@_cdecl("scn_slider_constraint_new")
public func scn_slider_constraint_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNSliderConstraint())
}

@_cdecl("scn_slider_constraint_get_collision_category_bit_mask")
public func scn_slider_constraint_get_collision_category_bit_mask(_ constraintHandle: UnsafeMutableRawPointer?) -> UInt {
    guard let constraint: SCNSliderConstraint = scnBorrow(constraintHandle) else { return 0 }
    return UInt(constraint.collisionCategoryBitMask)
}

@_cdecl("scn_slider_constraint_set_collision_category_bit_mask")
public func scn_slider_constraint_set_collision_category_bit_mask(_ constraintHandle: UnsafeMutableRawPointer?, _ mask: UInt) {
    guard let constraint: SCNSliderConstraint = scnBorrow(constraintHandle) else { return }
    constraint.collisionCategoryBitMask = Int(mask)
}

@_cdecl("scn_avoid_occluder_constraint_new")
public func scn_avoid_occluder_constraint_new(_ targetHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let target: SCNNode? = scnBorrow(targetHandle)
    return scnRetain(SCNAvoidOccluderConstraint(target: target))
}

@_cdecl("scn_avoid_occluder_constraint_get_target")
public func scn_avoid_occluder_constraint_get_target(_ constraintHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle), let target = constraint.target else { return nil }
    return scnRetain(target)
}

@_cdecl("scn_avoid_occluder_constraint_set_target")
public func scn_avoid_occluder_constraint_set_target(_ constraintHandle: UnsafeMutableRawPointer?, _ targetHandle: UnsafeMutableRawPointer?) {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle) else { return }
    let target: SCNNode? = scnBorrow(targetHandle)
    constraint.target = target
}

@_cdecl("scn_avoid_occluder_constraint_get_occluder_category_bit_mask")
public func scn_avoid_occluder_constraint_get_occluder_category_bit_mask(_ constraintHandle: UnsafeMutableRawPointer?) -> UInt {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle) else { return 0 }
    return UInt(constraint.occluderCategoryBitMask)
}

@_cdecl("scn_avoid_occluder_constraint_set_occluder_category_bit_mask")
public func scn_avoid_occluder_constraint_set_occluder_category_bit_mask(_ constraintHandle: UnsafeMutableRawPointer?, _ mask: UInt) {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle) else { return }
    constraint.occluderCategoryBitMask = Int(mask)
}

@_cdecl("scn_avoid_occluder_constraint_get_bias")
public func scn_avoid_occluder_constraint_get_bias(_ constraintHandle: UnsafeMutableRawPointer?) -> Double {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle) else { return 0 }
    return constraint.bias
}

@_cdecl("scn_avoid_occluder_constraint_set_bias")
public func scn_avoid_occluder_constraint_set_bias(_ constraintHandle: UnsafeMutableRawPointer?, _ bias: Double) {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle) else { return }
    constraint.bias = bias
}
