#![allow(dead_code)]

use std::error::Error;
use std::ffi::{c_char, c_void, CStr};
use std::path::Path;

use apple_cf::cg::CGRect;
use apple_metal::{
    pixel_format, storage_mode, texture_usage, CommandQueue, MetalDevice, MetalTexture,
    TextureDescriptor,
};
use scenekit::{
    read_texture_bytes, Camera, Color, Geometry, Light, LightType, Node, RenderPassDescriptor,
    Renderer, Scene, Vector3, View,
};

extern "C" {
    pub fn scn_node_test_invoke_renderer_delegate(node: *mut c_void, renderer: *mut c_void);
    pub fn scn_avoid_occluder_constraint_test_invoke_should(
        constraint: *mut c_void,
        occluder: *mut c_void,
        node: *mut c_void,
    ) -> bool;
    pub fn scn_avoid_occluder_constraint_test_invoke_did(
        constraint: *mut c_void,
        occluder: *mut c_void,
        node: *mut c_void,
    );
    pub fn scn_camera_controller_test_invoke_delegate_inertia_will_start(controller: *mut c_void);
    pub fn scn_camera_controller_test_invoke_delegate_inertia_did_end(controller: *mut c_void);
    pub fn scn_scene_renderer_test_invoke_delegate_update(renderer: *mut c_void, time: f64);
    pub fn scn_scene_renderer_test_invoke_delegate_will_render_scene(
        renderer: *mut c_void,
        time: f64,
    );
    pub fn scn_scene_renderer_test_invoke_delegate_did_render_scene(
        renderer: *mut c_void,
        time: f64,
    );
    pub fn scn_physics_world_test_invoke_delegate_did_begin(world: *mut c_void);
    pub fn scn_physics_world_test_invoke_delegate_did_update(world: *mut c_void);
    pub fn scn_physics_world_test_invoke_delegate_did_end(world: *mut c_void);

    fn objc_msgSend();
    fn objc_getClass(name: *const c_char) -> *mut c_void;
    fn sel_registerName(name: *const c_char) -> *mut c_void;
    fn objc_autoreleasePoolPush() -> *mut c_void;
    fn objc_autoreleasePoolPop(pool: *mut c_void);
}

fn selector(name: &CStr) -> *mut c_void {
    unsafe { sel_registerName(name.as_ptr()) }
}

pub fn send_object(receiver: *mut c_void, name: &CStr) -> *mut c_void {
    let send = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(),
            unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
        >(objc_msgSend)
    };
    unsafe { send(receiver, selector(name)) }
}

pub fn send_with_object(receiver: *mut c_void, name: &CStr, argument: *mut c_void) {
    let send = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(),
            unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
        >(objc_msgSend)
    };
    unsafe { send(receiver, selector(name), argument) };
}

pub fn send_with_two_objects(
    receiver: *mut c_void,
    name: &CStr,
    first: *mut c_void,
    second: *mut c_void,
) {
    let send = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(),
            unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void),
        >(objc_msgSend)
    };
    unsafe { send(receiver, selector(name), first, second) };
}

pub fn send_with_three_objects(
    receiver: *mut c_void,
    name: &CStr,
    first: *mut c_void,
    second: *mut c_void,
    third: *mut c_void,
) -> bool {
    let send = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(),
            unsafe extern "C" fn(
                *mut c_void,
                *mut c_void,
                *mut c_void,
                *mut c_void,
                *mut c_void,
            ) -> bool,
        >(objc_msgSend)
    };
    unsafe { send(receiver, selector(name), first, second, third) }
}

pub fn send_void_with_three_objects(
    receiver: *mut c_void,
    name: &CStr,
    first: *mut c_void,
    second: *mut c_void,
    third: *mut c_void,
) {
    let send = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(),
            unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void),
        >(objc_msgSend)
    };
    unsafe { send(receiver, selector(name), first, second, third) };
}

pub fn ns_error(domain: &CStr) -> *mut c_void {
    let string_with_utf8 = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(),
            unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_char) -> *mut c_void,
        >(objc_msgSend)
    };
    let error_with_domain = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(),
            unsafe extern "C" fn(
                *mut c_void,
                *mut c_void,
                *mut c_void,
                isize,
                *mut c_void,
            ) -> *mut c_void,
        >(objc_msgSend)
    };
    unsafe {
        let domain = string_with_utf8(
            objc_getClass(c"NSString".as_ptr()),
            selector(c"stringWithUTF8String:"),
            domain.as_ptr(),
        );
        error_with_domain(
            objc_getClass(c"NSError".as_ptr()),
            selector(c"errorWithDomain:code:userInfo:"),
            domain,
            -1,
            std::ptr::null_mut(),
        )
    }
}

pub fn autoreleasepool<R>(body: impl FnOnce() -> R) -> R {
    let pool = unsafe { objc_autoreleasePoolPush() };
    let result = body();
    unsafe { objc_autoreleasePoolPop(pool) };
    result
}

pub fn retain_count(object: *mut c_void) -> usize {
    let send = unsafe {
        std::mem::transmute::<
            unsafe extern "C" fn(),
            unsafe extern "C" fn(*mut c_void, *mut c_void) -> usize,
        >(objc_msgSend)
    };
    unsafe { send(object, sel_registerName(c"retainCount".as_ptr())) }
}

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
    let view = View::new(width, height)?;
    view.set_scene(Some(&scene));
    view.set_point_of_view(Some(&camera_node));
    Ok((view, scene, camera_node))
}

pub fn render_target(
    device: &MetalDevice,
    size: usize,
    format: usize,
) -> Result<MetalTexture, Box<dyn Error>> {
    Ok(device
        .new_texture(TextureDescriptor {
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
            ..TextureDescriptor::new_2d(size, size, format)
        })
        .ok_or("failed to create texture")?)
}

pub fn render_frame(
    device: &MetalDevice,
    renderer: &Renderer,
    texture: &MetalTexture,
    time: f64,
) -> Result<(), Box<dyn Error>> {
    let queue = device
        .new_command_queue()
        .ok_or("failed to create command queue")?;
    render_frame_on(&queue, renderer, texture, time)
}

pub fn render_frame_on(
    queue: &CommandQueue,
    renderer: &Renderer,
    texture: &MetalTexture,
    time: f64,
) -> Result<(), Box<dyn Error>> {
    let pass = RenderPassDescriptor::for_texture(texture, Color::black()).ok_or("missing pass")?;
    #[allow(clippy::cast_precision_loss)]
    let size = texture.width() as f64;
    let command_buffer = renderer.render(time, CGRect::new(0.0, 0.0, size, size), queue, &pass)?;
    command_buffer.commit()?;
    command_buffer.wait_until_completed()?;
    Ok(())
}

pub fn green_cube_scene() -> Result<(Scene, Node, Node), Box<dyn Error>> {
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
    Ok((scene, cube_node, camera_node))
}

pub fn renderer_smoke() -> Result<Vec<u8>, Box<dyn Error>> {
    let device = MetalDevice::system_default().ok_or("no Metal device")?;
    let texture = render_target(&device, 64, pixel_format::BGRA8UNORM)?;
    let (scene, _cube_node, camera_node) = green_cube_scene()?;
    let renderer = Renderer::new(Some(&device)).ok_or("missing renderer")?;
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));
    render_frame(&device, &renderer, &texture, 0.0)?;
    Ok(read_texture_bytes(&texture)?)
}

pub fn system_sound_path() -> &'static Path {
    Path::new("/System/Library/Sounds/Glass.aiff")
}
