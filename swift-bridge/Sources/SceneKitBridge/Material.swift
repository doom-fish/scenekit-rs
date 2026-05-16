import Foundation
import Metal
import SceneKit

@_cdecl("scn_material_new")
public func scn_material_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNMaterial())
}

@_cdecl("scn_material_diffuse")
public func scn_material_diffuse(_ materialHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let material: SCNMaterial = scnBorrow(materialHandle) else { return nil }
    return scnRetain(material.diffuse)
}

@_cdecl("scn_material_normal")
public func scn_material_normal(_ materialHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let material: SCNMaterial = scnBorrow(materialHandle) else { return nil }
    return scnRetain(material.normal)
}

@_cdecl("scn_material_specular")
public func scn_material_specular(_ materialHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let material: SCNMaterial = scnBorrow(materialHandle) else { return nil }
    return scnRetain(material.specular)
}

@_cdecl("scn_material_emission")
public func scn_material_emission(_ materialHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let material: SCNMaterial = scnBorrow(materialHandle) else { return nil }
    return scnRetain(material.emission)
}

@_cdecl("scn_material_ambient")
public func scn_material_ambient(_ materialHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let material: SCNMaterial = scnBorrow(materialHandle) else { return nil }
    return scnRetain(material.ambient)
}

@_cdecl("scn_material_transparent")
public func scn_material_transparent(_ materialHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let material: SCNMaterial = scnBorrow(materialHandle) else { return nil }
    return scnRetain(material.transparent)
}

@_cdecl("scn_material_multiply")
public func scn_material_multiply(_ materialHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let material: SCNMaterial = scnBorrow(materialHandle) else { return nil }
    return scnRetain(material.multiply)
}

@_cdecl("scn_material_property_set_color")
public func scn_material_property_set_color(
    _ propertyHandle: UnsafeMutableRawPointer?,
    _ r: Float,
    _ g: Float,
    _ b: Float,
    _ a: Float
) {
    guard let property: SCNMaterialProperty = scnBorrow(propertyHandle) else { return }
    property.contents = scnMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("scn_material_property_copy_color")
public func scn_material_property_copy_color(
    _ propertyHandle: UnsafeMutableRawPointer?,
    _ outRGBA: UnsafeMutableRawPointer?
) -> Bool {
    guard let property: SCNMaterialProperty = scnBorrow(propertyHandle) else { return false }
    return scnWriteColor(property.contents, outRGBA: outRGBA)
}

@_cdecl("scn_material_property_set_cg_image")
public func scn_material_property_set_cg_image(
    _ propertyHandle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?
) {
    guard let property: SCNMaterialProperty = scnBorrow(propertyHandle),
          let image = scnBorrowCGImage(imageHandle)
    else { return }
    property.contents = image
}

@_cdecl("scn_material_property_set_metal_texture")
public func scn_material_property_set_metal_texture(
    _ propertyHandle: UnsafeMutableRawPointer?,
    _ textureHandle: UnsafeMutableRawPointer?
) {
    guard let property: SCNMaterialProperty = scnBorrow(propertyHandle),
          let texture: MTLTexture = scnBorrow(textureHandle)
    else { return }
    property.contents = texture
}

@_cdecl("scn_material_property_set_file_url")
public func scn_material_property_set_file_url(
    _ propertyHandle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?
) {
    guard let property: SCNMaterialProperty = scnBorrow(propertyHandle), let path else { return }
    property.contents = URL(fileURLWithPath: String(cString: path))
}

@_cdecl("scn_material_property_clear_contents")
public func scn_material_property_clear_contents(_ propertyHandle: UnsafeMutableRawPointer?) {
    guard let property: SCNMaterialProperty = scnBorrow(propertyHandle) else { return }
    property.contents = nil
}

@_cdecl("scn_material_property_get_intensity")
public func scn_material_property_get_intensity(_ propertyHandle: UnsafeMutableRawPointer?) -> Double {
    guard let property: SCNMaterialProperty = scnBorrow(propertyHandle) else { return 0 }
    return property.intensity
}

@_cdecl("scn_material_property_set_intensity")
public func scn_material_property_set_intensity(_ propertyHandle: UnsafeMutableRawPointer?, _ intensity: Double) {
    guard let property: SCNMaterialProperty = scnBorrow(propertyHandle) else { return }
    property.intensity = intensity
}
