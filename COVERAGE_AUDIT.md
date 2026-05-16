# scenekit-rs coverage audit (vs MacOSX26.2.sdk)

_Audit scope: top-level public SceneKit classes/protocols/enums/options/structs/exported constants/exported C helpers from `SceneKit.framework/Headers`. Objective-C methods and properties are intentionally out of scope for this report._

SDK_PUBLIC_SYMBOLS: 255
VERIFIED: 40
GAPS: 206
EXEMPT: 9
COVERAGE_PCT: 16.26%

_Inventory mix: 66 interfaces, 15 protocols, 36 enums, 4 option sets, 2 structs, 120 exported constants, 12 exported C helpers._

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| SCNAction | interface | SCNAction.h | Action |
| SCNAnimation | interface | SCNAnimation.h | Animation |
| SCNAnimationPlayer | interface | SCNAnimation.h | AnimationPlayer |
| SCNAudioPlayer | interface | SCNAudioSource.h | AudioPlayer |
| SCNAudioSource | interface | SCNAudioSource.h | AudioSource |
| SCNBox | interface | SCNParametricGeometry.h | Geometry::box_geometry |
| SCNCamera | interface | SCNCamera.h | Camera |
| SCNCone | interface | SCNParametricGeometry.h | Geometry::cone |
| SCNConstraint | interface | SCNConstraint.h | Constraint |
| SCNCylinder | interface | SCNParametricGeometry.h | Geometry::cylinder |
| SCNDistanceConstraint | interface | SCNConstraint.h | Constraint::distance |
| SCNFloor | interface | SCNParametricGeometry.h | Geometry::floor |
| SCNGeometry | interface | SCNGeometry.h | Geometry |
| SCNHitTestResult | interface | SCNHitTest.h | HitTestResult |
| SCNLight | interface | SCNLight.h | Light |
| SCNLightTypeAmbient | const | SCNLight.h | LightType::Ambient |
| SCNLightTypeArea | const | SCNLight.h | LightType::Area |
| SCNLightTypeDirectional | const | SCNLight.h | LightType::Directional |
| SCNLightTypeIES | const | SCNLight.h | LightType::Ies |
| SCNLightTypeOmni | const | SCNLight.h | LightType::Omni |
| SCNLightTypeProbe | const | SCNLight.h | LightType::Probe |
| SCNLightTypeSpot | const | SCNLight.h | LightType::Spot |
| SCNLookAtConstraint | interface | SCNConstraint.h | Constraint::look_at |
| SCNMaterial | interface | SCNMaterial.h | Material |
| SCNMaterialProperty | interface | SCNMaterialProperty.h | MaterialProperty |
| SCNNode | interface | SCNNode.h | Node |
| SCNParticleSystem | interface | SCNParticleSystem.h | ParticleSystem |
| SCNPhysicsBody | interface | SCNPhysicsBody.h | PhysicsBody |
| SCNPhysicsBodyType | enum | SCNPhysicsBody.h | PhysicsBodyType |
| SCNPlane | interface | SCNParametricGeometry.h | Geometry::plane |
| SCNRenderer | interface | SCNRenderer.h | Renderer |
| SCNScene | interface | SCNScene.h | Scene |
| SCNShadowMode | enum | SCNLight.h | ShadowMode |
| SCNSphere | interface | SCNParametricGeometry.h | Geometry::sphere |
| SCNTechnique | interface | SCNTechnique.h | Technique |
| SCNText | interface | SCNParametricGeometry.h | Geometry::text |
| SCNTransaction | interface | SCNTransaction.h | Transaction |
| SCNVector3 | struct | SceneKitTypes.h | Vector3 |
| SCNVector4 | struct | SceneKitTypes.h | Vector4 |
| SCNView | interface | SCNView.h | View |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| SCNAccelerationConstraint | interface | SCNConstraint.h | Only SCNLookAtConstraint and SCNDistanceConstraint are wrapped. |
| SCNActionTimingMode | enum | SceneKitTypes.h | No Rust enum/options wrapper. |
| SCNActionable | protocol | SCNAction.h | No public Rust protocol/delegate bridge. |
| SCNAnimatable | protocol | SCNAnimation.h | No public Rust protocol/delegate bridge. |
| SCNAnimationEvent | interface | SCNAnimation.h | Advanced animation timing/event APIs are not wrapped. |
| SCNAntialiasingMode | enum | SCNSceneRenderer.h | Renderer/view configuration enum is not exposed. |
| SCNAvoidOccluderConstraint | interface | SCNConstraint.h | Only SCNLookAtConstraint and SCNDistanceConstraint are wrapped. |
| SCNAvoidOccluderConstraintDelegate | protocol | SCNConstraint.h | No public Rust protocol/delegate bridge. |
| SCNBillboardAxis | options | SCNConstraint.h | Constraint option set not exposed. |
| SCNBillboardConstraint | interface | SCNConstraint.h | Only SCNLookAtConstraint and SCNDistanceConstraint are wrapped. |
| SCNBlendMode | enum | SCNMaterial.h | Material/render-state enum is not exposed. |
| SCNBoundingVolume | protocol | SCNBoundingVolume.h | No public Rust protocol/delegate bridge. |
| SCNBufferFrequency | enum | SCNShadable.h | No Rust enum/options wrapper. |
| SCNBufferStream | protocol | SCNShadable.h | No public Rust protocol/delegate bridge. |
| SCNCameraControlConfiguration | protocol | SCNView.h | No public Rust protocol/delegate bridge. |
| SCNCameraController | interface | SCNCameraController.h | Interactive camera-controller APIs are not wrapped. |
| SCNCameraControllerDelegate | protocol | SCNCameraController.h | No public Rust protocol/delegate bridge. |
| SCNCameraProjectionDirection | enum | SCNCamera.h | No Rust enum/options wrapper. |
| SCNCapsule | interface | SCNParametricGeometry.h | Only box/sphere/cylinder/cone/plane/floor/text geometry constructors are wrapped. |
| SCNChamferMode | enum | SCNParametricGeometry.h | No Rust enum/options wrapper. |
| SCNColorMask | options | SceneKitTypes.h | Material/render-state enum is not exposed. |
| SCNConsistencyElementIDErrorKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNConsistencyElementTypeErrorKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNConsistencyLineNumberErrorKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNCullMode | enum | SCNMaterial.h | Material/render-state enum is not exposed. |
| SCNDebugOptions | options | SCNSceneRenderer.h | Renderer/view configuration enum is not exposed. |
| SCNDetailedErrorsKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNErrorDomain | const | SceneKitTypes.h | No wrapper for this exported SceneKit constant. |
| SCNExportJavaScriptModule | function | SCNJavascript.h | No wrapper for this exported C helper. |
| SCNFillMode | enum | SCNMaterial.h | Material/render-state enum is not exposed. |
| SCNFilterMode | enum | SCNMaterialProperty.h | Material/render-state enum is not exposed. |
| SCNGeometryElement | interface | SCNGeometry.h | Low-level geometry-source/element APIs are not wrapped. |
| SCNGeometryPrimitiveType | enum | SCNGeometry.h | Low-level geometry enums are not wrapped. |
| SCNGeometrySource | interface | SCNGeometry.h | Low-level geometry-source/element APIs are not wrapped. |
| SCNGeometrySourceSemanticBoneIndices | const | SCNGeometry.h | No wrapper for this exported SceneKit constant. |
| SCNGeometrySourceSemanticBoneWeights | const | SCNGeometry.h | No wrapper for this exported SceneKit constant. |
| SCNGeometrySourceSemanticColor | const | SCNGeometry.h | No wrapper for this exported SceneKit constant. |
| SCNGeometrySourceSemanticEdgeCrease | const | SCNGeometry.h | No wrapper for this exported SceneKit constant. |
| SCNGeometrySourceSemanticNormal | const | SCNGeometry.h | No wrapper for this exported SceneKit constant. |
| SCNGeometrySourceSemanticTangent | const | SCNGeometry.h | No wrapper for this exported SceneKit constant. |
| SCNGeometrySourceSemanticTexcoord | const | SCNGeometry.h | No wrapper for this exported SceneKit constant. |
| SCNGeometrySourceSemanticVertex | const | SCNGeometry.h | No wrapper for this exported SceneKit constant. |
| SCNGeometrySourceSemanticVertexCrease | const | SCNGeometry.h | No wrapper for this exported SceneKit constant. |
| SCNGeometryTessellator | interface | SCNGeometry.h | Low-level geometry-source/element APIs are not wrapped. |
| SCNHitTestBackFaceCullingKey | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNHitTestBoundingBoxOnlyKey | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNHitTestClipToZRangeKey | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNHitTestFirstFoundOnlyKey | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNHitTestIgnoreChildNodesKey | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNHitTestIgnoreHiddenNodesKey | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNHitTestOptionCategoryBitMask | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNHitTestOptionIgnoreLightArea | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNHitTestOptionSearchMode | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNHitTestRootNodeKey | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNHitTestSearchMode | enum | SCNHitTest.h | Hit-test search-mode enum is not exposed. |
| SCNHitTestSortResultsKey | const | SCNHitTest.h | Hit-test option constants are not exposed. |
| SCNIKConstraint | interface | SCNConstraint.h | Only SCNLookAtConstraint and SCNDistanceConstraint are wrapped. |
| SCNInteractionMode | enum | SCNCameraController.h | No Rust enum/options wrapper. |
| SCNLevelOfDetail | interface | SCNLevelOfDetail.h | Asset-pipeline/import APIs are not wrapped. |
| SCNLightAreaType | enum | SCNLight.h | Only basic light type/shadow mode enums are wrapped. |
| SCNLightProbeType | enum | SCNLight.h | Only basic light type/shadow mode enums are wrapped. |
| SCNLightProbeUpdateType | enum | SCNLight.h | Only basic light type/shadow mode enums are wrapped. |
| SCNLightingModelBlinn | const | SCNMaterial.h | No material lighting-model constant wrapper. |
| SCNLightingModelConstant | const | SCNMaterial.h | No material lighting-model constant wrapper. |
| SCNLightingModelLambert | const | SCNMaterial.h | No material lighting-model constant wrapper. |
| SCNLightingModelPhong | const | SCNMaterial.h | No material lighting-model constant wrapper. |
| SCNLightingModelPhysicallyBased | const | SCNMaterial.h | No material lighting-model constant wrapper. |
| SCNLightingModelShadowOnly | const | SCNMaterial.h | No material lighting-model constant wrapper. |
| SCNMatrix4EqualToMatrix4 | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNMatrix4FromGLKMatrix4 | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNMatrix4Identity | const | SceneKitTypes.h | No wrapper for this exported SceneKit constant. |
| SCNMatrix4Invert | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNMatrix4IsIdentity | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNMatrix4MakeRotation | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNMatrix4Mult | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNMatrix4Rotate | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNMatrix4Scale | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNMatrix4ToGLKMatrix4 | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNModelTransform | const | SCNNode.h | Shader semantic constants are not exposed. |
| SCNModelViewProjectionTransform | const | SCNNode.h | Shader semantic constants are not exposed. |
| SCNModelViewTransform | const | SCNNode.h | Shader semantic constants are not exposed. |
| SCNMorpher | interface | SCNMorpher.h | Asset-pipeline/import APIs are not wrapped. |
| SCNMorpherCalculationMode | enum | SCNMorpher.h | No Rust enum/options wrapper. |
| SCNMovabilityHint | enum | SCNNode.h | No Rust enum/options wrapper. |
| SCNNodeFocusBehavior | enum | SCNNode.h | No Rust enum/options wrapper. |
| SCNNodeRendererDelegate | protocol | SCNNode.h | No public Rust protocol/delegate bridge. |
| SCNNormalTransform | const | SCNNode.h | Shader semantic constants are not exposed. |
| SCNParticleBirthDirection | enum | SCNParticleSystem.h | Particle enums beyond basic birth-rate/life/loop controls are absent. |
| SCNParticleBirthLocation | enum | SCNParticleSystem.h | Particle enums beyond basic birth-rate/life/loop controls are absent. |
| SCNParticleBlendMode | enum | SCNParticleSystem.h | Particle enums beyond basic birth-rate/life/loop controls are absent. |
| SCNParticleEvent | enum | SCNParticleSystem.h | Particle enums beyond basic birth-rate/life/loop controls are absent. |
| SCNParticleImageSequenceAnimationMode | enum | SCNParticleSystem.h | Particle enums beyond basic birth-rate/life/loop controls are absent. |
| SCNParticleInputMode | enum | SCNParticleSystem.h | Particle enums beyond basic birth-rate/life/loop controls are absent. |
| SCNParticleModifierStage | enum | SCNParticleSystem.h | Particle enums beyond basic birth-rate/life/loop controls are absent. |
| SCNParticleOrientationMode | enum | SCNParticleSystem.h | Particle enums beyond basic birth-rate/life/loop controls are absent. |
| SCNParticlePropertyAngle | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyAngularVelocity | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyBounce | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyCharge | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyColor | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyContactNormal | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyContactPoint | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyController | interface | SCNParticleSystem.h | Particle property-controller APIs are not wrapped. |
| SCNParticlePropertyFrame | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyFrameRate | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyFriction | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyLife | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyOpacity | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyPosition | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyRotationAxis | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertySize | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticlePropertyVelocity | const | SCNParticleSystem.h | Particle modifier/event constants are not exposed. |
| SCNParticleSortingMode | enum | SCNParticleSystem.h | Particle enums beyond basic birth-rate/life/loop controls are absent. |
| SCNPhysicsBallSocketJoint | interface | SCNPhysicsBehavior.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsBehavior | interface | SCNPhysicsBehavior.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsCollisionCategory | options | SCNPhysicsBody.h | Physics enums/options beyond SCNPhysicsBodyType are absent. |
| SCNPhysicsConeTwistJoint | interface | SCNPhysicsBehavior.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsContact | interface | SCNPhysicsContact.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsContactDelegate | protocol | SCNPhysicsWorld.h | No public Rust protocol/delegate bridge. |
| SCNPhysicsField | interface | SCNPhysicsField.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsFieldScope | enum | SCNPhysicsField.h | Physics enums/options beyond SCNPhysicsBodyType are absent. |
| SCNPhysicsHingeJoint | interface | SCNPhysicsBehavior.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsShape | interface | SCNPhysicsShape.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsShapeKeepAsCompoundKey | const | SCNPhysicsShape.h | Physics option/search constants are not exposed. |
| SCNPhysicsShapeOptionCollisionMargin | const | SCNPhysicsShape.h | Physics option/search constants are not exposed. |
| SCNPhysicsShapeScaleKey | const | SCNPhysicsShape.h | Physics option/search constants are not exposed. |
| SCNPhysicsShapeTypeBoundingBox | const | SCNPhysicsShape.h | Physics option/search constants are not exposed. |
| SCNPhysicsShapeTypeConcavePolyhedron | const | SCNPhysicsShape.h | Physics option/search constants are not exposed. |
| SCNPhysicsShapeTypeConvexHull | const | SCNPhysicsShape.h | Physics option/search constants are not exposed. |
| SCNPhysicsShapeTypeKey | const | SCNPhysicsShape.h | Physics option/search constants are not exposed. |
| SCNPhysicsSliderJoint | interface | SCNPhysicsBehavior.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsTestBackfaceCullingKey | const | SCNPhysicsWorld.h | Physics option/search constants are not exposed. |
| SCNPhysicsTestCollisionBitMaskKey | const | SCNPhysicsWorld.h | Physics option/search constants are not exposed. |
| SCNPhysicsTestSearchModeAll | const | SCNPhysicsWorld.h | Physics option/search constants are not exposed. |
| SCNPhysicsTestSearchModeAny | const | SCNPhysicsWorld.h | Physics option/search constants are not exposed. |
| SCNPhysicsTestSearchModeClosest | const | SCNPhysicsWorld.h | Physics option/search constants are not exposed. |
| SCNPhysicsTestSearchModeKey | const | SCNPhysicsWorld.h | Physics option/search constants are not exposed. |
| SCNPhysicsVehicle | interface | SCNPhysicsBehavior.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsVehicleWheel | interface | SCNPhysicsBehavior.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsWorld | interface | SCNPhysicsWorld.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPreferLowPowerDeviceKey | const | SCNView.h | SCNView option constants are not exposed. |
| SCNPreferredDeviceKey | const | SCNView.h | SCNView option constants are not exposed. |
| SCNPreferredRenderingAPIKey | const | SCNView.h | SCNView option constants are not exposed. |
| SCNProgram | interface | SCNShadable.h | Shader/program APIs are not wrapped. |
| SCNProgramDelegate | protocol | SCNShadable.h | No public Rust protocol/delegate bridge. |
| SCNProgramMappingChannelKey | const | SCNShadable.h | Shader/program string constants are not exposed. |
| SCNProjectionTransform | const | SCNNode.h | Shader semantic constants are not exposed. |
| SCNPyramid | interface | SCNParametricGeometry.h | Only box/sphere/cylinder/cone/plane/floor/text geometry constructors are wrapped. |
| SCNReferenceLoadingPolicy | enum | SCNReferenceNode.h | No Rust enum/options wrapper. |
| SCNReferenceNode | interface | SCNReferenceNode.h | Asset-pipeline/import APIs are not wrapped. |
| SCNRenderingAPI | enum | SCNSceneRenderer.h | Renderer/view configuration enum is not exposed. |
| SCNReplicatorConstraint | interface | SCNConstraint.h | Only SCNLookAtConstraint and SCNDistanceConstraint are wrapped. |
| SCNSceneEndTimeAttributeKey | const | SCNScene.h | No wrapper for this exported SceneKit constant. |
| SCNSceneExportDelegate | protocol | SCNScene.h | No public Rust protocol/delegate bridge. |
| SCNSceneExportDestinationURL | const | SCNScene.h | No wrapper for this exported SceneKit constant. |
| SCNSceneFrameRateAttributeKey | const | SCNScene.h | No wrapper for this exported SceneKit constant. |
| SCNSceneRenderer | protocol | SCNSceneRenderer.h | No public Rust protocol/delegate bridge. |
| SCNSceneRendererDelegate | protocol | SCNSceneRenderer.h | No public Rust protocol/delegate bridge. |
| SCNSceneSource | interface | SCNSceneSource.h | Asset-pipeline/import APIs are not wrapped. |
| SCNSceneSourceAnimationImportPolicyDoNotPlay | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAnimationImportPolicyKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAnimationImportPolicyPlay | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAnimationImportPolicyPlayRepeatedly | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAnimationImportPolicyPlayUsingSceneTimeBase | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAssetAuthorKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAssetAuthoringToolKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAssetContributorsKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAssetCreatedDateKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAssetDirectoryURLsKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAssetModifiedDateKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAssetUnitKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAssetUnitMeterKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAssetUnitNameKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceAssetUpAxisKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceCheckConsistencyKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceConvertToYUpKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceConvertUnitsToMetersKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceCreateNormalsIfAbsentKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceFlattenSceneKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceLoadingOptionPreserveOriginalTopology | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceOverrideAssetURLsKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneSourceStatus | enum | SCNSceneSource.h | No Rust enum/options wrapper. |
| SCNSceneSourceStrictConformanceKey | const | SCNSceneSource.h | Scene-source string keys are not exposed. |
| SCNSceneStartTimeAttributeKey | const | SCNScene.h | No wrapper for this exported SceneKit constant. |
| SCNSceneUpAxisAttributeKey | const | SCNScene.h | No wrapper for this exported SceneKit constant. |
| SCNShadable | protocol | SCNShadable.h | No public Rust protocol/delegate bridge. |
| SCNShaderModifierEntryPointFragment | const | SCNShadable.h | Shader/program string constants are not exposed. |
| SCNShaderModifierEntryPointGeometry | const | SCNShadable.h | Shader/program string constants are not exposed. |
| SCNShaderModifierEntryPointLightingModel | const | SCNShadable.h | Shader/program string constants are not exposed. |
| SCNShaderModifierEntryPointSurface | const | SCNShadable.h | Shader/program string constants are not exposed. |
| SCNShape | interface | SCNParametricGeometry.h | Only box/sphere/cylinder/cone/plane/floor/text geometry constructors are wrapped. |
| SCNSkinner | interface | SCNSkinner.h | Asset-pipeline/import APIs are not wrapped. |
| SCNSliderConstraint | interface | SCNConstraint.h | Only SCNLookAtConstraint and SCNDistanceConstraint are wrapped. |
| SCNTechniqueSupport | protocol | SCNTechnique.h | No public Rust protocol/delegate bridge. |
| SCNTessellationSmoothingMode | enum | SCNGeometry.h | Low-level geometry enums are not wrapped. |
| SCNTimingFunction | interface | SCNAnimation.h | Advanced animation timing/event APIs are not wrapped. |
| SCNTorus | interface | SCNParametricGeometry.h | Only box/sphere/cylinder/cone/plane/floor/text geometry constructors are wrapped. |
| SCNTransformConstraint | interface | SCNConstraint.h | Only SCNLookAtConstraint and SCNDistanceConstraint are wrapped. |
| SCNTransparencyMode | enum | SCNMaterial.h | Material/render-state enum is not exposed. |
| SCNTube | interface | SCNParametricGeometry.h | Only box/sphere/cylinder/cone/plane/floor/text geometry constructors are wrapped. |
| SCNVector3EqualToVector3 | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNVector3Zero | const | SceneKitTypes.h | No wrapper for this exported SceneKit constant. |
| SCNVector4EqualToVector4 | function | SceneKitTypes.h | No wrapper for this exported C helper. |
| SCNVector4Zero | const | SceneKitTypes.h | No wrapper for this exported SceneKit constant. |
| SCNViewTransform | const | SCNNode.h | Shader semantic constants are not exposed. |
| SCNWrapMode | enum | SCNMaterialProperty.h | Material/render-state enum is not exposed. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| SCNLayer | interface | SCNLayer.h | OpenGL-backed layer is deprecated on macOS and intentionally skipped. | SCN_GL_DEPRECATED_MAC(10_8, 10_14) |
| SCNLightAttenuationEndKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightAttenuationFalloffExponentKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightAttenuationStartKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightShadowFarClippingKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightShadowNearClippingKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightSpotInnerAngleKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightSpotOuterAngleKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNSceneSourceUseSafeModeKey | const | SCNSceneSource.h | Deprecated scene-source loading option skipped by policy. | API_DEPRECATED(... macos(10.8, 10.13)) |
