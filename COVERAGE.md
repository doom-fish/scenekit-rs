# SceneKit coverage audit for `scenekit-rs` v0.2.2

`scenekit-rs` v0.2.2 closes the audited non-exempt SceneKit SDK surface for macOS 26.2.

## Summary

- `COVERAGE_AUDIT.md` now reports **246/246 non-exempt public symbols wrapped**.
- The remaining 9 symbols stay explicitly exempt because they are deprecated/OpenGL-only APIs.
- `SCNSceneRenderer` now includes scene presentation, frustum queries, project/unproject helpers, prepare helpers, SpriteKit overlays/transitions, working color space access, Metal/audio handles, reverse-Z toggles, and delegate getter/setter coverage.
- Public protocol/delegate bridges now cover actionable, animatable, bounding-volume, technique-support, node-renderer, avoid-occluder, scene-export, and scene-renderer surfaces.

## Newly completed logical areas in v0.2.2

| Logical area | Status | Notes |
| --- | --- | --- |
| `SCNSceneRenderer` | ✅ complete | Full protocol-level surface plus tests for overlays, projection, frustum queries, export, and delegate bridges. |
| Advanced geometry / asset pipeline | ✅ complete | `SCNPyramid`, `SCNTube`, `SCNCapsule`, `SCNTorus`, `SCNShape`, `SCNGeometrySource`, `SCNGeometryElement`, `SCNGeometryTessellator`, `SCNLevelOfDetail`, `SCNMorpher`, `SCNReferenceNode`, `SCNSkinner`, and `SCNParticlePropertyController`. |
| Extended constraints | ✅ complete | `SCNBillboardConstraint`, `SCNTransformConstraint`, `SCNIKConstraint`, `SCNReplicatorConstraint`, `SCNAccelerationConstraint`, `SCNSliderConstraint`, and `SCNAvoidOccluderConstraint`. |
| Extended physics | ✅ complete | `SCNPhysicsBehavior`, `SCNPhysicsField`, `SCNPhysicsShape`, joints, vehicle/wheel wrappers, physics-world behavior helpers, and node physics-field helpers. |
| Protocols / delegates / export | ✅ complete | Public Rust bridges for `SCNActionable`, `SCNAnimatable`, `SCNBoundingVolume`, `SCNTechniqueSupport`, `SCNNodeRendererDelegate`, `SCNAvoidOccluderConstraintDelegate`, `SCNSceneExportDelegate`, `SCNTimingFunction`, `SCNAnimationEvent`, and `SCNExportJavaScriptModule`. |
| Constants / enums / C helpers | ✅ complete | Remaining audited SceneKit enums, option sets, exported constants, and math helpers are now exposed. |

## Verification

```bash
cargo clippy --all-targets -- -D warnings
cargo test
```
