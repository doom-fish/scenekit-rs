import SceneKit

@_cdecl("scn_constant_lookup")
public func scn_constant_lookup(_ name: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let name else { return nil }
    switch String(cString: name) {
    case "SCNConsistencyElementIDErrorKey":
        return scnDup(SCNConsistencyElementIDErrorKey)
    case "SCNConsistencyElementTypeErrorKey":
        return scnDup(SCNConsistencyElementTypeErrorKey)
    case "SCNConsistencyLineNumberErrorKey":
        return scnDup(SCNConsistencyLineNumberErrorKey)
    case "SCNDetailedErrorsKey":
        return scnDup(SCNDetailedErrorsKey)
    case "SCNPhysicsTestBackfaceCullingKey":
        return scnDup(SCNPhysicsWorld.TestOption.backfaceCulling.rawValue)
    case "SCNPhysicsTestCollisionBitMaskKey":
        return scnDup(SCNPhysicsWorld.TestOption.collisionBitMask.rawValue)
    case "SCNPhysicsTestSearchModeAll":
        return scnDup(SCNPhysicsWorld.TestSearchMode.all.rawValue)
    case "SCNPhysicsTestSearchModeAny":
        return scnDup(SCNPhysicsWorld.TestSearchMode.any.rawValue)
    case "SCNPhysicsTestSearchModeClosest":
        return scnDup(SCNPhysicsWorld.TestSearchMode.closest.rawValue)
    case "SCNPhysicsTestSearchModeKey":
        return scnDup(SCNPhysicsWorld.TestOption.searchMode.rawValue)
    case "SCNProgramMappingChannelKey":
        return scnDup(SCNProgramMappingChannelKey)
    case "SCNSceneSourceAnimationImportPolicyDoNotPlay":
        return scnDup(SCNSceneSource.AnimationImportPolicy.doNotPlay.rawValue)
    case "SCNSceneSourceAnimationImportPolicyKey":
        return scnDup(SCNSceneSource.LoadingOption.animationImportPolicy.rawValue)
    case "SCNSceneSourceAnimationImportPolicyPlay":
        return scnDup(SCNSceneSource.AnimationImportPolicy.play.rawValue)
    case "SCNSceneSourceAnimationImportPolicyPlayRepeatedly":
        return scnDup(SCNSceneSource.AnimationImportPolicy.playRepeatedly.rawValue)
    case "SCNSceneSourceAnimationImportPolicyPlayUsingSceneTimeBase":
        return scnDup(SCNSceneSource.AnimationImportPolicy.playUsingSceneTimeBase.rawValue)
    case "SCNSceneSourceAssetAuthorKey":
        return scnDup(SCNSceneSourceAssetAuthorKey)
    case "SCNSceneSourceAssetAuthoringToolKey":
        return scnDup(SCNSceneSourceAssetAuthoringToolKey)
    case "SCNSceneSourceAssetContributorsKey":
        return scnDup(SCNSceneSourceAssetContributorsKey)
    case "SCNSceneSourceAssetCreatedDateKey":
        return scnDup(SCNSceneSourceAssetCreatedDateKey)
    case "SCNSceneSourceAssetDirectoryURLsKey":
        return scnDup(SCNSceneSource.LoadingOption.assetDirectoryURLs.rawValue)
    case "SCNSceneSourceAssetModifiedDateKey":
        return scnDup(SCNSceneSourceAssetModifiedDateKey)
    case "SCNSceneSourceAssetUnitKey":
        return scnDup(SCNSceneSourceAssetUnitKey)
    case "SCNSceneSourceAssetUnitMeterKey":
        return scnDup(SCNSceneSourceAssetUnitMeterKey)
    case "SCNSceneSourceAssetUnitNameKey":
        return scnDup(SCNSceneSourceAssetUnitNameKey)
    case "SCNSceneSourceAssetUpAxisKey":
        return scnDup(SCNSceneSourceAssetUpAxisKey)
    case "SCNSceneSourceCheckConsistencyKey":
        return scnDup(SCNSceneSource.LoadingOption.checkConsistency.rawValue)
    case "SCNSceneSourceConvertToYUpKey":
        return scnDup(SCNSceneSource.LoadingOption.convertToYUp.rawValue)
    case "SCNSceneSourceConvertUnitsToMetersKey":
        return scnDup(SCNSceneSource.LoadingOption.convertUnitsToMeters.rawValue)
    case "SCNSceneSourceCreateNormalsIfAbsentKey":
        return scnDup(SCNSceneSource.LoadingOption.createNormalsIfAbsent.rawValue)
    case "SCNSceneSourceFlattenSceneKey":
        return scnDup(SCNSceneSource.LoadingOption.flattenScene.rawValue)
    case "SCNSceneSourceLoadingOptionPreserveOriginalTopology":
        return scnDup(SCNSceneSource.LoadingOption.preserveOriginalTopology.rawValue)
    case "SCNSceneSourceOverrideAssetURLsKey":
        return scnDup(SCNSceneSource.LoadingOption.overrideAssetURLs.rawValue)
    case "SCNSceneSourceStrictConformanceKey":
        return scnDup(SCNSceneSource.LoadingOption.strictConformance.rawValue)
    case "SCNShaderModifierEntryPointFragment":
        return scnDup(SCNShaderModifierEntryPoint.fragment.rawValue)
    case "SCNShaderModifierEntryPointGeometry":
        return scnDup(SCNShaderModifierEntryPoint.geometry.rawValue)
    case "SCNShaderModifierEntryPointLightingModel":
        return scnDup(SCNShaderModifierEntryPoint.lightingModel.rawValue)
    case "SCNShaderModifierEntryPointSurface":
        return scnDup(SCNShaderModifierEntryPoint.surface.rawValue)
    default:
        return nil
    }
}
