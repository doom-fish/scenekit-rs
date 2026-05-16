#![allow(dead_code)]

use std::error::Error;
use std::path::Path;

use apple_cf::cg::CGRect;
use apple_metal::{pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor};
use scenekit::{
    read_texture_bytes, Camera, Color, Geometry, Light, LightType, Node, RenderPassDescriptor,
    Renderer, Scene, Vector3, View,
};

pub fn scene_with_camera() -> Result<(Scene, Node, Node), Box<dyn Error>> {
    let scene = Scene::new().ok_or("failed to create scene")?;
    let root = scene.root_node();

    let camera = Camera::new().ok_or("failed to create camera")?;
    let camera_node = Node::new().ok_or("failed to create camera node")?;
    camera_node.set_camera(Some(&camera));
    camera_node.set_position(Vector3::new(0.0, 0.0, 5.0));
    root.add_child_node(&camera_node);

    Ok((scene, root, camera_node))
}

pub fn view_with_camera(width: f64, height: f64) -> Result<(View, Scene, Node), Box<dyn Error>> {
    let (scene, _root, camera_node) = scene_with_camera()?;
    let view = View::new(width, height).ok_or("failed to create view")?;
    view.set_scene(Some(&scene));
    view.set_point_of_view(Some(&camera_node));
    Ok((view, scene, camera_node))
}

pub fn renderer_smoke() -> Result<(), Box<dyn Error>> {
    let device = MetalDevice::system_default().ok_or("no Metal device")?;
    let queue = device
        .new_command_queue()
        .ok_or("failed to create command queue")?;
    let texture = device
        .new_texture(TextureDescriptor {
            pixel_format: pixel_format::BGRA8UNORM,
            width: 64,
            height: 64,
            mipmapped: false,
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
        })
        .ok_or("failed to create texture")?;

    let (scene, root, camera_node) = scene_with_camera()?;
    let cube = Geometry::box_geometry(1.0, 1.0, 1.0, 0.0).ok_or("missing cube")?;
    cube.first_material()
        .ok_or("missing material")?
        .diffuse()
        .set_color(Color::green());
    let cube_node = Node::with_geometry(Some(&cube)).ok_or("missing cube node")?;
    root.add_child_node(&cube_node);

    let light = Light::new().ok_or("missing light")?;
    light.set_light_type(LightType::Omni);
    light.set_intensity(1_200.0);
    let light_node = Node::new().ok_or("missing light node")?;
    light_node.set_light(Some(&light));
    light_node.set_position(Vector3::new(0.0, 2.0, 5.0));
    root.add_child_node(&light_node);

    let renderer = Renderer::new(Some(&device)).ok_or("missing renderer")?;
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));
    let pass = RenderPassDescriptor::for_texture(&texture, Color::black()).ok_or("missing pass")?;
    let command_buffer = queue
        .new_command_buffer()
        .ok_or("failed to create command buffer")?;
    renderer.render(
        0.0,
        CGRect::new(0.0, 0.0, 64.0, 64.0),
        &command_buffer,
        &pass,
    );
    command_buffer.commit();
    command_buffer.wait_until_completed();

    let pixels = read_texture_bytes(&texture)?;
    assert!(pixels.iter().any(|&byte| byte != 0));
    Ok(())
}

pub fn system_sound_path() -> &'static Path {
    Path::new("/System/Library/Sounds/Glass.aiff")
}
