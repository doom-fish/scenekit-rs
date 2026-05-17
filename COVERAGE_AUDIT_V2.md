# scenekit-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 255
VERIFIED: 246
GAPS: 0
EXEMPT: 9
COVERAGE_PCT: 100.00%

Re-verified against MacOSX26.2.sdk by checking all 9 previous EXEMPT entries in SCNLayer.h, SceneKitDeprecated.h, and SCNSceneSource.h; all carry valid SDK deprecation attributes (API_DEPRECATED or SCN_GL_DEPRECATED_MAC). 246 verified symbols cover 100% of non-deprecated public SceneKit surface. Inventory: 66 interfaces, 15 protocols, 36 enums, 4 option sets, 2 structs, 120 exported constants, 12 exported C helpers.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| SCNAccelerationConstraint | interface | SCNConstraint.h | extended_constraints::* |
| SCNAction | interface | SCNAction.h | Action |
| SCNActionable | protocol | SCNAction.h | protocols::* |
| SCNActionTimingMode | enum | SceneKitTypes.h | symbols::* |
| SCNAnimatable | protocol | SCNAnimation.h | protocols::* |
| SCNAnimation | interface | SCNAnimation.h | Animation |
| SCNAnimationEvent | interface | SCNAnimation.h | protocols::* |
| SCNAnimationPlayer | interface | SCNAnimation.h | AnimationPlayer |
| SCNAntialiasingMode | enum | SCNSceneRenderer.h | scene_renderer::AntialiasingMode |
| SCNAudioPlayer | interface | SCNAudioSource.h | AudioPlayer |
| SCNAudioSource | interface | SCNAudioSource.h | AudioSource |
| SCNAvoidOccluderConstraint | interface | SCNConstraint.h | extended_constraints::* |
| SCNAvoidOccluderConstraintDelegate | protocol | SCNConstraint.h | delegates::* |
| SCNBillboardAxis | options | SCNConstraint.h | symbols::* |
| SCNBillboardConstraint | interface | SCNConstraint.h | extended_constraints::* |
| SCNBlendMode | enum | SCNMaterial.h | symbols::* |
| SCNBoundingVolume | protocol | SCNBoundingVolume.h | protocols::* |
| SCNBox | interface | SCNParametricGeometry.h | Geometry::box_geometry |
| SCNBufferFrequency | enum | SCNShadable.h | program::BufferFrequency |
| SCNBufferStream | protocol | SCNShadable.h | BufferStream |
| SCNCamera | interface | SCNCamera.h | Camera |
| SCNCameraControlConfiguration | protocol | SCNView.h | CameraControlConfiguration |
| SCNCameraController | interface | SCNCameraController.h | CameraController |
| SCNCameraControllerDelegate | protocol | SCNCameraController.h | CameraControllerDelegate |
| SCNCameraProjectionDirection | enum | SCNCamera.h | symbols::* |
| SCNCapsule | interface | SCNParametricGeometry.h | extended_geometry::* |
| SCNChamferMode | enum | SCNParametricGeometry.h | symbols::* |
| SCNColorMask | options | SceneKitTypes.h | symbols::* |
| SCNCone | interface | SCNParametricGeometry.h | Geometry::cone |
| SCNConsistencyElementIDErrorKey | const | SCNSceneSource.h | scene_source::consistency_element_id_error_key |
| SCNConsistencyElementTypeErrorKey | const | SCNSceneSource.h | scene_source::consistency_element_type_error_key |
| SCNConsistencyLineNumberErrorKey | const | SCNSceneSource.h | scene_source::consistency_line_number_error_key |
| SCNConstraint | interface | SCNConstraint.h | Constraint |
| SCNCullMode | enum | SCNMaterial.h | symbols::* |
| SCNCylinder | interface | SCNParametricGeometry.h | Geometry::cylinder |
| SCNDebugOptions | options | SCNSceneRenderer.h | scene_renderer::DebugOptions |
| SCNDetailedErrorsKey | const | SCNSceneSource.h | scene_source::detailed_errors_key |
| SCNDistanceConstraint | interface | SCNConstraint.h | Constraint::distance |
| SCNErrorDomain | const | SceneKitTypes.h | symbols::* |
| SCNExportJavaScriptModule | function | SCNJavascript.h | delegates::* |
| SCNFillMode | enum | SCNMaterial.h | symbols::* |
| SCNFilterMode | enum | SCNMaterialProperty.h | symbols::* |
| SCNFloor | interface | SCNParametricGeometry.h | Geometry::floor |
| SCNGeometry | interface | SCNGeometry.h | Geometry |
| SCNGeometryElement | interface | SCNGeometry.h | extended_geometry::* |
| SCNGeometryPrimitiveType | enum | SCNGeometry.h | symbols::* |
| SCNGeometrySource | interface | SCNGeometry.h | extended_geometry::* |
| SCNGeometrySourceSemanticBoneIndices | const | SCNGeometry.h | symbols::* |
| SCNGeometrySourceSemanticBoneWeights | const | SCNGeometry.h | symbols::* |
| SCNGeometrySourceSemanticColor | const | SCNGeometry.h | symbols::* |
| SCNGeometrySourceSemanticEdgeCrease | const | SCNGeometry.h | symbols::* |
| SCNGeometrySourceSemanticNormal | const | SCNGeometry.h | symbols::* |
| SCNGeometrySourceSemanticTangent | const | SCNGeometry.h | symbols::* |
| SCNGeometrySourceSemanticTexcoord | const | SCNGeometry.h | symbols::* |
| SCNGeometrySourceSemanticVertex | const | SCNGeometry.h | symbols::* |
| SCNGeometrySourceSemanticVertexCrease | const | SCNGeometry.h | symbols::* |
| SCNGeometryTessellator | interface | SCNGeometry.h | extended_geometry::* |
| SCNHitTestBackFaceCullingKey | const | SCNHitTest.h | symbols::* |
| SCNHitTestBoundingBoxOnlyKey | const | SCNHitTest.h | symbols::* |
| SCNHitTestClipToZRangeKey | const | SCNHitTest.h | symbols::* |
| SCNHitTestFirstFoundOnlyKey | const | SCNHitTest.h | symbols::* |
| SCNHitTestIgnoreChildNodesKey | const | SCNHitTest.h | symbols::* |
| SCNHitTestIgnoreHiddenNodesKey | const | SCNHitTest.h | symbols::* |
| SCNHitTestOptionCategoryBitMask | const | SCNHitTest.h | symbols::* |
| SCNHitTestOptionIgnoreLightArea | const | SCNHitTest.h | symbols::* |
| SCNHitTestOptionSearchMode | const | SCNHitTest.h | symbols::* |
| SCNHitTestResult | interface | SCNHitTest.h | HitTestResult |
| SCNHitTestRootNodeKey | const | SCNHitTest.h | symbols::* |
| SCNHitTestSearchMode | enum | SCNHitTest.h | symbols::* |
| SCNHitTestSortResultsKey | const | SCNHitTest.h | symbols::* |
| SCNIKConstraint | interface | SCNConstraint.h | extended_constraints::* |
| SCNInteractionMode | enum | SCNCameraController.h | camera_controller::InteractionMode |
| SCNLevelOfDetail | interface | SCNLevelOfDetail.h | extended_geometry::* |
| SCNLight | interface | SCNLight.h | Light |
| SCNLightAreaType | enum | SCNLight.h | symbols::* |
| SCNLightingModelBlinn | const | SCNMaterial.h | symbols::* |
| SCNLightingModelConstant | const | SCNMaterial.h | symbols::* |
| SCNLightingModelLambert | const | SCNMaterial.h | symbols::* |
| SCNLightingModelPhong | const | SCNMaterial.h | symbols::* |
| SCNLightingModelPhysicallyBased | const | SCNMaterial.h | symbols::* |
| SCNLightingModelShadowOnly | const | SCNMaterial.h | symbols::* |
| SCNLightProbeType | enum | SCNLight.h | symbols::* |
| SCNLightProbeUpdateType | enum | SCNLight.h | symbols::* |
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
| SCNMatrix4EqualToMatrix4 | function | SceneKitTypes.h | symbols::* |
| SCNMatrix4FromGLKMatrix4 | function | SceneKitTypes.h | symbols::* |
| SCNMatrix4Identity | const | SceneKitTypes.h | symbols::* |
| SCNMatrix4Invert | function | SceneKitTypes.h | symbols::* |
| SCNMatrix4IsIdentity | function | SceneKitTypes.h | symbols::* |
| SCNMatrix4MakeRotation | function | SceneKitTypes.h | symbols::* |
| SCNMatrix4Mult | function | SceneKitTypes.h | symbols::* |
| SCNMatrix4Rotate | function | SceneKitTypes.h | symbols::* |
| SCNMatrix4Scale | function | SceneKitTypes.h | symbols::* |
| SCNMatrix4ToGLKMatrix4 | function | SceneKitTypes.h | symbols::* |
| SCNModelTransform | const | SCNNode.h | symbols::* |
| SCNModelViewProjectionTransform | const | SCNNode.h | symbols::* |
| SCNModelViewTransform | const | SCNNode.h | symbols::* |
| SCNMorpher | interface | SCNMorpher.h | extended_geometry::* |
| SCNMorpherCalculationMode | enum | SCNMorpher.h | symbols::* |
| SCNMovabilityHint | enum | SCNNode.h | symbols::* |
| SCNNode | interface | SCNNode.h | Node |
| SCNNodeFocusBehavior | enum | SCNNode.h | symbols::* |
| SCNNodeRendererDelegate | protocol | SCNNode.h | delegates::* |
| SCNNormalTransform | const | SCNNode.h | symbols::* |
| SCNParticleBirthDirection | enum | SCNParticleSystem.h | symbols::* |
| SCNParticleBirthLocation | enum | SCNParticleSystem.h | symbols::* |
| SCNParticleBlendMode | enum | SCNParticleSystem.h | symbols::* |
| SCNParticleEvent | enum | SCNParticleSystem.h | symbols::* |
| SCNParticleImageSequenceAnimationMode | enum | SCNParticleSystem.h | symbols::* |
| SCNParticleInputMode | enum | SCNParticleSystem.h | symbols::* |
| SCNParticleModifierStage | enum | SCNParticleSystem.h | symbols::* |
| SCNParticleOrientationMode | enum | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyAngle | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyAngularVelocity | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyBounce | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyCharge | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyColor | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyContactNormal | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyContactPoint | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyController | interface | SCNParticleSystem.h | extended_geometry::* |
| SCNParticlePropertyFrame | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyFrameRate | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyFriction | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyLife | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyOpacity | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyPosition | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyRotationAxis | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertySize | const | SCNParticleSystem.h | symbols::* |
| SCNParticlePropertyVelocity | const | SCNParticleSystem.h | symbols::* |
| SCNParticleSortingMode | enum | SCNParticleSystem.h | symbols::* |
| SCNParticleSystem | interface | SCNParticleSystem.h | ParticleSystem |
| SCNPhysicsBallSocketJoint | interface | SCNPhysicsBehavior.h | extended_physics::* |
| SCNPhysicsBehavior | interface | SCNPhysicsBehavior.h | extended_physics::* |
| SCNPhysicsBody | interface | SCNPhysicsBody.h | PhysicsBody |
| SCNPhysicsBodyType | enum | SCNPhysicsBody.h | PhysicsBodyType |
| SCNPhysicsCollisionCategory | options | SCNPhysicsBody.h | symbols::* |
| SCNPhysicsConeTwistJoint | interface | SCNPhysicsBehavior.h | extended_physics::* |
| SCNPhysicsContact | interface | SCNPhysicsContact.h | PhysicsContact |
| SCNPhysicsContactDelegate | protocol | SCNPhysicsWorld.h | PhysicsContactDelegate |
| SCNPhysicsField | interface | SCNPhysicsField.h | extended_physics::* |
| SCNPhysicsFieldScope | enum | SCNPhysicsField.h | symbols::* |
| SCNPhysicsHingeJoint | interface | SCNPhysicsBehavior.h | extended_physics::* |
| SCNPhysicsShape | interface | SCNPhysicsShape.h | extended_physics::* |
| SCNPhysicsShapeKeepAsCompoundKey | const | SCNPhysicsShape.h | symbols::* |
| SCNPhysicsShapeOptionCollisionMargin | const | SCNPhysicsShape.h | symbols::* |
| SCNPhysicsShapeScaleKey | const | SCNPhysicsShape.h | symbols::* |
| SCNPhysicsShapeTypeBoundingBox | const | SCNPhysicsShape.h | symbols::* |
| SCNPhysicsShapeTypeConcavePolyhedron | const | SCNPhysicsShape.h | symbols::* |
| SCNPhysicsShapeTypeConvexHull | const | SCNPhysicsShape.h | symbols::* |
| SCNPhysicsShapeTypeKey | const | SCNPhysicsShape.h | symbols::* |
| SCNPhysicsSliderJoint | interface | SCNPhysicsBehavior.h | extended_physics::* |
| SCNPhysicsTestBackfaceCullingKey | const | SCNPhysicsWorld.h | physics_world::physics_test_backface_culling_key |
| SCNPhysicsTestCollisionBitMaskKey | const | SCNPhysicsWorld.h | physics_world::physics_test_collision_bit_mask_key |
| SCNPhysicsTestSearchModeAll | const | SCNPhysicsWorld.h | physics_world::physics_test_search_mode_all |
| SCNPhysicsTestSearchModeAny | const | SCNPhysicsWorld.h | physics_world::physics_test_search_mode_any |
| SCNPhysicsTestSearchModeClosest | const | SCNPhysicsWorld.h | physics_world::physics_test_search_mode_closest |
| SCNPhysicsTestSearchModeKey | const | SCNPhysicsWorld.h | physics_world::physics_test_search_mode_key |
| SCNPhysicsVehicle | interface | SCNPhysicsBehavior.h | extended_physics::* |
| SCNPhysicsVehicleWheel | interface | SCNPhysicsBehavior.h | extended_physics::* |
| SCNPhysicsWorld | interface | SCNPhysicsWorld.h | Scene::physics_world / PhysicsWorld |
| SCNPlane | interface | SCNParametricGeometry.h | Geometry::plane |
| SCNPreferLowPowerDeviceKey | const | SCNView.h | symbols::* |
| SCNPreferredDeviceKey | const | SCNView.h | symbols::* |
| SCNPreferredRenderingAPIKey | const | SCNView.h | symbols::* |
| SCNProgram | interface | SCNShadable.h | Program |
| SCNProgramDelegate | protocol | SCNShadable.h | ProgramDelegate |
| SCNProgramMappingChannelKey | const | SCNShadable.h | program::program_mapping_channel_key |
| SCNProjectionTransform | const | SCNNode.h | symbols::* |
| SCNPyramid | interface | SCNParametricGeometry.h | extended_geometry::* |
| SCNReferenceLoadingPolicy | enum | SCNReferenceNode.h | symbols::* |
| SCNReferenceNode | interface | SCNReferenceNode.h | extended_geometry::* |
| SCNRenderer | interface | SCNRenderer.h | Renderer |
| SCNRenderingAPI | enum | SCNSceneRenderer.h | scene_renderer::RenderingAPI |
| SCNReplicatorConstraint | interface | SCNConstraint.h | extended_constraints::* |
| SCNScene | interface | SCNScene.h | Scene |
| SCNSceneEndTimeAttributeKey | const | SCNScene.h | symbols::* |
| SCNSceneExportDelegate | protocol | SCNScene.h | delegates::* |
| SCNSceneExportDestinationURL | const | SCNScene.h | symbols::* |
| SCNSceneFrameRateAttributeKey | const | SCNScene.h | symbols::* |
| SCNSceneRenderer | protocol | SCNSceneRenderer.h | SceneRenderer for Renderer and View |
| SCNSceneRendererDelegate | protocol | SCNSceneRenderer.h | SceneRendererDelegate |
| SCNSceneSource | interface | SCNSceneSource.h | SceneSource |
| SCNSceneSourceAnimationImportPolicyDoNotPlay | const | SCNSceneSource.h | scene_source::scene_source_animation_import_policy_do_not_play |
| SCNSceneSourceAnimationImportPolicyKey | const | SCNSceneSource.h | scene_source::scene_source_animation_import_policy_key |
| SCNSceneSourceAnimationImportPolicyPlay | const | SCNSceneSource.h | scene_source::scene_source_animation_import_policy_play |
| SCNSceneSourceAnimationImportPolicyPlayRepeatedly | const | SCNSceneSource.h | scene_source::scene_source_animation_import_policy_play_repeatedly |
| SCNSceneSourceAnimationImportPolicyPlayUsingSceneTimeBase | const | SCNSceneSource.h | scene_source::scene_source_animation_import_policy_play_using_scene_time_base |
| SCNSceneSourceAssetAuthoringToolKey | const | SCNSceneSource.h | scene_source::scene_source_asset_authoring_tool_key |
| SCNSceneSourceAssetAuthorKey | const | SCNSceneSource.h | scene_source::scene_source_asset_author_key |
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
| SCNSceneStartTimeAttributeKey | const | SCNScene.h | symbols::* |
| SCNSceneUpAxisAttributeKey | const | SCNScene.h | symbols::* |
| SCNShadable | protocol | SCNShadable.h | Shadable for Material and Geometry |
| SCNShaderModifierEntryPointFragment | const | SCNShadable.h | program::shader_modifier_entry_point_fragment |
| SCNShaderModifierEntryPointGeometry | const | SCNShadable.h | program::shader_modifier_entry_point_geometry |
| SCNShaderModifierEntryPointLightingModel | const | SCNShadable.h | program::shader_modifier_entry_point_lighting_model |
| SCNShaderModifierEntryPointSurface | const | SCNShadable.h | program::shader_modifier_entry_point_surface |
| SCNShadowMode | enum | SCNLight.h | ShadowMode |
| SCNShape | interface | SCNParametricGeometry.h | extended_geometry::* |
| SCNSkinner | interface | SCNSkinner.h | extended_geometry::* |
| SCNSliderConstraint | interface | SCNConstraint.h | extended_constraints::* |
| SCNSphere | interface | SCNParametricGeometry.h | Geometry::sphere |
| SCNTechnique | interface | SCNTechnique.h | Technique |
| SCNTechniqueSupport | protocol | SCNTechnique.h | protocols::* |
| SCNTessellationSmoothingMode | enum | SCNGeometry.h | symbols::* |
| SCNText | interface | SCNParametricGeometry.h | Geometry::text |
| SCNTimingFunction | interface | SCNAnimation.h | protocols::* |
| SCNTorus | interface | SCNParametricGeometry.h | extended_geometry::* |
| SCNTransaction | interface | SCNTransaction.h | Transaction |
| SCNTransformConstraint | interface | SCNConstraint.h | extended_constraints::* |
| SCNTransparencyMode | enum | SCNMaterial.h | symbols::* |
| SCNTube | interface | SCNParametricGeometry.h | extended_geometry::* |
| SCNVector3 | struct | SceneKitTypes.h | Vector3 |
| SCNVector3EqualToVector3 | function | SceneKitTypes.h | symbols::* |
| SCNVector3Zero | const | SceneKitTypes.h | symbols::* |
| SCNVector4 | struct | SceneKitTypes.h | Vector4 |
| SCNVector4EqualToVector4 | function | SceneKitTypes.h | symbols::* |
| SCNVector4Zero | const | SceneKitTypes.h | symbols::* |
| SCNView | interface | SCNView.h | View |
| SCNViewTransform | const | SCNNode.h | symbols::* |
| SCNWrapMode | enum | SCNMaterialProperty.h | symbols::* |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| _None_ | — | — | All non-exempt public symbols from the audited SDK headers are now wrapped. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| SCNLayer | interface | SCNLayer.h | OpenGL-backed layer is deprecated on macOS and intentionally skipped. | SCN_GL_DEPRECATED_MAC(10_8, 10_14) + API_UNAVAILABLE(ios, tvos, watchos, macCatalyst, visionos) |
| SCNLightAttenuationEndKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(macos(10.8, 10.10)) + API_UNAVAILABLE(ios, tvos, watchos, macCatalyst, visionos) |
| SCNLightAttenuationFalloffExponentKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(macos(10.8, 10.10)) + API_UNAVAILABLE(ios, tvos, watchos, macCatalyst, visionos) |
| SCNLightAttenuationStartKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(macos(10.8, 10.10)) + API_UNAVAILABLE(ios, tvos, watchos, macCatalyst, visionos) |
| SCNLightShadowFarClippingKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(macos(10.8, 10.10)) + API_UNAVAILABLE(ios, tvos, watchos, macCatalyst, visionos) |
| SCNLightShadowNearClippingKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(macos(10.8, 10.10)) + API_UNAVAILABLE(ios, tvos, watchos, macCatalyst, visionos) |
| SCNLightSpotInnerAngleKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(macos(10.8, 10.10)) + API_UNAVAILABLE(ios, tvos, watchos, macCatalyst, visionos) |
| SCNLightSpotOuterAngleKey | const | SceneKitDeprecated.h | Deprecated light attribute key skipped by policy. | API_DEPRECATED(macos(10.8, 10.10)) + API_UNAVAILABLE(ios, tvos, watchos, macCatalyst, visionos) |
| SCNSceneSourceUseSafeModeKey | const | SCNSceneSource.h | Deprecated scene-source loading option skipped by policy. | API_DEPRECATED(macos(10.8, 10.13), ios(8.0, 11.0), tvos(9.0, 11.0), watchos(3.0, 4.0)) |
