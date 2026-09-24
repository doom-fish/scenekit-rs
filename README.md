# scenekit-rs

Safe Rust bindings for Apple's [SceneKit](https://developer.apple.com/documentation/scenekit) framework on macOS.

> **SceneKit status:** at WWDC25 ([session 288](https://developer.apple.com/videos/play/wwdc2025/288/)) Apple announced a soft deprecation of SceneKit on all platforms. Existing apps keep working, SceneKit is in maintenance mode with critical bug fixes only, and Apple recommends RealityKit for new apps and significant updates. Apple's documentation lists SceneKit as deprecated in 26.0; the macOS 26.5 SDK headers do not mark its classes `API_DEPRECATED`.

> **Coverage:** `COVERAGE_AUDIT.md` counts top-level SDK symbols (classes, protocols, enums, constants, C helpers): 246 of the 255 on the macOS 26.2 SDK are named by a Rust item. A symbol counts as covered even when only a few of its methods and properties are wrapped, so method-level coverage is much thinner than that figure suggests.

## Installation

```toml
[dependencies]
scenekit-rs = "0.3"
```

The library is imported as `scenekit`. It needs macOS 11 or later, Rust 1.82 or later, and the Xcode command-line tools (the build compiles a Swift bridge).

## Quick start

```rust,no_run
use apple_cf::cg::CGRect;
use apple_metal::{pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor};
use scenekit::{Camera, Color, Geometry, Light, LightType, Node, RenderPassDescriptor, Renderer, Scene, Vector3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = MetalDevice::system_default().expect("no Metal device");
    let queue = device.new_command_queue().expect("no command queue");
    let texture = device
        .new_texture(TextureDescriptor {
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
            ..TextureDescriptor::new_2d(256, 256, pixel_format::BGRA8UNORM)
        })
        .expect("texture");

    let scene = Scene::new().expect("scene");
    let root = scene.root_node();

    let cube = Geometry::box_geometry(1.0, 1.0, 1.0, 0.0).expect("box");
    cube.first_material().expect("material").diffuse().set_color(Color::green());
    let cube_node = Node::with_geometry(Some(&cube)).expect("cube node");
    root.add_child_node(&cube_node);

    let camera = Camera::new().expect("camera");
    let camera_node = Node::new().expect("camera node");
    camera_node.set_camera(Some(&camera));
    camera_node.set_position(Vector3::new(0.0, 0.0, 5.0));
    root.add_child_node(&camera_node);

    let light = Light::new().expect("light");
    light.set_light_type(LightType::Omni);
    let light_node = Node::new().expect("light node");
    light_node.set_light(Some(&light));
    light_node.set_position(Vector3::new(0.0, 2.0, 5.0));
    root.add_child_node(&light_node);

    let renderer = Renderer::new(Some(&device)).expect("renderer");
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));

    let pass = RenderPassDescriptor::for_texture(&texture, Color::black()).expect("pass");
    let command_buffer = renderer.render(0.0, CGRect::new(0.0, 0.0, 256.0, 256.0), &queue, &pass)?;
    command_buffer.commit()?;
    command_buffer.wait_until_completed()?;

    let pixels = scenekit::read_texture_bytes(&texture)?;
    assert!(pixels.iter().any(|&byte| byte != 0));
    Ok(())
}
```

## Threads, callbacks and lifetimes

- `View` (`SCNView`) is an `NSView`: `View::new` returns an error off the main thread, and the view's bridge calls do nothing there. `View` is neither `Send` nor `Sync`.
- SceneKit crashes when several threads create renderers at the same moment, so `Renderer::new` and `View::new` serialize renderer creation. Rendering itself is not serialized.
- SceneKit calls renderer, node-renderer, physics, avoid-occluder, program and animation callbacks on its rendering thread, so every callback closure must be `Send`. Each delegate's closures sit behind a mutex; a callback that re-enters its own delegate on the same thread is skipped instead of deadlocking.
- `SCNNode.rendererDelegate`, `SCNAvoidOccluderConstraint.delegate`, `SCNCameraController.delegate` and `SCNProgram.delegate` are unretained (`assign`) in the SDK, and SceneKit loads them on its rendering thread without retaining them. A delegate that another thread clears or replaces can therefore still be in use by a frame in flight, and SceneKit offers no way to wait for that frame. The node, constraint, controller or program therefore keeps every delegate object that was set on it alive until the owner itself is freed (clones made with `Node::clone_node` keep the delegates they copied). SceneKit stops calling a node's renderer delegate before the node is freed. Dropping the Rust delegate handle deactivates its callbacks at once; the small bridge object and the closure are freed with the owner, so replacing delegates many times on one long-lived owner holds one of them per delegate.
- `SceneRendererDelegate` and `PhysicsContactDelegate` are held weakly by SceneKit and stop when their handle is dropped.
- Node and renderer arguments are borrowed for the duration of a callback and are not retained per call.
- SceneKit applies delegate, buffer-binding and scene-graph changes through implicit `SCNTransaction`s that are committed by a running run loop. On a thread whose run loop does not run (a plain thread or a test driving an offline `Renderer`), call `Transaction::flush()` after such changes so the next frame sees them and SceneKit releases the objects the transaction holds.
- SceneKit never calls `SCNAvoidOccluderConstraintDelegate.didAvoidOccluder`, and it reports `SCNProgramDelegate` errors only from its OpenGL renderer, so those two callbacks do not run with the Metal renderers this crate creates.

## Offline rendering

- `Renderer::render` takes a `CommandQueue`, encodes the frame into a new command buffer and returns it uncommitted, so no other encoder or thread can use that buffer while SceneKit encodes. Add more work to it if needed, then commit it; `commit` and `wait_until_completed` report GPU errors.
- `Renderer::render_into` encodes into an existing command buffer through apple-metal's `CommandBuffer::encode_foreign`. It returns an error for a buffer that was already committed or failed, or that still has an apple-metal encoder open, and while SceneKit encodes, commits, enqueues and new encoders on that buffer fail with `ActiveEncoder` on every thread. Metal would abort the process in each of these cases.
- `render` and `render_into` return an error instead of letting SceneKit abort when temporal antialiasing and jittering are both enabled (SceneKit cannot create its history texture for an offline renderer), and when the command buffer, the renderer and the render target belong to different Metal devices.

## Validation

- `GeometryElement::with_data` accepts 1-, 2- or 4-byte indices only, and the data length must equal the index count implied by the primitive type and count (polygon data starts with one vertex count, of at least 3, per polygon). `Geometry::with_sources_elements` rejects indices at or beyond the vector count of the smallest source.
- `GeometrySource::with_data` checks the layout (1–4 components, float components of 4 or 8 bytes, integer components of 1, 2 or 4 bytes, stride and offset) against the data length.
- `Skinner::new` checks that bone weights and indices describe the base geometry's vertices and that every bone index is below the bone count. SceneKit needs inverse bind transforms; when none are given, each bone's current world transform is inverted.
- `BufferStream::write_bytes` (in a `ProgramBufferBinding` callback) checks the length against the size Metal reflection reports for the named buffer argument of the program's vertex and fragment functions (`MTLLibrary` function reflection on macOS 26 and later, a reflection pipeline before that), and rejects empty writes and writes the Metal device could not allocate. SceneKit copies each write into a new buffer and binds it, so a later write replaces an earlier one: write the whole argument at once. When a callback makes no valid write, or its binding was dropped or removed, the argument is bound as zeros instead of whatever the GPU finds. `BufferStream::required_length` and `maximum_length` report the limits, and the `unsafe` `write_bytes_unchecked` skips the size check for programs Metal cannot reflect. Give the program its shaders with `Program::set_library`.
- `read_texture_bytes` sizes its buffer from the pixel format (for example 8 bytes per pixel for `RGBA16Float` and `BGRA10_XR`) and rejects compressed, depth, stencil and framebuffer-only textures, textures that are not 2D, and private or memoryless storage. A managed texture must be synchronized before its bytes are current.

## Highlights

- Scene graph construction with `Scene`, `Node`, `Camera`, `Light`, `Geometry`, and `Material`, including node hierarchy queries, clones, world transforms, opacity and category masks
- Animation and action playback through `Animation`, `AnimationPlayer`, and `Action`, including `Action::custom`
- Physics, constraints, particles, audio, morpher/skinner, and reference-node helpers across `Node`, `Scene`, and `PhysicsWorld`, including physics-body category, collision and contact-test masks (SceneKit reports no contacts until a contact-test mask is set)
- `SCNSceneRenderer` protocol methods for presentation, frustum queries, project/unproject, prepare helpers, overlays, audio listener, reverse-Z, and delegates
- Custom geometry from raw vertex, normal, colour, tangent, crease and bone data
- Scene loading options (`SceneSourceOptions`) for `SceneSource` and `Scene::from_url_with_options`
- Scene export with an image-writing delegate
- Offline `Renderer` + `RenderPassDescriptor` integration for `apple-metal`

## Examples and tests

The crate ships with 20 numbered examples and 30 integration test files. `tests/main_thread.rs` uses its own harness so that its `SCNView` and camera-inertia tests run on the main thread. The Swift bridge exports no test-only entry points: the tests drive SceneKit for real (offline renders, physics simulation, camera inertia) and message a delegate directly only for the two callbacks SceneKit never sends. To run the full verification suite:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

The original offline render smoke test is still available as:

```bash
cargo run --example 01_offline_render_smoke
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
