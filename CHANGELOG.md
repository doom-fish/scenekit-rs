# Changelog

## [0.2.2] - 2026-05-17

- Closed the audited non-exempt SceneKit SDK surface: `COVERAGE_AUDIT.md` now reports 246/246 verified symbols with 9 explicit exemptions and no remaining gaps
- Expanded `SCNSceneRenderer` to cover scene presentation, hit/frustum queries, project/unproject, prepare helpers, SpriteKit overlays/transitions, working color space, Metal/audio handles, reverse-Z, and delegate getter/setter accessors
- Added public Rust bridges for `SCNActionable`, `SCNAnimatable`, `SCNBoundingVolume`, `SCNTechniqueSupport`, `SCNNodeRendererDelegate`, `SCNAvoidOccluderConstraintDelegate`, `SCNSceneExportDelegate`, `SCNTimingFunction`, `SCNAnimationEvent`, and `SCNExportJavaScriptModule`
- Added extended geometry, constraint, and physics wrappers including morpher/skinner/reference-node, advanced constraints, physics behaviors/joints/vehicles, and SpriteKit helper handles
- Added focused integration coverage for the new scene-renderer, delegate, export, and extended-physics surfaces

## [0.2.1] - 2026-05-16

- Added `SCNSceneRenderer`, `SCNSceneSource`, `SCNPhysicsWorld`, `SCNProgram`, and `SCNCameraController` bindings plus their delegate/constant surfaces
- Added integration tests, OBJ scene-source fixture coverage, and new camera-controller / scene-source / program examples
- Switched `apple-cf` and `apple-metal` to registry dependencies for reproducible builds outside the local sibling-checkout setup

## [0.2.0] - 2026-05-16

- Added dedicated `SCNAnimation`, `SCNTransaction`, `SCNConstraint`, `SCNParticleSystem`, `SCNAudioPlayer`, `SCNHitTest`, `SCNTechnique`, and `SCNView` bridges and Rust modules
- Added per-area examples and integration tests across the requested 17 logical SceneKit areas
- Added headless-safe view snapshots, hit testing, minimal technique symbols, animation players, and audio/particle/constraint node attachment helpers
- Kept the offline Metal renderer smoke workflow and bumped the crate to `v0.2.0`

## [0.1.0] - 2026-05-16

- Initial release of `scenekit-rs`
- Added SceneKit scene, node, geometry, material, camera, light, action, physics, and renderer bindings
- Added an offline Metal-render smoke example
