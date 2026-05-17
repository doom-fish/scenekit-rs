import AppKit
import Foundation
import QuartzCore
import SceneKit

@_cdecl("scn_geometry_new_pyramid")
public func scn_geometry_new_pyramid(_ width: Double, _ height: Double, _ length: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNPyramid(width: width, height: height, length: length))
}

@_cdecl("scn_geometry_new_tube")
public func scn_geometry_new_tube(_ innerRadius: Double, _ outerRadius: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNTube(innerRadius: innerRadius, outerRadius: outerRadius, height: height))
}

@_cdecl("scn_geometry_new_capsule")
public func scn_geometry_new_capsule(_ capRadius: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNCapsule(capRadius: capRadius, height: height))
}

@_cdecl("scn_geometry_new_torus")
public func scn_geometry_new_torus(_ ringRadius: Double, _ pipeRadius: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNTorus(ringRadius: ringRadius, pipeRadius: pipeRadius))
}

@_cdecl("scn_geometry_new_shape")
public func scn_geometry_new_shape(_ extrusionDepth: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNShape(path: nil, extrusionDepth: extrusionDepth))
}

@_cdecl("scn_geometry_source_new_vertices")
public func scn_geometry_source_new_vertices(_ vertices: UnsafeRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    guard let vertices else { return nil }
    let typed = vertices.bindMemory(to: SCNVector3.self, capacity: count)
    return scnRetain(SCNGeometrySource(__vertices: typed, count: count))
}

@_cdecl("scn_geometry_source_new_normals")
public func scn_geometry_source_new_normals(_ normals: UnsafeRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    guard let normals else { return nil }
    let typed = normals.bindMemory(to: SCNVector3.self, capacity: count)
    return scnRetain(SCNGeometrySource(__normals: typed, count: count))
}

@_cdecl("scn_geometry_source_new_texcoords")
public func scn_geometry_source_new_texcoords(_ texcoords: UnsafeRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    guard let texcoords else { return nil }
    let typed = texcoords.bindMemory(to: CGPoint.self, capacity: count)
    return scnRetain(SCNGeometrySource(__textureCoordinates: typed, count: count))
}

@_cdecl("scn_geometry_element_new")
public func scn_geometry_element_new(
    _ data: UnsafeRawPointer?,
    _ length: Int,
    _ primitiveType: Int32,
    _ primitiveCount: Int,
    _ bytesPerIndex: Int
) -> UnsafeMutableRawPointer? {
    let nsData = data.map { Data(bytes: $0, count: length) }
    let primitive = SCNGeometryPrimitiveType(rawValue: Int(primitiveType)) ?? .triangles
    return scnRetain(
        SCNGeometryElement(
            data: nsData,
            primitiveType: primitive,
            primitiveCount: primitiveCount,
            bytesPerIndex: bytesPerIndex
        )
    )
}

@_cdecl("scn_geometry_new_with_sources_elements")
public func scn_geometry_new_with_sources_elements(
    _ sourcesHandle: UnsafeMutableRawPointer?,
    _ sourceCount: Int,
    _ elementsHandle: UnsafeMutableRawPointer?,
    _ elementCount: Int
) -> UnsafeMutableRawPointer? {
    let sourcePtr = sourcesHandle?.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    let elementPtr = elementsHandle?.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    let sources = (0..<sourceCount).compactMap { index -> SCNGeometrySource? in
        guard let sourcePtr else { return nil }
        return scnBorrow(sourcePtr[index])
    }
    let elements = (0..<elementCount).compactMap { index -> SCNGeometryElement? in
        guard let elementPtr else { return nil }
        return scnBorrow(elementPtr[index])
    }
    return scnRetain(SCNGeometry(sources: sources, elements: elements))
}

@_cdecl("scn_geometry_tessellator_new")
public func scn_geometry_tessellator_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNGeometryTessellator())
}

@_cdecl("scn_geometry_get_tessellator")
public func scn_geometry_get_tessellator(_ geometryHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle), let tessellator = geometry.tessellator else { return nil }
    return scnRetain(tessellator)
}

@_cdecl("scn_geometry_set_tessellator")
public func scn_geometry_set_tessellator(_ geometryHandle: UnsafeMutableRawPointer?, _ tessellatorHandle: UnsafeMutableRawPointer?) {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle) else { return }
    let tessellator: SCNGeometryTessellator? = scnBorrow(tessellatorHandle)
    geometry.tessellator = tessellator
}

@_cdecl("scn_level_of_detail_new_screen_space")
public func scn_level_of_detail_new_screen_space(_ geometryHandle: UnsafeMutableRawPointer?, _ radius: Double) -> UnsafeMutableRawPointer? {
    let geometry: SCNGeometry? = scnBorrow(geometryHandle)
    return scnRetain(SCNLevelOfDetail(geometry: geometry, screenSpaceRadius: radius))
}

@_cdecl("scn_level_of_detail_new_world_space")
public func scn_level_of_detail_new_world_space(_ geometryHandle: UnsafeMutableRawPointer?, _ distance: Double) -> UnsafeMutableRawPointer? {
    let geometry: SCNGeometry? = scnBorrow(geometryHandle)
    return scnRetain(SCNLevelOfDetail(geometry: geometry, worldSpaceDistance: distance))
}

@_cdecl("scn_geometry_set_levels_of_detail")
public func scn_geometry_set_levels_of_detail(_ geometryHandle: UnsafeMutableRawPointer?, _ lodsHandle: UnsafeMutableRawPointer?, _ count: Int) {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle) else { return }
    let ptr = lodsHandle?.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    geometry.levelsOfDetail = (0..<count).compactMap { index -> SCNLevelOfDetail? in
        guard let ptr else { return nil }
        return scnBorrow(ptr[index])
    }
}

@_cdecl("scn_geometry_levels_of_detail_count")
public func scn_geometry_levels_of_detail_count(_ geometryHandle: UnsafeMutableRawPointer?) -> Int {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle) else { return 0 }
    return geometry.levelsOfDetail?.count ?? 0
}

@_cdecl("scn_morpher_new")
public func scn_morpher_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNMorpher())
}

@_cdecl("scn_morpher_get_calculation_mode")
public func scn_morpher_get_calculation_mode(_ morpherHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let morpher: SCNMorpher = scnBorrow(morpherHandle) else { return 0 }
    return Int32(morpher.calculationMode.rawValue)
}

@_cdecl("scn_morpher_set_calculation_mode")
public func scn_morpher_set_calculation_mode(_ morpherHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let morpher: SCNMorpher = scnBorrow(morpherHandle) else { return }
    morpher.calculationMode = SCNMorpherCalculationMode(rawValue: Int(mode)) ?? .normalized
}

@_cdecl("scn_node_get_morpher")
public func scn_node_get_morpher(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let morpher = node.morpher else { return nil }
    return scnRetain(morpher)
}

@_cdecl("scn_node_set_morpher")
public func scn_node_set_morpher(_ nodeHandle: UnsafeMutableRawPointer?, _ morpherHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    let morpher: SCNMorpher? = scnBorrow(morpherHandle)
    node.morpher = morpher
}

@_cdecl("scn_particle_property_controller_new_with_animation")
public func scn_particle_property_controller_new_with_animation(_ animationHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let animation: CAAnimation = scnBorrow(animationHandle) else { return nil }
    return scnRetain(SCNParticlePropertyController(animation: animation))
}

@_cdecl("scn_particle_property_controller_get_input_mode")
public func scn_particle_property_controller_get_input_mode(_ controllerHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let controller: SCNParticlePropertyController = scnBorrow(controllerHandle) else { return 0 }
    return Int32(controller.inputMode.rawValue)
}

@_cdecl("scn_particle_property_controller_set_input_mode")
public func scn_particle_property_controller_set_input_mode(_ controllerHandle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let controller: SCNParticlePropertyController = scnBorrow(controllerHandle) else { return }
    controller.inputMode = SCNParticleInputMode(rawValue: Int(mode)) ?? .overLife
}

@_cdecl("scn_reference_node_new_url")
public func scn_reference_node_new_url(_ path: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    guard let node = SCNReferenceNode(url: URL(fileURLWithPath: String(cString: path))) else { return nil }
    return scnRetain(node)
}

@_cdecl("scn_reference_node_get_loading_policy")
public func scn_reference_node_get_loading_policy(_ nodeHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let node: SCNReferenceNode = scnBorrow(nodeHandle) else { return 0 }
    return Int32(node.loadingPolicy.rawValue)
}

@_cdecl("scn_reference_node_set_loading_policy")
public func scn_reference_node_set_loading_policy(_ nodeHandle: UnsafeMutableRawPointer?, _ policy: Int32) {
    guard let node: SCNReferenceNode = scnBorrow(nodeHandle) else { return }
    node.loadingPolicy = SCNReferenceLoadingPolicy(rawValue: Int(policy)) ?? .immediate
}

@_cdecl("scn_reference_node_load")
public func scn_reference_node_load(_ nodeHandle: UnsafeMutableRawPointer?) {
    (scnBorrow(nodeHandle) as SCNReferenceNode?)?.load()
}

@_cdecl("scn_reference_node_unload")
public func scn_reference_node_unload(_ nodeHandle: UnsafeMutableRawPointer?) {
    (scnBorrow(nodeHandle) as SCNReferenceNode?)?.unload()
}

@_cdecl("scn_reference_node_get_loaded")
public func scn_reference_node_get_loaded(_ nodeHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let node: SCNReferenceNode = scnBorrow(nodeHandle) else { return false }
    return node.isLoaded
}

@_cdecl("scn_skinner_new")
public func scn_skinner_new(
    _ baseGeometryHandle: UnsafeMutableRawPointer?,
    _ bonesHandle: UnsafeMutableRawPointer?,
    _ boneCount: Int,
    _ boneWeightsHandle: UnsafeMutableRawPointer?,
    _ boneIndicesHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    let baseGeometry: SCNGeometry? = scnBorrow(baseGeometryHandle)
    let ptr = bonesHandle?.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    let bones = (0..<boneCount).compactMap { index -> SCNNode? in
        guard let ptr else { return nil }
        return scnBorrow(ptr[index])
    }
    guard let boneWeights: SCNGeometrySource = scnBorrow(boneWeightsHandle), let boneIndices: SCNGeometrySource = scnBorrow(boneIndicesHandle) else { return nil }
    return scnRetain(SCNSkinner(baseGeometry: baseGeometry, bones: bones, boneInverseBindTransforms: nil, boneWeights: boneWeights, boneIndices: boneIndices))
}

@_cdecl("scn_node_get_skinner")
public func scn_node_get_skinner(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let skinner = node.skinner else { return nil }
    return scnRetain(skinner)
}

@_cdecl("scn_node_set_skinner")
public func scn_node_set_skinner(_ nodeHandle: UnsafeMutableRawPointer?, _ skinnerHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    let skinner: SCNSkinner? = scnBorrow(skinnerHandle)
    node.skinner = skinner
}
