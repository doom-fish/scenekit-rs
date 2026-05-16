import Foundation
import ModelIO
import SceneKit

@_cdecl("scn_geometry_new_box")
public func scn_geometry_new_box(
    _ width: Double,
    _ height: Double,
    _ length: Double,
    _ chamferRadius: Double
) -> UnsafeMutableRawPointer? {
    scnRetain(SCNBox(width: width, height: height, length: length, chamferRadius: chamferRadius))
}

@_cdecl("scn_geometry_new_sphere")
public func scn_geometry_new_sphere(_ radius: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNSphere(radius: radius))
}

@_cdecl("scn_geometry_new_cylinder")
public func scn_geometry_new_cylinder(_ radius: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNCylinder(radius: radius, height: height))
}

@_cdecl("scn_geometry_new_cone")
public func scn_geometry_new_cone(
    _ topRadius: Double,
    _ bottomRadius: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    scnRetain(SCNCone(topRadius: topRadius, bottomRadius: bottomRadius, height: height))
}

@_cdecl("scn_geometry_new_plane")
public func scn_geometry_new_plane(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNPlane(width: width, height: height))
}

@_cdecl("scn_geometry_new_floor")
public func scn_geometry_new_floor() -> UnsafeMutableRawPointer? {
    scnRetain(SCNFloor())
}

@_cdecl("scn_geometry_new_text")
public func scn_geometry_new_text(
    _ string: UnsafePointer<CChar>?,
    _ extrusionDepth: Double
) -> UnsafeMutableRawPointer? {
    guard let string else { return nil }
    return scnRetain(SCNText(string: String(cString: string), extrusionDepth: extrusionDepth))
}

@_cdecl("scn_geometry_new_from_mdl_mesh")
public func scn_geometry_new_from_mdl_mesh(_ meshHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let mesh = scnBorrowMDLMesh(meshHandle) else { return nil }
    let selector = NSSelectorFromString("geometryWithMDLMesh:")
    guard (SCNGeometry.self as AnyObject).responds(to: selector),
          let unmanaged = (SCNGeometry.self as AnyObject).perform(selector, with: mesh),
          let geometry = unmanaged.takeUnretainedValue() as? SCNGeometry
    else {
        return nil
    }
    return scnRetain(geometry)
}

@_cdecl("scn_geometry_first_material")
public func scn_geometry_first_material(_ geometryHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle) else { return nil }
    if geometry.firstMaterial == nil {
        geometry.firstMaterial = SCNMaterial()
    }
    guard let material = geometry.firstMaterial else { return nil }
    return scnRetain(material)
}

@_cdecl("scn_geometry_set_first_material")
public func scn_geometry_set_first_material(
    _ geometryHandle: UnsafeMutableRawPointer?,
    _ materialHandle: UnsafeMutableRawPointer?
) {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle) else { return }
    let material: SCNMaterial? = scnBorrow(materialHandle)
    geometry.firstMaterial = material
}
