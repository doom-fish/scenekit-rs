# scenekit-rs

Safe Rust bindings for Apple's [SceneKit](https://developer.apple.com/documentation/scenekit) framework on macOS.

> **Status:** v0.2.0 adds dedicated Swift bridge files, Rust modules, examples, and tests for `SCNScene`, `SCNNode`, `SCNCamera`, `SCNLight`, `SCNGeometry`, `SCNMaterial`, `SCNAnimation`, `SCNPhysics`, `SCNView`, `SCNRenderer`, `SCNAction`, `SCNTransaction`, `SCNConstraint`, `SCNParticleSystem`, `SCNAudioPlayer`, `SCNHitTest`, and `SCNTechnique`.

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
            pixel_format: pixel_format::BGRA8UNORM,
            width: 256,
            height: 256,
            mipmapped: false,
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
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
    command_buffer.commit();
    command_buffer.wait_until_completed();

    let pixels = scenekit::read_texture_bytes(&texture)?;
    assert!(pixels.iter().any(|&byte| byte != 0));
    Ok(())
}
```

## Highlights

- Scene graph construction with `Scene`, `Node`, `Camera`, `Light`, `Geometry`, and `Material`
- Animation and action playback through `Animation`, `AnimationPlayer`, and `Action`
- Physics, constraints, particles, and audio attachment helpers on `Node`
- Headless-safe `View` snapshots, `HitTest` helpers, and minimal `Technique` support
- `Transaction` helpers for implicit animation scopes
- Offline `Renderer` + `RenderPassDescriptor` integration for `apple-metal`

## Examples and tests

The crate ships with 17 numbered examples and 17 integration test files. To run the full verification suite:

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
