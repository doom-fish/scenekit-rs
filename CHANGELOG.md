# Changelog

All notable changes to `scenekit-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - Unreleased

### Security

- `SCNNode.rendererDelegate`, `SCNAvoidOccluderConstraint.delegate`,
  `SCNCameraController.delegate` and `SCNProgram.delegate` are unretained
  (`assign`) in the SDK, and the setters did not retain the delegate. Dropping
  the Rust delegate while it was set left SceneKit messaging a freed object that
  then called into freed Rust state. The owner now retains the delegate object
  while it is set (clones made with `Node::clone_node` too, because SceneKit
  copies the unretained pointer into clones); setting `None` releases it.
- `read_texture_bytes` assumed 4 bytes per pixel and never told Swift how large
  its buffer was, so reading an `RGBA16Float`, `RGBA32Float` or `BGRA10_XR`
  texture overflowed the heap. The buffer is now sized with
  `apple_metal::bytes_per_pixel`, its length is passed to and checked by the
  bridge, and compressed, depth, stencil, framebuffer-only, non-2D, private and
  memoryless textures are rejected.
- `GeometryElement::with_data` did not check the data against the primitive
  count or `bytes_per_index`, so SceneKit and Metal read past the end of the
  index data while drawing and hit-testing. See Changed for the new rules.

### Fixed

- `renderNode` and the avoid-occluder callbacks passed +1 node and renderer
  pointers that were never released, leaking two objects per call per frame.
  They are now borrowed for the duration of the callback.
- Render-thread callbacks (scene renderer, node renderer, physics contact,
  avoid occluder, program, buffer binding, animation event, camera controller)
  took `FnMut + 'static` closures without `Send` and handed out an
  unsynchronised `&mut` to shared state. They now require `Send`, run behind a
  mutex, and a callback that re-enters its own delegate on the same thread is
  skipped instead of deadlocking.
- Callback contexts use `doom_fish_utils::callback_context::CallbackContext`:
  a trampoline holds a reference for the duration of its call, and dropping a
  delegate handle deactivates its callbacks.
- `SCNView` was created and changed on any thread. `View::new` now returns an
  error off the main thread and every `SCNView` bridge call (including the
  `SceneRenderer` and technique methods on a `View`) does nothing there.
- The scene-export delegate never received the image it was supposed to write,
  and `Scene::write_to_url` reported whether a file existed at the path rather
  than whether the write succeeded, so an old file reported success.
- `Skinner::new` passed no inverse bind transforms, which `SCNSkinner` rejects,
  so it never returned a skinner.
- `SceneRenderer::hit_test` returned an array the results type could not read,
  so it always reported no hits.
- Swift conversions that trapped on caller or SceneKit values: category bit
  masks above `Int.max` (for example `u64::MAX`) and negative framework masks,
  and a negative antialiasing mode.
- The `Vector3`, `Vector4` and `Matrix4` docs claimed the `SCNVector3`,
  `SCNVector4` and `SCNMatrix4` layouts; on macOS those are `CGFloat`-based and
  the bridge converts each component.
- The README and coverage files claimed the SceneKit surface was complete; the
  audit counts top-level symbols only. The README now also cites Apple's
  WWDC25 guidance that SceneKit is in maintenance mode.

### Changed

- **Breaking:** delegate and callback closures must be `Send`
  (`NodeRendererDelegateCallbacks`, `AvoidOccluderConstraintDelegateCallbacks`,
  `SceneRendererDelegateCallbacks`, `PhysicsContactDelegateCallbacks`,
  `CameraControllerDelegateCallbacks`, `ProgramDelegate`,
  `ProgramBufferBinding`, `AnimationEvent`, `SceneExportDelegate`).
- **Breaking:** `Action::custom` passes `&Node` instead of an owned,
  non-owning `Node`.
- **Breaking:** `SceneExportDelegate::new` takes
  `FnMut(&CGImage, &str, Option<&str>) -> Option<String> + Send`; the closure
  writes the image and returns where it did. `Scene::write_to_url` returns
  `Result<(), SceneKitError>`.
- **Breaking:** `View::new` returns `Result<View, SceneKitError>`.
- **Breaking:** `GeometryElement::with_data` returns `Result` and accepts
  1-, 2- or 4-byte indices whose data length equals the index count implied by
  the primitive type and count (polygon data starts with one vertex count of at
  least 3 per polygon); `None` data means implicit ordering except for
  polygons. `Geometry::with_sources_elements` returns `Result` and rejects
  indices at or beyond the vector count of the smallest source.
- **Breaking:** `Skinner::new(base_geometry, bones, bone_inverse_bind_transforms, bone_weights, bone_indices)`
  requires a base geometry, takes optional inverse bind transforms (each bone's
  inverted world transform by default) and returns `Result`; bone weights and
  indices must match the base geometry's vertex count and every bone index must
  be below the bone count.
- **Breaking:** the raw callback aliases in `ffi` (`ScnActionCallback`,
  `ScnDropCallback`, …) are `unsafe extern "C" fn`, and the scene loading and
  texture copy exports take option and length arguments.
- Requires `apple-cf >=0.11, <0.12`, `apple-metal >=0.10, <0.11` and
  `doom-fish-utils >=0.4.1, <0.5`, all by path to the local siblings;
  `rust-version` is 1.82 (was 1.76).

### Added

- `GeometrySource::with_data` with `GeometrySourceLayout` and
  `GeometrySourceSemantic` (vertices, normals, colours, texture coordinates,
  tangents, creases, bone weights and bone indices), and
  `GeometrySource::vector_count`.
- `Node::child_nodes`, `parent`, `child_node_with_name`, `clone_node`,
  `world_transform`/`set_world_transform`, `opacity`/`set_opacity` and
  `category_bit_mask`/`set_category_bit_mask`.
- `SceneSourceOptions` and `SceneSourceAnimationImportPolicy`, accepted by
  `SceneSource::from_url_with_options`, `from_data_with_options`,
  `scene_with_options` and `Scene::from_url_with_options`.
- `Material::lighting_model` and `set_lighting_model`.
- `CameraController::new`, `CameraController::delegate` and
  `Program::delegate`.

### Removed

- **Breaking:** the test-only hooks `Node::test_invoke_renderer_delegate`,
  `AvoidOccluderConstraint::test_invoke_should_avoid_occluder`,
  `test_invoke_did_avoid_occluder` and the `ffi::*_test_invoke_*`
  declarations. The tests declare the bridge symbols they need.

## [0.2.9] - 2026-06-06

- Pinned the `Vector3`/`Vector4`/`Matrix4`/`Color` FFI layouts with
  compile-time assertions and a Swift layout check, consolidated the delegate
  `Drop` impls, removed the empty bridge header, and fixed the geometry bridge
  reading 24-byte `SCNVector3` values from 12-byte `Vector3` buffers.

## [0.2.8] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.2.7] - 2026-05-18

- Added one-line rustdoc coverage across the public SceneKit wrapper surface, lifting the crate from an undocumented API surface to a fully documented one for rustc `missing_docs` checks.
- Documented helper newtypes, protocol bridges, constants, and module exports with SceneKit counterpart references.

## [0.2.6] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.2.5] - 2026-05-18

- Widen apple-metal version bound so the 0.x bump dep resolves. No source changes.

## [0.2.4] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

## [0.2.3] - 2025-05-17

- Added panic safety to all FFI callbacks using `catch_unwind`: `action_invoke`, `camera_controller` delegates, `node_renderer` delegate, `avoid_occluder_constraint` delegates, `scene_export` delegate, `physics_contact` delegates, `program` delegates, `animation_event` callbacks, and `scene_renderer` delegates

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
