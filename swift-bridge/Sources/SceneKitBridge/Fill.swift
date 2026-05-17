import AppKit
import Foundation
import GLKit
import SceneKit

@_cdecl("scn_constant_lookup_extra")
public func scn_constant_lookup_extra(_ name: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let name else { return nil }
    switch String(cString: name) {
    case "SCNErrorDomain":
        return scnDup(SCNErrorDomain)
    case "SCNGeometrySourceSemanticBoneIndices":
        return scnDup(SCNGeometrySource.Semantic.boneIndices.rawValue)
    case "SCNGeometrySourceSemanticBoneWeights":
        return scnDup(SCNGeometrySource.Semantic.boneWeights.rawValue)
    case "SCNGeometrySourceSemanticColor":
        return scnDup(SCNGeometrySource.Semantic.color.rawValue)
    case "SCNGeometrySourceSemanticEdgeCrease":
        return scnDup(SCNGeometrySource.Semantic.edgeCrease.rawValue)
    case "SCNGeometrySourceSemanticNormal":
        return scnDup(SCNGeometrySource.Semantic.normal.rawValue)
    case "SCNGeometrySourceSemanticTangent":
        return scnDup(SCNGeometrySource.Semantic.tangent.rawValue)
    case "SCNGeometrySourceSemanticTexcoord":
        return scnDup(SCNGeometrySource.Semantic.texcoord.rawValue)
    case "SCNGeometrySourceSemanticVertex":
        return scnDup(SCNGeometrySource.Semantic.vertex.rawValue)
    case "SCNGeometrySourceSemanticVertexCrease":
        return scnDup(SCNGeometrySource.Semantic.vertexCrease.rawValue)
    case "SCNHitTestBackFaceCullingKey":
        return scnDup(SCNHitTestOption.backFaceCulling.rawValue)
    case "SCNHitTestBoundingBoxOnlyKey":
        return scnDup(SCNHitTestOption.boundingBoxOnly.rawValue)
    case "SCNHitTestClipToZRangeKey":
        return scnDup(SCNHitTestOption.clipToZRange.rawValue)
    case "SCNHitTestFirstFoundOnlyKey":
        return scnDup(SCNHitTestOption.firstFoundOnly.rawValue)
    case "SCNHitTestIgnoreChildNodesKey":
        return scnDup(SCNHitTestOption.ignoreChildNodes.rawValue)
    case "SCNHitTestIgnoreHiddenNodesKey":
        return scnDup(SCNHitTestOption.ignoreHiddenNodes.rawValue)
    case "SCNHitTestOptionCategoryBitMask":
        return scnDup(SCNHitTestOption.categoryBitMask.rawValue)
    case "SCNHitTestOptionIgnoreLightArea":
        return scnDup(SCNHitTestOption.ignoreLightArea.rawValue)
    case "SCNHitTestOptionSearchMode":
        return scnDup(SCNHitTestOption.searchMode.rawValue)
    case "SCNHitTestRootNodeKey":
        return scnDup(SCNHitTestOption.rootNode.rawValue)
    case "SCNHitTestSortResultsKey":
        return scnDup(SCNHitTestOption.sortResults.rawValue)
    case "SCNLightingModelBlinn":
        return scnDup(SCNMaterial.LightingModel.blinn.rawValue)
    case "SCNLightingModelConstant":
        return scnDup(SCNMaterial.LightingModel.constant.rawValue)
    case "SCNLightingModelLambert":
        return scnDup(SCNMaterial.LightingModel.lambert.rawValue)
    case "SCNLightingModelPhong":
        return scnDup(SCNMaterial.LightingModel.phong.rawValue)
    case "SCNLightingModelPhysicallyBased":
        return scnDup(SCNMaterial.LightingModel.physicallyBased.rawValue)
    case "SCNLightingModelShadowOnly":
        return scnDup(SCNMaterial.LightingModel.shadowOnly.rawValue)
    case "SCNModelTransform":
        return scnDup(SCNModelTransform)
    case "SCNModelViewProjectionTransform":
        return scnDup(SCNModelViewProjectionTransform)
    case "SCNModelViewTransform":
        return scnDup(SCNModelViewTransform)
    case "SCNNormalTransform":
        return scnDup(SCNNormalTransform)
    case "SCNProjectionTransform":
        return scnDup(SCNProjectionTransform)
    case "SCNViewTransform":
        return scnDup(SCNViewTransform)
    case "SCNParticlePropertyAngle":
        return scnDup(SCNParticleSystem.ParticleProperty.angle.rawValue)
    case "SCNParticlePropertyAngularVelocity":
        return scnDup(SCNParticleSystem.ParticleProperty.angularVelocity.rawValue)
    case "SCNParticlePropertyBounce":
        return scnDup(SCNParticleSystem.ParticleProperty.bounce.rawValue)
    case "SCNParticlePropertyCharge":
        return scnDup(SCNParticleSystem.ParticleProperty.charge.rawValue)
    case "SCNParticlePropertyColor":
        return scnDup(SCNParticleSystem.ParticleProperty.color.rawValue)
    case "SCNParticlePropertyContactNormal":
        return scnDup(SCNParticleSystem.ParticleProperty.contactNormal.rawValue)
    case "SCNParticlePropertyContactPoint":
        return scnDup(SCNParticleSystem.ParticleProperty.contactPoint.rawValue)
    case "SCNParticlePropertyFrame":
        return scnDup(SCNParticleSystem.ParticleProperty.frame.rawValue)
    case "SCNParticlePropertyFrameRate":
        return scnDup(SCNParticleSystem.ParticleProperty.frameRate.rawValue)
    case "SCNParticlePropertyFriction":
        return scnDup(SCNParticleSystem.ParticleProperty.friction.rawValue)
    case "SCNParticlePropertyLife":
        return scnDup(SCNParticleSystem.ParticleProperty.life.rawValue)
    case "SCNParticlePropertyOpacity":
        return scnDup(SCNParticleSystem.ParticleProperty.opacity.rawValue)
    case "SCNParticlePropertyPosition":
        return scnDup(SCNParticleSystem.ParticleProperty.position.rawValue)
    case "SCNParticlePropertyRotationAxis":
        return scnDup(SCNParticleSystem.ParticleProperty.rotationAxis.rawValue)
    case "SCNParticlePropertySize":
        return scnDup(SCNParticleSystem.ParticleProperty.size.rawValue)
    case "SCNParticlePropertyVelocity":
        return scnDup(SCNParticleSystem.ParticleProperty.velocity.rawValue)
    case "SCNPhysicsShapeKeepAsCompoundKey":
        return scnDup(SCNPhysicsShape.Option.keepAsCompound.rawValue)
    case "SCNPhysicsShapeOptionCollisionMargin":
        return scnDup(SCNPhysicsShape.Option.collisionMargin.rawValue)
    case "SCNPhysicsShapeScaleKey":
        return scnDup(SCNPhysicsShape.Option.scale.rawValue)
    case "SCNPhysicsShapeTypeBoundingBox":
        return scnDup(SCNPhysicsShape.ShapeType.boundingBox.rawValue)
    case "SCNPhysicsShapeTypeConcavePolyhedron":
        return scnDup(SCNPhysicsShape.ShapeType.concavePolyhedron.rawValue)
    case "SCNPhysicsShapeTypeConvexHull":
        return scnDup(SCNPhysicsShape.ShapeType.convexHull.rawValue)
    case "SCNPhysicsShapeTypeKey":
        return scnDup(SCNPhysicsShape.Option.type.rawValue)
    case "SCNPreferLowPowerDeviceKey":
        return scnDup(SCNView.Option.preferLowPowerDevice.rawValue)
    case "SCNPreferredDeviceKey":
        return scnDup(SCNView.Option.preferredDevice.rawValue)
    case "SCNPreferredRenderingAPIKey":
        return scnDup(SCNView.Option.preferredRenderingAPI.rawValue)
    case "SCNSceneEndTimeAttributeKey":
        return scnDup(SCNScene.Attribute.endTime.rawValue)
    case "SCNSceneExportDestinationURL":
        return scnDup(SCNSceneExportDestinationURL)
    case "SCNSceneFrameRateAttributeKey":
        return scnDup(SCNScene.Attribute.frameRate.rawValue)
    case "SCNSceneStartTimeAttributeKey":
        return scnDup(SCNScene.Attribute.startTime.rawValue)
    case "SCNSceneUpAxisAttributeKey":
        return scnDup(SCNScene.Attribute.upAxis.rawValue)
    default:
        return nil
    }
}

@_cdecl("scn_extra_vector3_equal")
public func scn_extra_vector3_equal(_ a: UnsafeMutableRawPointer?, _ b: UnsafeMutableRawPointer?) -> Bool {
    guard let a = scnReadVector3(a), let b = scnReadVector3(b) else { return false }
    return SCNVector3EqualToVector3(a, b)
}

@_cdecl("scn_extra_vector4_equal")
public func scn_extra_vector4_equal(_ a: UnsafeMutableRawPointer?, _ b: UnsafeMutableRawPointer?) -> Bool {
    guard let a = scnReadVector4(a), let b = scnReadVector4(b) else { return false }
    return SCNVector4EqualToVector4(a, b)
}

@_cdecl("scn_extra_matrix4_equal")
public func scn_extra_matrix4_equal(_ a: UnsafeMutableRawPointer?, _ b: UnsafeMutableRawPointer?) -> Bool {
    guard let a = scnReadMatrix4(a), let b = scnReadMatrix4(b) else { return false }
    return SCNMatrix4EqualToMatrix4(a, b)
}

@_cdecl("scn_extra_matrix4_is_identity")
public func scn_extra_matrix4_is_identity(_ matrix: UnsafeMutableRawPointer?) -> Bool {
    guard let matrix = scnReadMatrix4(matrix) else { return false }
    return SCNMatrix4IsIdentity(matrix)
}

@_cdecl("scn_extra_matrix4_make_rotation")
public func scn_extra_matrix4_make_rotation(
    _ angle: Float,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ outMatrix: UnsafeMutableRawPointer?
) {
    _ = scnWriteMatrix4(
        SCNMatrix4MakeRotation(CGFloat(angle), CGFloat(x), CGFloat(y), CGFloat(z)),
        out: outMatrix
    )
}

@_cdecl("scn_extra_matrix4_scale")
public func scn_extra_matrix4_scale(
    _ matrix: UnsafeMutableRawPointer?,
    _ sx: Float,
    _ sy: Float,
    _ sz: Float,
    _ outMatrix: UnsafeMutableRawPointer?
) {
    guard let matrix = scnReadMatrix4(matrix) else { return }
    _ = scnWriteMatrix4(SCNMatrix4Scale(matrix, CGFloat(sx), CGFloat(sy), CGFloat(sz)), out: outMatrix)
}

@_cdecl("scn_extra_matrix4_rotate")
public func scn_extra_matrix4_rotate(
    _ matrix: UnsafeMutableRawPointer?,
    _ angle: Float,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ outMatrix: UnsafeMutableRawPointer?
) {
    guard let matrix = scnReadMatrix4(matrix) else { return }
    _ = scnWriteMatrix4(
        SCNMatrix4Rotate(matrix, CGFloat(angle), CGFloat(x), CGFloat(y), CGFloat(z)),
        out: outMatrix
    )
}

@_cdecl("scn_extra_matrix4_invert")
public func scn_extra_matrix4_invert(_ matrix: UnsafeMutableRawPointer?, _ outMatrix: UnsafeMutableRawPointer?) {
    guard let matrix = scnReadMatrix4(matrix) else { return }
    _ = scnWriteMatrix4(SCNMatrix4Invert(matrix), out: outMatrix)
}

@_cdecl("scn_extra_matrix4_mult")
public func scn_extra_matrix4_mult(
    _ a: UnsafeMutableRawPointer?,
    _ b: UnsafeMutableRawPointer?,
    _ outMatrix: UnsafeMutableRawPointer?
) {
    guard let a = scnReadMatrix4(a), let b = scnReadMatrix4(b) else { return }
    _ = scnWriteMatrix4(SCNMatrix4Mult(a, b), out: outMatrix)
}

@_cdecl("scn_extra_matrix4_to_glk")
public func scn_extra_matrix4_to_glk(_ matrix: UnsafeMutableRawPointer?, _ outElements: UnsafeMutablePointer<Float>?) {
    guard let matrix = scnReadMatrix4(matrix), let outElements else { return }
    let glk = SCNMatrix4ToGLKMatrix4(matrix)
    withUnsafeBytes(of: glk.m) { rawBuffer in
        let floats = rawBuffer.bindMemory(to: Float.self)
        for index in 0..<16 {
            outElements[index] = floats[index]
        }
    }
}

@_cdecl("scn_extra_matrix4_from_glk")
public func scn_extra_matrix4_from_glk(_ elements: UnsafePointer<Float>?, _ outMatrix: UnsafeMutableRawPointer?) {
    guard let elements else { return }
    let glk = GLKMatrix4(m: (
        elements[0], elements[1], elements[2], elements[3],
        elements[4], elements[5], elements[6], elements[7],
        elements[8], elements[9], elements[10], elements[11],
        elements[12], elements[13], elements[14], elements[15]
    ))
    _ = scnWriteMatrix4(SCNMatrix4FromGLKMatrix4(glk), out: outMatrix)
}
