# scenekit-rs

Safe Rust bindings for Apple's [SceneKit](https://developer.apple.com/documentation/scenekit) framework on macOS.

> **SceneKit status:** at WWDC25 ([session 288](https://developer.apple.com/videos/play/wwdc2025/288/)) Apple announced a soft deprecation of SceneKit on all platforms. Existing apps keep working, SceneKit is in maintenance mode with critical bug fixes only, and Apple recommends RealityKit for new apps and significant updates. Apple's documentation lists SceneKit as deprecated in 26.0; the macOS 26.5 SDK headers do not mark its classes `API_DEPRECATED`.

> **Coverage:** `COVERAGE_AUDIT.md` counts top-level SDK symbols (classes, protocols, enums, constants, C helpers): 246 of the 255 on the macOS 26.2 SDK are named by a Rust item. A symbol counts as covered even when only a few of its methods and properties are wrapped, so method-level coverage is much thinner than that figure suggests.

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
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    renderer.render(
        0.0,
        CGRect::new(0.0, 0.0, 256.0, 256.0),
        &command_buffer,
        &pass,
    );
    command_buffer.commit()?;
    command_buffer.wait_until_completed()?;

    let pixels = scenekit::read_texture_bytes(&texture)?;
    assert!(pixels.iter().any(|&byte| byte != 0));
    Ok(())
}
```

## Threads, callbacks and lifetimes

- `View` (`SCNView`) is an `NSView`: `View::new` returns an error off the main thread, and the view's bridge calls do nothing there. `View` is neither `Send` nor `Sync`.
- SceneKit calls renderer, node-renderer, physics, avoid-occluder, program and animation callbacks on its rendering thread, so every callback closure must be `Send`. Each delegate's closures sit behind a mutex; a callback that re-enters its own delegate on the same thread is skipped instead of deadlocking.
- `SCNNode.rendererDelegate`, `SCNAvoidOccluderConstraint.delegate`, `SCNCameraController.delegate` and `SCNProgram.delegate` are unretained (`assign`) in the SDK. The node, constraint, controller or program keeps the delegate object alive while it is set (clones made with `Node::clone_node` do too), so SceneKit never messages freed memory. Dropping the Rust delegate handle deactivates its callbacks; setting the property to `None` releases them.
- `SceneRendererDelegate` and `PhysicsContactDelegate` are held weakly by SceneKit and stop when their handle is dropped.
- Node and renderer arguments are borrowed for the duration of a callback and are not retained per call.

## Validation

- `GeometryElement::with_data` accepts 1-, 2- or 4-byte indices only, and the data length must equal the index count implied by the primitive type and count (polygon data starts with one vertex count, of at least 3, per polygon). `Geometry::with_sources_elements` rejects indices at or beyond the vector count of the smallest source.
- `GeometrySource::with_data` checks the layout (1–4 components, float components of 4 or 8 bytes, integer components of 1, 2 or 4 bytes, stride and offset) against the data length.
- `Skinner::new` checks that bone weights and indices describe the base geometry's vertices and that every bone index is below the bone count. SceneKit needs inverse bind transforms; when none are given, each bone's current world transform is inverted.
- `read_texture_bytes` sizes its buffer from the pixel format (for example 8 bytes per pixel for `RGBA16Float` and `BGRA10_XR`) and rejects compressed, depth, stencil and framebuffer-only textures, textures that are not 2D, and private or memoryless storage. A managed texture must be synchronized before its bytes are current.

## Highlights

- Scene graph construction with `Scene`, `Node`, `Camera`, `Light`, `Geometry`, and `Material`, including node hierarchy queries, clones, world transforms, opacity and category masks
- Animation and action playback through `Animation`, `AnimationPlayer`, and `Action`, including `Action::custom`
- Physics, constraints, particles, audio, morpher/skinner, and reference-node helpers across `Node`, `Scene`, and `PhysicsWorld`
- `SCNSceneRenderer` protocol methods for presentation, frustum queries, project/unproject, prepare helpers, overlays, audio listener, reverse-Z, and delegates
- Custom geometry from raw vertex, normal, colour, tangent, crease and bone data
- Scene loading options (`SceneSourceOptions`) for `SceneSource` and `Scene::from_url_with_options`
- Scene export with an image-writing delegate
- Offline `Renderer` + `RenderPassDescriptor` integration for `apple-metal`

## Examples and tests

The crate ships with 20 numbered examples and 27 integration test files. `tests/main_thread.rs` uses its own harness so that its `SCNView` tests run on the main thread. To run the full verification suite:

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
