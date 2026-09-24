# SceneKit coverage for `scenekit-rs` v0.3.0

## What the numbers measure

`COVERAGE_AUDIT.md` and `COVERAGE_AUDIT_V2.md` were generated against the macOS 26.2 SDK and have not been regenerated for 26.5. They list top-level public symbols only: classes, protocols, enums, option sets, structs, exported constants and exported C helpers. A symbol is **VERIFIED** when some Rust item names it, however few of its methods and properties are wrapped. Their "246/246" figure therefore says every non-exempt top-level symbol has a Rust counterpart; it does not say the SceneKit API is fully wrapped. The 9 exemptions are deprecated or OpenGL-only symbols (checked in `COVERAGE_AUDIT_V2.md`).

Apple announced a soft deprecation of SceneKit at WWDC25 (maintenance mode, critical bug fixes only); see the README.

## Method-level status

| Area | Status |
| --- | --- |
| Scene graph (`SCNScene`, `SCNNode`) | Construction, transforms, hierarchy (`child_nodes`, `parent`, `child_node_with_name`, `clone_node`), world transform, opacity, category mask, geometry/light/camera/physics attachments. Constraints can be set and counted but not read back. Not wrapped: flattened clones, presentation nodes and filters. |
| Geometry | Primitive shapes, vertex/normal/texture-coordinate sources, generic `SCNGeometrySource(data:semantic:...)` with layout validation, validated `SCNGeometryElement(data:...)`. Reading sources or elements back from a geometry is not wrapped. |
| Materials | A subset of material properties (diffuse, normal, specular, emission, ambient, transparent, multiply), colours, images, Metal textures, file URLs, intensity and the lighting model. Most other `SCNMaterial`/`SCNMaterialProperty` properties are not wrapped. |
| Skinning / morphing | `SCNSkinner` built from validated bone sources; `SCNMorpher` calculation mode only. |
| Scene loading | `SCNSceneSource` and `SCNScene(url:options:)` with the loading options in `SceneSourceOptions`; entry lookups return identifiers only. |
| `SCNSceneRenderer` | The protocol's properties and methods for presentation, projection, frustum queries, prepare, overlays, audio and delegates, on `Renderer` and `View`. |
| Delegates | Node renderer, avoid-occluder, camera controller, program, scene renderer, physics contact and scene export delegates, animation events and custom actions. SceneKit never sends `didAvoidOccluder` and sends program errors only from its OpenGL renderer, so those two callbacks do not run with this crate's Metal renderers. |
| Programs | `SCNProgram` source and Metal function names, `library` (set only), semantics, opacity, delegate and buffer bindings; buffer writes are checked against Metal's reflection of the named argument. |
| Physics | Static, dynamic and kinematic bodies with mass, restitution, friction, forces and category, collision and contact-test masks; physics world gravity, speed, time step, contact tests and the contact delegate; joints, fields and vehicles. |
| Offline rendering | `Renderer::render` into a new command buffer and `render_into` for an existing one (under apple-metal's command-buffer lock), both returning errors instead of aborting; `read_texture_bytes`. |
| Export | `SCNScene.write(to:)` with an export delegate that receives each image; the write result is reported. |

## Verification

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```
