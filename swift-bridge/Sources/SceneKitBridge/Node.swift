import SceneKit

@_cdecl("scn_node_new")
public func scn_node_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNNode())
}

@_cdecl("scn_node_new_with_geometry")
public func scn_node_new_with_geometry(_ geometryHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let geometry: SCNGeometry? = scnBorrow(geometryHandle)
    return scnRetain(SCNNode(geometry: geometry))
}

@_cdecl("scn_node_add_child")
public func scn_node_add_child(_ parentHandle: UnsafeMutableRawPointer?, _ childHandle: UnsafeMutableRawPointer?) {
    guard let parent: SCNNode = scnBorrow(parentHandle),
          let child: SCNNode = scnBorrow(childHandle)
    else { return }
    parent.addChildNode(child)
}

@_cdecl("scn_node_remove_from_parent")
public func scn_node_remove_from_parent(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    node.removeFromParentNode()
}

@_cdecl("scn_node_copy_name")
public func scn_node_copy_name(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return nil }
    return scnDup(node.name)
}

@_cdecl("scn_node_set_name")
public func scn_node_set_name(_ nodeHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    node.name = name.map(String.init(cString:))
}

@_cdecl("scn_node_get_transform")
public func scn_node_get_transform(_ nodeHandle: UnsafeMutableRawPointer?, _ outMatrix: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return false }
    return scnWriteMatrix4(node.transform, out: outMatrix)
}

@_cdecl("scn_node_set_transform")
public func scn_node_set_transform(_ nodeHandle: UnsafeMutableRawPointer?, _ matrixHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle), let matrix = scnReadMatrix4(matrixHandle) else { return }
    node.transform = matrix
}

@_cdecl("scn_node_get_position")
public func scn_node_get_position(_ nodeHandle: UnsafeMutableRawPointer?, _ outVector: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return false }
    return scnWriteVector3(node.position, out: outVector)
}

@_cdecl("scn_node_set_position")
public func scn_node_set_position(_ nodeHandle: UnsafeMutableRawPointer?, _ vectorHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle), let vector = scnReadVector3(vectorHandle) else { return }
    node.position = vector
}

@_cdecl("scn_node_get_rotation")
public func scn_node_get_rotation(_ nodeHandle: UnsafeMutableRawPointer?, _ outVector: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return false }
    return scnWriteVector4(node.rotation, out: outVector)
}

@_cdecl("scn_node_set_rotation")
public func scn_node_set_rotation(_ nodeHandle: UnsafeMutableRawPointer?, _ vectorHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle), let vector = scnReadVector4(vectorHandle) else { return }
    node.rotation = vector
}

@_cdecl("scn_node_get_scale")
public func scn_node_get_scale(_ nodeHandle: UnsafeMutableRawPointer?, _ outVector: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return false }
    return scnWriteVector3(node.scale, out: outVector)
}

@_cdecl("scn_node_set_scale")
public func scn_node_set_scale(_ nodeHandle: UnsafeMutableRawPointer?, _ vectorHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle), let vector = scnReadVector3(vectorHandle) else { return }
    node.scale = vector
}

@_cdecl("scn_node_get_euler_angles")
public func scn_node_get_euler_angles(_ nodeHandle: UnsafeMutableRawPointer?, _ outVector: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return false }
    return scnWriteVector3(node.eulerAngles, out: outVector)
}

@_cdecl("scn_node_set_euler_angles")
public func scn_node_set_euler_angles(_ nodeHandle: UnsafeMutableRawPointer?, _ vectorHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle), let vector = scnReadVector3(vectorHandle) else { return }
    node.eulerAngles = vector
}

@_cdecl("scn_node_get_pivot")
public func scn_node_get_pivot(_ nodeHandle: UnsafeMutableRawPointer?, _ outMatrix: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return false }
    return scnWriteMatrix4(node.pivot, out: outMatrix)
}

@_cdecl("scn_node_set_pivot")
public func scn_node_set_pivot(_ nodeHandle: UnsafeMutableRawPointer?, _ matrixHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle), let matrix = scnReadMatrix4(matrixHandle) else { return }
    node.pivot = matrix
}

@_cdecl("scn_node_get_hidden")
public func scn_node_get_hidden(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return false }
    return node.isHidden
}

@_cdecl("scn_node_set_hidden")
public func scn_node_set_hidden(_ nodeHandle: UnsafeMutableRawPointer?, _ hidden: Bool) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    node.isHidden = hidden
}

@_cdecl("scn_node_get_geometry")
public func scn_node_get_geometry(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let geometry = node.geometry else { return nil }
    return scnRetain(geometry)
}

@_cdecl("scn_node_set_geometry")
public func scn_node_set_geometry(_ nodeHandle: UnsafeMutableRawPointer?, _ geometryHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    let geometry: SCNGeometry? = scnBorrow(geometryHandle)
    node.geometry = geometry
}

@_cdecl("scn_node_get_light")
public func scn_node_get_light(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let light = node.light else { return nil }
    return scnRetain(light)
}

@_cdecl("scn_node_set_light")
public func scn_node_set_light(_ nodeHandle: UnsafeMutableRawPointer?, _ lightHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    let light: SCNLight? = scnBorrow(lightHandle)
    node.light = light
}

@_cdecl("scn_node_get_camera")
public func scn_node_get_camera(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let camera = node.camera else { return nil }
    return scnRetain(camera)
}

@_cdecl("scn_node_set_camera")
public func scn_node_set_camera(_ nodeHandle: UnsafeMutableRawPointer?, _ cameraHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    let camera: SCNCamera? = scnBorrow(cameraHandle)
    node.camera = camera
}

@_cdecl("scn_node_get_physics_body")
public func scn_node_get_physics_body(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let body = node.physicsBody else { return nil }
    return scnRetain(body)
}

@_cdecl("scn_node_set_physics_body")
public func scn_node_set_physics_body(_ nodeHandle: UnsafeMutableRawPointer?, _ physicsBodyHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    let physicsBody: SCNPhysicsBody? = scnBorrow(physicsBodyHandle)
    node.physicsBody = physicsBody
}

@_cdecl("scn_node_run_action")
public func scn_node_run_action(_ nodeHandle: UnsafeMutableRawPointer?, _ actionHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle),
          let action: SCNAction = scnBorrow(actionHandle)
    else { return }
    node.runAction(action)
}

@_cdecl("scn_node_child_nodes")
public func scn_node_child_nodes(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return nil }
    return scnRetain(node.childNodes as NSArray)
}

@_cdecl("scn_node_get_parent")
public func scn_node_get_parent(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let parent = node.parent else { return nil }
    return scnRetain(parent)
}

@_cdecl("scn_node_child_node_with_name")
public func scn_node_child_node_with_name(
    _ nodeHandle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ recursively: Bool
) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let name,
          let child = node.childNode(withName: String(cString: name), recursively: recursively)
    else { return nil }
    return scnRetain(child)
}

@_cdecl("scn_node_clone")
public func scn_node_clone(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return nil }
    let clone = node.clone()
    scnAdoptRendererDelegates(from: node, to: clone)
    return scnRetain(clone)
}

@_cdecl("scn_node_get_world_transform")
public func scn_node_get_world_transform(_ nodeHandle: UnsafeMutableRawPointer?, _ outMatrix: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return false }
    return scnWriteMatrix4(node.worldTransform, out: outMatrix)
}

@_cdecl("scn_node_set_world_transform")
public func scn_node_set_world_transform(_ nodeHandle: UnsafeMutableRawPointer?, _ matrixHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle), let matrix = scnReadMatrix4(matrixHandle) else { return }
    node.setWorldTransform(matrix)
}

@_cdecl("scn_node_get_opacity")
public func scn_node_get_opacity(_ nodeHandle: UnsafeMutableRawPointer?) -> Double {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return 1 }
    return Double(node.opacity)
}

@_cdecl("scn_node_set_opacity")
public func scn_node_set_opacity(_ nodeHandle: UnsafeMutableRawPointer?, _ opacity: Double) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    node.opacity = CGFloat(opacity)
}

@_cdecl("scn_node_get_category_bit_mask")
public func scn_node_get_category_bit_mask(_ nodeHandle: UnsafeMutableRawPointer?) -> UInt {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return 0 }
    return UInt(bitPattern: node.categoryBitMask)
}

@_cdecl("scn_node_set_category_bit_mask")
public func scn_node_set_category_bit_mask(_ nodeHandle: UnsafeMutableRawPointer?, _ mask: UInt) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    node.categoryBitMask = Int(bitPattern: mask)
}
