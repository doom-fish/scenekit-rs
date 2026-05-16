# scenekit-rs coverage audit (vs MacOSX26.2.sdk)

_Audit scope: top-level public SceneKit classes/protocols/enums/options/structs/exported constants/exported C helpers from `SceneKit.framework/Headers`. Objective-C methods and properties are intentionally out of scope for this report._

SDK_PUBLIC_SYMBOLS: 255
VERIFIED: 97
GAPS: 149
EXEMPT: 9
COVERAGE_PCT: 38.04%

_Inventory mix: 66 interfaces, 15 protocols, 36 enums, 4 option sets, 2 structs, 120 exported constants, 12 exported C helpers._

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| SCNAction | interface | SCNAction.h | Action |
| SCNAnimation | interface | SCNAnimation.h | Animation |
| SCNAnimationPlayer | interface | SCNAnimation.h | AnimationPlayer |
| SCNAntialiasingMode | enum | SCNSceneRenderer.h | scene_renderer::AntialiasingMode |
| SCNAudioPlayer | interface | SCNAudioSource.h | AudioPlayer |
| SCNAudioSource | interface | SCNAudioSource.h | AudioSource |
| SCNBox | interface | SCNParametricGeometry.h | Geometry::box_geometry |
| SCNBufferFrequency | enum | SCNShadable.h | program::BufferFrequency |
| SCNBufferStream | protocol | SCNShadable.h | BufferStream |
| SCNCamera | interface | SCNCamera.h | Camera |
| SCNCameraControlConfiguration | protocol | SCNView.h | CameraControlConfiguration |
| SCNCameraController | interface | SCNCameraController.h | CameraController |
| SCNCameraControllerDelegate | protocol | SCNCameraController.h | CameraControllerDelegate |
| SCNCone | interface | SCNParametricGeometry.h | Geometry::cone |
| SCNConsistencyElementIDErrorKey | const | SCNSceneSource.h | scene_source::consistency_element_id_error_key |
| SCNConsistencyElementTypeErrorKey | const | SCNSceneSource.h | scene_source::consistency_element_type_error_key |
| SCNConsistencyLineNumberErrorKey | const | SCNSceneSource.h | scene_source::consistency_line_number_error_key |
| SCNConstraint | interface | SCNConstraint.h | Constraint |
| SCNCylinder | interface | SCNParametricGeometry.h | Geometry::cylinder |
| SCNDebugOptions | options | SCNSceneRenderer.h | scene_renderer::DebugOptions |
| SCNDetailedErrorsKey | const | SCNSceneSource.h | scene_source::detailed_errors_key |
| SCNDistanceConstraint | interface | SCNConstraint.h | Constraint::distance |
| SCNFloor | interface | SCNParametricGeometry.h | Geometry::floor |
| SCNGeometry | interface | SCNGeometry.h | Geometry |
| SCNHitTestResult | interface | SCNHitTest.h | HitTestResult |
| SCNInteractionMode | enum | SCNCameraController.h | camera_controller::InteractionMode |
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
| SCNPhysicsContact | interface | SCNPhysicsContact.h | PhysicsContact |
| SCNPhysicsContactDelegate | protocol | SCNPhysicsWorld.h | PhysicsContactDelegate |
| SCNPhysicsTestBackfaceCullingKey | const | SCNPhysicsWorld.h | physics_world::physics_test_backface_culling_key |
| SCNPhysicsTestCollisionBitMaskKey | const | SCNPhysicsWorld.h | physics_world::physics_test_collision_bit_mask_key |
| SCNPhysicsTestSearchModeAll | const | SCNPhysicsWorld.h | physics_world::physics_test_search_mode_all |
| SCNPhysicsTestSearchModeAny | const | SCNPhysicsWorld.h | physics_world::physics_test_search_mode_any |
| SCNPhysicsTestSearchModeClosest | const | SCNPhysicsWorld.h | physics_world::physics_test_search_mode_closest |
| SCNPhysicsTestSearchModeKey | const | SCNPhysicsWorld.h | physics_world::physics_test_search_mode_key |
| SCNPhysicsWorld | interface | SCNPhysicsWorld.h | Scene::physics_world / PhysicsWorld |
| SCNPlane | interface | SCNParametricGeometry.h | Geometry::plane |
| SCNProgram | interface | SCNShadable.h | Program |
| SCNProgramDelegate | protocol | SCNShadable.h | ProgramDelegate |
| SCNProgramMappingChannelKey | const | SCNShadable.h | program::program_mapping_channel_key |
| SCNRenderer | interface | SCNRenderer.h | Renderer |
| SCNRenderingAPI | enum | SCNSceneRenderer.h | scene_renderer::RenderingAPI |
| SCNScene | interface | SCNScene.h | Scene |
| SCNSceneRenderer | protocol | SCNSceneRenderer.h | SceneRenderer for Renderer and View |
| SCNSceneRendererDelegate | protocol | SCNSceneRenderer.h | SceneRendererDelegate |
| SCNSceneSource | interface | SCNSceneSource.h | SceneSource |
| SCNSceneSourceAnimationImportPolicyDoNotPlay | const | SCNSceneSource.h | scene_source::scene_source_animation_import_policy_do_not_play |
| SCNSceneSourceAnimationImportPolicyKey | const | SCNSceneSource.h | scene_source::scene_source_animation_import_policy_key |
| SCNSceneSourceAnimationImportPolicyPlay | const | SCNSceneSource.h | scene_source::scene_source_animation_import_policy_play |
| SCNSceneSourceAnimationImportPolicyPlayRepeatedly | const | SCNSceneSource.h | scene_source::scene_source_animation_import_policy_play_repeatedly |
| SCNSceneSourceAnimationImportPolicyPlayUsingSceneTimeBase | const | SCNSceneSource.h | scene_source::scene_source_animation_import_policy_play_using_scene_time_base |
| SCNSceneSourceAssetAuthorKey | const | SCNSceneSource.h | scene_source::scene_source_asset_author_key |
| SCNSceneSourceAssetAuthoringToolKey | const | SCNSceneSource.h | scene_source::scene_source_asset_authoring_tool_key |
| SCNSceneSourceAssetContributorsKey | const | SCNSceneSource.h | scene_source::scene_source_asset_contributors_key |
| SCNSceneSourceAssetCreatedDateKey | const | SCNSceneSource.h | scene_source::scene_source_asset_created_date_key |
| SCNSceneSourceAssetDirectoryURLsKey | const | SCNSceneSource.h | scene_source::scene_source_asset_directory_urls_key |
| SCNSceneSourceAssetModifiedDateKey | const | SCNSceneSource.h | scene_source::scene_source_asset_modified_date_key |
| SCNSceneSourceAssetUnitKey | const | SCNSceneSource.h | scene_source::scene_source_asset_unit_key |
| SCNSceneSourceAssetUnitMeterKey | const | SCNSceneSource.h | scene_source::scene_source_asset_unit_meter_key |
| SCNSceneSourceAssetUnitNameKey | const | SCNSceneSource.h | scene_source::scene_source_asset_unit_name_key |
| SCNSceneSourceAssetUpAxisKey | const | SCNSceneSource.h | scene_source::scene_source_asset_up_axis_key |
| SCNSceneSourceCheckConsistencyKey | const | SCNSceneSource.h | scene_source::scene_source_check_consistency_key |
| SCNSceneSourceConvertToYUpKey | const | SCNSceneSource.h | scene_source::scene_source_convert_to_y_up_key |
| SCNSceneSourceConvertUnitsToMetersKey | const | SCNSceneSource.h | scene_source::scene_source_convert_units_to_meters_key |
| SCNSceneSourceCreateNormalsIfAbsentKey | const | SCNSceneSource.h | scene_source::scene_source_create_normals_if_absent_key |
| SCNSceneSourceFlattenSceneKey | const | SCNSceneSource.h | scene_source::scene_source_flatten_scene_key |
| SCNSceneSourceLoadingOptionPreserveOriginalTopology | const | SCNSceneSource.h | scene_source::scene_source_loading_option_preserve_original_topology |
| SCNSceneSourceOverrideAssetURLsKey | const | SCNSceneSource.h | scene_source::scene_source_override_asset_urls_key |
| SCNSceneSourceStatus | enum | SCNSceneSource.h | SceneSourceStatus |
| SCNSceneSourceStrictConformanceKey | const | SCNSceneSource.h | scene_source::scene_source_strict_conformance_key |
| SCNShadable | protocol | SCNShadable.h | Shadable for Material and Geometry |
| SCNShaderModifierEntryPointFragment | const | SCNShadable.h | program::shader_modifier_entry_point_fragment |
| SCNShaderModifierEntryPointGeometry | const | SCNShadable.h | program::shader_modifier_entry_point_geometry |
| SCNShaderModifierEntryPointLightingModel | const | SCNShadable.h | program::shader_modifier_entry_point_lighting_model |
| SCNShaderModifierEntryPointSurface | const | SCNShadable.h | program::shader_modifier_entry_point_surface |
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
| SCNAvoidOccluderConstraint | interface | SCNConstraint.h | Only SCNLookAtConstraint and SCNDistanceConstraint are wrapped. |
| SCNAvoidOccluderConstraintDelegate | protocol | SCNConstraint.h | No public Rust protocol/delegate bridge. |
| SCNBillboardAxis | options | SCNConstraint.h | Constraint option set not exposed. |
| SCNBillboardConstraint | interface | SCNConstraint.h | Only SCNLookAtConstraint and SCNDistanceConstraint are wrapped. |
| SCNBlendMode | enum | SCNMaterial.h | Material/render-state enum is not exposed. |
| SCNBoundingVolume | protocol | SCNBoundingVolume.h | No public Rust protocol/delegate bridge. |
| SCNCameraProjectionDirection | enum | SCNCamera.h | No Rust enum/options wrapper. |
| SCNCapsule | interface | SCNParametricGeometry.h | Only box/sphere/cylinder/cone/plane/floor/text geometry constructors are wrapped. |
| SCNChamferMode | enum | SCNParametricGeometry.h | No Rust enum/options wrapper. |
| SCNColorMask | options | SceneKitTypes.h | Material/render-state enum is not exposed. |
| SCNCullMode | enum | SCNMaterial.h | Material/render-state enum is not exposed. |
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
| SCNPhysicsVehicle | interface | SCNPhysicsBehavior.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPhysicsVehicleWheel | interface | SCNPhysicsBehavior.h | Only SCNPhysicsBody is wrapped; world/shape/behavior APIs are absent. |
| SCNPreferLowPowerDeviceKey | const | SCNView.h | SCNView option constants are not exposed. |
| SCNPreferredDeviceKey | const | SCNView.h | SCNView option constants are not exposed. |
| SCNPreferredRenderingAPIKey | const | SCNView.h | SCNView option constants are not exposed. |
| SCNProjectionTransform | const | SCNNode.h | Shader semantic constants are not exposed. |
| SCNPyramid | interface | SCNParametricGeometry.h | Only box/sphere/cylinder/cone/plane/floor/text geometry constructors are wrapped. |
| SCNReferenceLoadingPolicy | enum | SCNReferenceNode.h | No Rust enum/options wrapper. |
| SCNReferenceNode | interface | SCNReferenceNode.h | Asset-pipeline/import APIs are not wrapped. |
| SCNReplicatorConstraint | interface | SCNConstraint.h | Only SCNLookAtConstraint and SCNDistanceConstraint are wrapped. |
| SCNSceneEndTimeAttributeKey | const | SCNScene.h | No wrapper for this exported SceneKit constant. |
| SCNSceneExportDelegate | protocol | SCNScene.h | No public Rust protocol/delegate bridge. |
| SCNSceneExportDestinationURL | const | SCNScene.h | No wrapper for this exported SceneKit constant. |
| SCNSceneFrameRateAttributeKey | const | SCNScene.h | No wrapper for this exported SceneKit constant. |
| SCNSceneStartTimeAttributeKey | const | SCNScene.h | No wrapper for this exported SceneKit constant. |
| SCNSceneUpAxisAttributeKey | const | SCNScene.h | No wrapper for this exported SceneKit constant. |
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
| Symbol | Kind | Header | Rationale |
| --- | --- | --- | --- |
| SCNLayer | interface | SCNLayer.h | OpenGL-backed layer is deprecated on macOS and intentionally skipped. | SCN_GL_DEPRECATED_MAC(10_8, 10_14) |
| SCNLightAttenuationEndKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightAttenuationFalloffExponentKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightAttenuationStartKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightShadowFarClippingKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightShadowNearClippingKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightSpotInnerAngleKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNLightSpotOuterAngleKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(... macos(10.8, 10.10)) |
| SCNSceneSourceUseSafeModeKey | const | SCNSceneSource.h | Deprecated scene-source loading option skipped by policy. | API_DEPRECATED(... macos(10.8, 10.13)) |
