import SceneKit

@_cdecl("scn_camera_new")
public func scn_camera_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNCamera())
}

@_cdecl("scn_camera_get_field_of_view")
public func scn_camera_get_field_of_view(_ cameraHandle: UnsafeMutableRawPointer?) -> Double {
    guard let camera: SCNCamera = scnBorrow(cameraHandle) else { return 0 }
    return camera.fieldOfView
}

@_cdecl("scn_camera_set_field_of_view")
public func scn_camera_set_field_of_view(_ cameraHandle: UnsafeMutableRawPointer?, _ fieldOfView: Double) {
    guard let camera: SCNCamera = scnBorrow(cameraHandle) else { return }
    camera.fieldOfView = fieldOfView
}

@_cdecl("scn_camera_get_z_near")
public func scn_camera_get_z_near(_ cameraHandle: UnsafeMutableRawPointer?) -> Double {
    guard let camera: SCNCamera = scnBorrow(cameraHandle) else { return 0 }
    return camera.zNear
}

@_cdecl("scn_camera_set_z_near")
public func scn_camera_set_z_near(_ cameraHandle: UnsafeMutableRawPointer?, _ zNear: Double) {
    guard let camera: SCNCamera = scnBorrow(cameraHandle) else { return }
    camera.zNear = zNear
}

@_cdecl("scn_camera_get_z_far")
public func scn_camera_get_z_far(_ cameraHandle: UnsafeMutableRawPointer?) -> Double {
    guard let camera: SCNCamera = scnBorrow(cameraHandle) else { return 0 }
    return camera.zFar
}

@_cdecl("scn_camera_set_z_far")
public func scn_camera_set_z_far(_ cameraHandle: UnsafeMutableRawPointer?, _ zFar: Double) {
    guard let camera: SCNCamera = scnBorrow(cameraHandle) else { return }
    camera.zFar = zFar
}

@_cdecl("scn_camera_get_projection_transform")
public func scn_camera_get_projection_transform(
    _ cameraHandle: UnsafeMutableRawPointer?,
    _ outMatrix: UnsafeMutableRawPointer?
) -> Bool {
    guard let camera: SCNCamera = scnBorrow(cameraHandle) else { return false }
    return scnWriteMatrix4(camera.projectionTransform, out: outMatrix)
}

@_cdecl("scn_camera_set_projection_transform")
public func scn_camera_set_projection_transform(
    _ cameraHandle: UnsafeMutableRawPointer?,
    _ matrix: UnsafeMutableRawPointer?
) {
    guard let camera: SCNCamera = scnBorrow(cameraHandle),
          let matrix = scnReadMatrix4(matrix)
    else { return }
    camera.projectionTransform = matrix
}
