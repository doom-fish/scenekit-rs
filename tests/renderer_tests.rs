mod common;

#[test]
fn test_renderer_offline_smoke() {
    let pixels = common::renderer_smoke().expect("renderer smoke");
    assert_eq!(pixels.len(), 64 * 64 * 4);
    let center = (32 * 64 + 32) * 4;
    let (blue, green, red, alpha) = (
        pixels[center],
        pixels[center + 1],
        pixels[center + 2],
        pixels[center + 3],
    );
    assert!(
        green > 32,
        "the lit green cube should cover the center pixel"
    );
    assert!(
        green > red && green > blue,
        "center pixel {blue},{green},{red} is not green"
    );
    assert_eq!(alpha, 255);
    assert_eq!(
        &pixels[0..3],
        &[0, 0, 0],
        "the corner keeps the black clear color"
    );
}

fn green_cube_renderer() -> (
    apple_metal::MetalDevice,
    scenekit::Renderer,
    apple_metal::MetalTexture,
    scenekit::Scene,
) {
    let device = apple_metal::MetalDevice::system_default().expect("device");
    let texture =
        common::render_target(&device, 16, apple_metal::pixel_format::BGRA8UNORM).expect("texture");
    let (scene, _cube_node, camera_node) = common::green_cube_scene().expect("scene");
    let renderer = scenekit::Renderer::new(Some(&device)).expect("renderer");
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));
    (device, renderer, texture, scene)
}

#[test]
fn render_returns_an_uncommitted_command_buffer() {
    let (device, renderer, texture, _scene) = green_cube_renderer();
    let queue = device.new_command_queue().expect("queue");
    let pass = scenekit::RenderPassDescriptor::for_texture(&texture, scenekit::Color::black())
        .expect("pass");
    let command_buffer = renderer
        .render(
            0.0,
            scenekit::CGRect::new(0.0, 0.0, 16.0, 16.0),
            &queue,
            &pass,
        )
        .expect("render");
    assert_eq!(
        command_buffer.status(),
        apple_metal::command_buffer_status::NOT_ENQUEUED
    );
    command_buffer.commit().expect("commit");
    command_buffer.wait_until_completed().expect("wait");
    assert_eq!(
        command_buffer.status(),
        apple_metal::command_buffer_status::COMPLETED
    );
}

#[test]
fn render_into_rejects_committed_command_buffers() {
    let (device, renderer, texture, _scene) = green_cube_renderer();
    let queue = device.new_command_queue().expect("queue");
    let pass = scenekit::RenderPassDescriptor::for_texture(&texture, scenekit::Color::black())
        .expect("pass");
    let viewport = scenekit::CGRect::new(0.0, 0.0, 16.0, 16.0);

    let enqueued = queue.new_command_buffer().expect("command buffer");
    enqueued.enqueue().expect("enqueue");
    unsafe { renderer.render_into(0.0, viewport, &enqueued, &pass) }
        .expect("an enqueued command buffer still accepts work");
    enqueued.commit().expect("commit");
    enqueued.wait_until_completed().expect("wait");

    let error = unsafe { renderer.render_into(0.1, viewport, &enqueued, &pass) }
        .expect_err("a completed command buffer must be rejected");
    assert!(error.to_string().contains("already committed"), "{error}");

    let committed = queue.new_command_buffer().expect("command buffer");
    committed.commit().expect("commit");
    let error = unsafe { renderer.render_into(0.2, viewport, &committed, &pass) }
        .expect_err("a committed command buffer must be rejected");
    assert!(error.to_string().contains("already committed"), "{error}");
    committed.wait_until_completed().expect("wait");
}

#[test]
fn render_rejects_temporal_antialiasing_with_jittering() {
    let (device, renderer, texture, _scene) = green_cube_renderer();
    let queue = device.new_command_queue().expect("queue");
    let pass = scenekit::RenderPassDescriptor::for_texture(&texture, scenekit::Color::black())
        .expect("pass");
    let viewport = scenekit::CGRect::new(0.0, 0.0, 16.0, 16.0);
    scenekit::SceneRenderer::set_temporal_antialiasing_enabled(&renderer, true);
    scenekit::SceneRenderer::set_jittering_enabled(&renderer, true);
    let Err(error) = renderer.render(0.0, viewport, &queue, &pass) else {
        panic!("SceneKit aborts on this combination, so render must refuse it");
    };
    assert!(error.to_string().contains("jittering"), "{error}");

    scenekit::SceneRenderer::set_jittering_enabled(&renderer, false);
    let command_buffer = renderer
        .render(0.0, viewport, &queue, &pass)
        .expect("temporal antialiasing alone renders");
    command_buffer.commit().expect("commit");
    command_buffer.wait_until_completed().expect("wait");
}
