# SceneKit coverage audit for `scenekit-rs` v0.2.0

This audit tracks the requested logical SceneKit areas for the crate's v0.2.0 expansion. Each requested area now has a dedicated Swift bridge file, a Rust module, at least one example, and at least one integration test.

## Status matrix

| Logical area | Status | Notes |
| --- | --- | --- |
| `SCNScene` | ✅ implemented | Scene creation/loading plus root node, background, lighting environment, and fog color accessors. |
| `SCNNode` | ✅ implemented | Hierarchy, naming, transforms, hidden state, geometry/light/camera/physics attachments, and action execution. |
| `SCNCamera` | ✅ implemented | Field of view, near/far clipping planes, and projection matrix round-tripping. |
| `SCNLight` | ✅ implemented | Light type, color, intensity, shadow mode, and shadow-casting controls. |
| `SCNGeometry` | ✅ implemented | Primitive constructors, `SCNText`, `SCNFloor`, `MDLMesh` import, and first-material access. |
| `SCNMaterial` | ✅ implemented | Core material channels with color / texture / file URL contents and intensity management. |
| `SCNAnimation` | ✅ implemented | Headless-safe `SCNAnimation` opacity creation, repeat/autoreverse/scene-time toggles, `SCNAnimationPlayer`, and node attachment helpers. |
| `SCNPhysics` | ✅ implemented | `SCNPhysicsBody` creation, type switching, mass/restitution/friction, and force application. |
| `SCNView` | ✅ implemented | `SCNView` creation, scene/point-of-view assignment, camera control, continuous rendering, background color, preferred FPS, and snapshot sizing. |
| `SCNRenderer` | ✅ implemented | Offline renderer creation, scene / POV assignment, Metal render pass bridging, and texture readback. |
| `SCNAction` | ✅ implemented | Move/rotate/scale, sequencing/grouping/repeat, and Rust callback-backed custom actions. |
| `SCNTransaction` | ✅ implemented | Begin/commit/flush plus animation duration and action-disabling transaction globals. |
| `SCNConstraint` | ✅ implemented | `SCNLookAtConstraint`, `SCNDistanceConstraint`, influence factor, gimbal lock, distance limits, and node constraint assignment. |
| `SCNParticleSystem` | ✅ implemented | Particle system creation, birth rate/life span/looping, and node attachment/removal helpers. |
| `SCNAudioPlayer` | ✅ implemented | URL-backed `SCNAudioSource`, volume/positional/looping controls, `SCNAudioPlayer`, and node attachment/removal helpers. |
| `SCNHitTest` | ✅ implemented | `SCNView` hit-test entry point plus retained result collections, hit nodes, and world coordinates. |
| `SCNTechnique` | ✅ implemented | Minimal draw-scene technique creation, dictionary key counting, float symbol KVC, and `SCNView.technique` assignment. |

## Deferred broader SceneKit surface

These broader SceneKit APIs remain out of scope for v0.2.0 and are intentionally deferred rather than silently omitted:

1. `SCNSceneSource`, `SCNReferenceNode`, `SCNLevelOfDetail`, `SCNSkinner`, and `SCNMorpher` asset-pipeline wrappers.
2. `SCNPhysicsWorld`, `SCNPhysicsField`, `SCNPhysicsBehavior`, `SCNPhysicsShape`, and contact-delegate bridging.
3. Advanced `SCNAnimation` events, timing functions, and `CAAnimationGroup`/keyframe import helpers.
4. Interactive `SCNView` delegate, playback, camera-controller-configuration, and event-handling APIs.
5. Full `SCNTechnique` multi-pass shader/library/binding-block configuration beyond the minimal dictionary + symbol bridge.
6. File-based `SCNParticleSystem` emitters and the wider particle modifier/event API surface.
7. `SCNAudioPlayer` playback callback hooks and custom `AVAudioNode` initialization paths.
