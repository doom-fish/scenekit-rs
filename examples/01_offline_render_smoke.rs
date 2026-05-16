use apple_cf::cg::CGRect;
use apple_metal::{pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor};
use scenekit::{
    read_texture_bytes, Camera, Color, Geometry, Light, LightType, Node, RenderPassDescriptor,
    Renderer, Scene, Vector3,
};

const WIDTH: usize = 128;
const HEIGHT: usize = 128;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = MetalDevice::system_default().ok_or("no Metal device available")?;
    let queue = device
        .new_command_queue()
        .ok_or("failed to create command queue")?;
    let texture = device
        .new_texture(TextureDescriptor {
            pixel_format: pixel_format::BGRA8UNORM,
            width: WIDTH,
            height: HEIGHT,
            mipmapped: false,
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
        })
        .ok_or("failed to allocate render target texture")?;

    let scene = Scene::new().ok_or("failed to create scene")?;
    let root = scene.root_node();

    let cube = Geometry::box_geometry(1.0, 1.0, 1.0, 0.0).ok_or("failed to create box")?;
    let material = cube
        .first_material()
        .ok_or("box was missing first material")?;
    material.diffuse().set_color(Color::green());
    material.specular().set_color(Color::white());

    let cube_node = Node::with_geometry(Some(&cube)).ok_or("failed to create cube node")?;
    root.add_child_node(&cube_node);

    let camera = Camera::new().ok_or("failed to create camera")?;
    let camera_node = Node::new().ok_or("failed to create camera node")?;
    camera_node.set_camera(Some(&camera));
    camera_node.set_position(Vector3::new(0.0, 0.0, 5.0));
    root.add_child_node(&camera_node);

    let light = Light::new().ok_or("failed to create light")?;
    light.set_light_type(LightType::Omni);
    light.set_intensity(1_500.0);
    let light_node = Node::new().ok_or("failed to create light node")?;
    light_node.set_light(Some(&light));
    light_node.set_position(Vector3::new(0.0, 2.0, 5.0));
    root.add_child_node(&light_node);

    let renderer = Renderer::new(Some(&device)).ok_or("failed to create renderer")?;
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));

    let pass = RenderPassDescriptor::for_texture(&texture, Color::black())
        .ok_or("failed to create render pass descriptor")?;
    let command_buffer = queue
        .new_command_buffer()
        .ok_or("failed to create command buffer")?;
    renderer.render(
        0.0,
        CGRect::new(0.0, 0.0, 128.0, 128.0),
        &command_buffer,
        &pass,
    );
    command_buffer.commit();
    command_buffer.wait_until_completed();

    let pixels = read_texture_bytes(&texture)?;
    assert!(
        pixels.iter().any(|&byte| byte != 0),
        "rendered texture was all zeroes"
    );
    println!("✅ scenekit offline render OK");
    Ok(())
}
