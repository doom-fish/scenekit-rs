# Changelog

## [0.2.0] - 2026-05-16

- Added dedicated `SCNAnimation`, `SCNTransaction`, `SCNConstraint`, `SCNParticleSystem`, `SCNAudioPlayer`, `SCNHitTest`, `SCNTechnique`, and `SCNView` bridges and Rust modules
- Added per-area examples and integration tests across the requested 17 logical SceneKit areas
- Added headless-safe view snapshots, hit testing, minimal technique symbols, animation players, and audio/particle/constraint node attachment helpers
- Kept the offline Metal renderer smoke workflow and bumped the crate to `v0.2.0`

## [0.1.0] - 2026-05-16

- Initial release of `scenekit-rs`
- Added SceneKit scene, node, geometry, material, camera, light, action, physics, and renderer bindings
- Added an offline Metal-render smoke example
