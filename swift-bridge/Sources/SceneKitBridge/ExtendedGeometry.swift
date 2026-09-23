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
    guard let vertices, count > 0 else { return nil }
    // The Rust side marshals `Vector3` as three 32-bit `Float`s (12 bytes),
    // whereas `SCNVector3` on macOS is three `CGFloat`s (24 bytes). Binding the
    // buffer directly to `SCNVector3` would read 24 bytes per element from a
    // 12-byte-per-element allocation (out-of-bounds read + garbage values), so
    // read `Float` triples and convert, matching `scnReadVector3` in Core.swift.
    let floats = vertices.assumingMemoryBound(to: Float.self)
    let typed = (0..<count).map { index in
        SCNVector3(
            x: CGFloat(floats[index * 3]),
            y: CGFloat(floats[index * 3 + 1]),
            z: CGFloat(floats[index * 3 + 2])
        )
    }
    return scnRetain(SCNGeometrySource(vertices: typed))
}

@_cdecl("scn_geometry_source_new_normals")
public func scn_geometry_source_new_normals(_ normals: UnsafeRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    guard let normals, count > 0 else { return nil }
    // See `scn_geometry_source_new_vertices`: the Rust `Vector3` buffer is
    // `Float`-packed, so convert rather than binding to the wider `SCNVector3`.
    let floats = normals.assumingMemoryBound(to: Float.self)
    let typed = (0..<count).map { index in
        SCNVector3(
            x: CGFloat(floats[index * 3]),
            y: CGFloat(floats[index * 3 + 1]),
            z: CGFloat(floats[index * 3 + 2])
        )
    }
    return scnRetain(SCNGeometrySource(normals: typed))
}

private func scnGeometrySourceSemantic(_ rawValue: Int32) -> SCNGeometrySource.Semantic? {
    switch rawValue {
    case 0: return .vertex
    case 1: return .normal
    case 2: return .color
    case 3: return .texcoord
    case 4: return .tangent
    case 5: return .vertexCrease
    case 6: return .edgeCrease
    case 7: return .boneWeights
    case 8: return .boneIndices
    default: return nil
    }
}

@_cdecl("scn_geometry_source_new_data")
public func scn_geometry_source_new_data(
    _ bytes: UnsafeRawPointer?,
    _ length: Int,
    _ semanticRaw: Int32,
    _ layout: UnsafePointer<Int>?,
    _ usesFloatComponents: Bool
) -> UnsafeMutableRawPointer? {
    guard let bytes, let layout, length > 0, let semantic = scnGeometrySourceSemantic(semanticRaw) else { return nil }
    let vectorCount = layout[0]
    let componentsPerVector = layout[1]
    let bytesPerComponent = layout[2]
    let dataOffset = layout[3]
    let dataStride = layout[4]
    guard vectorCount > 0, (1...4).contains(componentsPerVector), [1, 2, 4, 8].contains(bytesPerComponent), dataOffset >= 0 else { return nil }
    let vectorLength = componentsPerVector * bytesPerComponent
    guard dataStride >= vectorLength else { return nil }
    let (span, spanOverflow) = (vectorCount - 1).multipliedReportingOverflow(by: dataStride)
    let (withOffset, offsetOverflow) = span.addingReportingOverflow(dataOffset)
    let (required, requiredOverflow) = withOffset.addingReportingOverflow(vectorLength)
    guard !spanOverflow, !offsetOverflow, !requiredOverflow, required <= length else { return nil }
    let source = SCNGeometrySource(
        data: Data(bytes: bytes, count: length),
        semantic: semantic,
        vectorCount: vectorCount,
        usesFloatComponents: usesFloatComponents,
        componentsPerVector: componentsPerVector,
        bytesPerComponent: bytesPerComponent,
        dataOffset: dataOffset,
        dataStride: dataStride
    )
    return scnRetain(source)
}

@_cdecl("scn_geometry_source_get_vector_count")
public func scn_geometry_source_get_vector_count(_ sourceHandle: UnsafeMutableRawPointer?) -> Int {
    guard let source: SCNGeometrySource = scnBorrow(sourceHandle) else { return 0 }
    return source.vectorCount
}

private func scnReadUnsigned(_ bytes: UnsafeRawBufferPointer, offset: Int, width: Int) -> Int? {
    guard offset >= 0, width > 0, width <= bytes.count, offset <= bytes.count - width else { return nil }
    switch width {
    case 1: return Int(bytes.load(fromByteOffset: offset, as: UInt8.self))
    case 2: return Int(bytes.loadUnaligned(fromByteOffset: offset, as: UInt16.self))
    case 4: return Int(bytes.loadUnaligned(fromByteOffset: offset, as: UInt32.self))
    default: return nil
    }
}

private func scnImplicitIndexCount(_ primitiveType: SCNGeometryPrimitiveType, _ primitiveCount: Int) -> Int? {
    switch primitiveType {
    case .triangles:
        let (count, overflow) = primitiveCount.multipliedReportingOverflow(by: 3)
        return overflow ? nil : count
    case .triangleStrip:
        if primitiveCount == 0 { return 0 }
        let (count, overflow) = primitiveCount.addingReportingOverflow(2)
        return overflow ? nil : count
    case .line:
        let (count, overflow) = primitiveCount.multipliedReportingOverflow(by: 2)
        return overflow ? nil : count
    case .point:
        return primitiveCount
    case .polygon:
        return primitiveCount == 0 ? 0 : nil
    @unknown default:
        return nil
    }
}

private func scnElementVertexRequirement(_ element: SCNGeometryElement) -> Int? {
    let width = element.bytesPerIndex
    let primitiveCount = element.primitiveCount
    guard width == 1 || width == 2 || width == 4, primitiveCount >= 0 else { return nil }
    if #available(macOS 13.0, *) {
        guard element.indicesChannelCount == 1 else { return nil }
    }
    let data = element.data
    if data.isEmpty {
        return scnImplicitIndexCount(element.primitiveType, primitiveCount)
    }
    return data.withUnsafeBytes { bytes -> Int? in
        var firstIndex = 0
        var indexCount: Int
        if element.primitiveType == .polygon {
            var total = 0
            for polygon in 0..<primitiveCount {
                guard let vertices = scnReadUnsigned(bytes, offset: polygon * width, width: width), vertices >= 3 else { return nil }
                let (sum, overflow) = total.addingReportingOverflow(vertices)
                guard !overflow else { return nil }
                total = sum
            }
            firstIndex = primitiveCount
            indexCount = total
        } else {
            guard let count = scnImplicitIndexCount(element.primitiveType, primitiveCount) else { return nil }
            indexCount = count
        }
        let (end, endOverflow) = firstIndex.addingReportingOverflow(indexCount)
        let (endBytes, bytesOverflow) = end.multipliedReportingOverflow(by: width)
        guard !endOverflow, !bytesOverflow, endBytes <= bytes.count else { return nil }
        var required = 0
        for position in firstIndex..<end {
            guard let index = scnReadUnsigned(bytes, offset: position * width, width: width) else { return nil }
            required = max(required, index + 1)
        }
        return required
    }
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
    guard bytesPerIndex == 1 || bytesPerIndex == 2 || bytesPerIndex == 4, primitiveCount >= 0, length >= 0,
          let primitive = SCNGeometryPrimitiveType(rawValue: Int(primitiveType))
    else { return nil }
    let nsData = data.map { Data(bytes: $0, count: length) }
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
    _ elementCount: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    outError?.pointee = nil
    guard sourceCount >= 0, elementCount >= 0 else { return nil }
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
    let vertexLimit = sources.map(\.vectorCount).min() ?? 0
    for (position, element) in elements.enumerated() {
        guard let required = scnElementVertexRequirement(element) else {
            outError?.pointee = scnDup("geometry element \(position) has an unsupported or malformed index layout")
            return nil
        }
        guard required <= vertexLimit else {
            outError?.pointee = scnDup("geometry element \(position) references vertex \(required - 1) but the sources hold \(vertexLimit) vectors")
            return nil
        }
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
    return Int32(clamping: morpher.calculationMode.rawValue)
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
    return Int32(clamping: controller.inputMode.rawValue)
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
    return Int32(clamping: node.loadingPolicy.rawValue)
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

private func scnValidateBoneIndices(_ source: SCNGeometrySource, boneCount: Int) -> String? {
    guard !source.usesFloatComponents else { return "bone indices must use integer components" }
    let width = source.bytesPerComponent
    guard width == 1 || width == 2 || width == 4 else { return "bone indices must be 1, 2 or 4 bytes wide" }
    let vectorCount = source.vectorCount
    let components = source.componentsPerVector
    let offset = source.dataOffset
    let stride = source.dataStride
    guard vectorCount >= 0, components >= 1, offset >= 0, stride >= 0 else { return "bone index source has an invalid layout" }
    return source.data.withUnsafeBytes { bytes -> String? in
        for vector in 0..<vectorCount {
            let (base, baseOverflow) = vector.multipliedReportingOverflow(by: stride)
            let (start, startOverflow) = base.addingReportingOverflow(offset)
            guard !baseOverflow, !startOverflow else { return "bone index source layout overflows" }
            for component in 0..<components {
                let (componentOffset, componentOverflow) = component.multipliedReportingOverflow(by: width)
                let (position, overflow) = start.addingReportingOverflow(componentOffset)
                guard !componentOverflow, !overflow, let index = scnReadUnsigned(bytes, offset: position, width: width) else {
                    return "bone index data is shorter than its layout"
                }
                guard index < boneCount else {
                    return "bone index \(index) of vertex \(vector) is out of range for \(boneCount) bones"
                }
            }
        }
        return nil
    }
}

@_cdecl("scn_skinner_new")
public func scn_skinner_new(
    _ baseGeometryHandle: UnsafeMutableRawPointer?,
    _ bonesHandle: UnsafeMutableRawPointer?,
    _ boneCount: Int,
    _ inverseBindTransforms: UnsafeMutableRawPointer?,
    _ boneWeightsHandle: UnsafeMutableRawPointer?,
    _ boneIndicesHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    outError?.pointee = nil
    guard boneCount >= 0 else { return nil }
    guard let baseGeometry: SCNGeometry = scnBorrow(baseGeometryHandle) else {
        outError?.pointee = scnDup("a skinner needs a base geometry")
        return nil
    }
    let ptr = bonesHandle?.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    let bones = (0..<boneCount).compactMap { index -> SCNNode? in
        guard let ptr else { return nil }
        return scnBorrow(ptr[index])
    }
    guard let boneWeights: SCNGeometrySource = scnBorrow(boneWeightsHandle), let boneIndices: SCNGeometrySource = scnBorrow(boneIndicesHandle) else {
        outError?.pointee = scnDup("missing bone weight or bone index source")
        return nil
    }
    guard !bones.isEmpty, bones.count == boneCount else {
        outError?.pointee = scnDup("a skinner needs at least one bone node")
        return nil
    }
    guard boneWeights.usesFloatComponents else {
        outError?.pointee = scnDup("bone weights must use float components")
        return nil
    }
    guard boneWeights.vectorCount == boneIndices.vectorCount, boneWeights.componentsPerVector == boneIndices.componentsPerVector else {
        outError?.pointee = scnDup("bone weights and bone indices must have the same vector count and components per vector")
        return nil
    }
    if let vertices = baseGeometry.sources(for: .vertex).first, vertices.vectorCount != boneWeights.vectorCount {
        outError?.pointee = scnDup("the base geometry has \(vertices.vectorCount) vertices but the bone sources have \(boneWeights.vectorCount) vectors")
        return nil
    }
    if let problem = scnValidateBoneIndices(boneIndices, boneCount: bones.count) {
        outError?.pointee = scnDup(problem)
        return nil
    }
    let transforms: [NSValue] = (0..<bones.count).map { index in
        if let inverseBindTransforms, let matrix = scnReadMatrix4(inverseBindTransforms.advanced(by: index * 16 * MemoryLayout<Float>.stride)) {
            return NSValue(scnMatrix4: matrix)
        }
        return NSValue(scnMatrix4: SCNMatrix4Invert(bones[index].worldTransform))
    }
    let skinner = SCNSkinner(baseGeometry: baseGeometry, bones: bones, boneInverseBindTransforms: transforms, boneWeights: boneWeights, boneIndices: boneIndices)
    guard unsafeBitCast(skinner, to: UnsafeRawPointer?.self) != nil else {
        outError?.pointee = scnDup("SceneKit rejected the skinner inputs")
        return nil
    }
    return scnRetain(skinner)
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
