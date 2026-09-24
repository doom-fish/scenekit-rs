use std::sync::{Arc, Mutex};

use apple_metal::{pixel_format, MetalDevice};
use scenekit::{
    DebugOptions, Renderer, RenderingAPI, SceneRenderer, SceneRendererDelegate,
    SceneRendererDelegateCallbacks,
};

mod common;

#[test]
fn test_scene_renderer_trait_and_delegate_round_trip() {
    let device = MetalDevice::system_default().expect("device");
    let (scene, _root, camera_node) = common::scene_with_camera().expect("scene setup");

    let renderer = Renderer::new(Some(&device)).expect("renderer");
    SceneRenderer::set_scene(&renderer, Some(&scene));
    SceneRenderer::set_point_of_view(&renderer, Some(&camera_node));
    SceneRenderer::set_scene_time(&renderer, 1.25);
    SceneRenderer::set_loops(&renderer, true);
    SceneRenderer::set_playing(&renderer, true);
    SceneRenderer::set_autoenables_default_lighting(&renderer, true);
    SceneRenderer::set_jittering_enabled(&renderer, true);
    SceneRenderer::set_temporal_antialiasing_enabled(&renderer, true);
    SceneRenderer::set_shows_statistics(&renderer, true);
    SceneRenderer::set_debug_options(
        &renderer,
        DebugOptions::SHOW_BOUNDING_BOXES | DebugOptions::SHOW_CAMERAS,
    );

    assert!(SceneRenderer::scene(&renderer).is_some());
    assert!((SceneRenderer::scene_time(&renderer) - 1.25).abs() < f64::EPSILON);
    assert!(SceneRenderer::loops(&renderer));
    assert!(SceneRenderer::is_playing(&renderer));
    assert!(SceneRenderer::autoenables_default_lighting(&renderer));
    assert!(SceneRenderer::jittering_enabled(&renderer));
    assert!(SceneRenderer::temporal_antialiasing_enabled(&renderer));
    assert!(SceneRenderer::shows_statistics(&renderer));
    assert!(SceneRenderer::debug_options(&renderer).contains(DebugOptions::SHOW_CAMERAS));
    assert_eq!(
        SceneRenderer::rendering_api(&renderer),
        Some(RenderingAPI::Metal)
    );

    let events = Arc::new(Mutex::new(Vec::new()));
    let delegate = SceneRendererDelegate::new(
        SceneRendererDelegateCallbacks::new()
            .on_update({
                let events = Arc::clone(&events);
                move |_| events.lock().expect("events").push("update")
            })
            .on_will_render_scene({
                let events = Arc::clone(&events);
                move |_, _| events.lock().expect("events").push("will-render")
            })
            .on_did_render_scene({
                let events = Arc::clone(&events);
                move |_, _| events.lock().expect("events").push("did-render")
            }),
    )
    .expect("delegate");
    SceneRenderer::set_delegate(&renderer, Some(&delegate));
    assert!(SceneRenderer::delegate(&renderer).is_some());

    let texture = common::render_target(&device, 16, pixel_format::BGRA8UNORM).expect("texture");
    let error = common::render_frame(&device, &renderer, &texture, 1.25)
        .expect_err("temporal antialiasing with jittering aborts inside SceneKit");
    assert!(error.to_string().contains("jittering"), "{error}");
    assert!(events.lock().expect("events").is_empty());
    SceneRenderer::set_jittering_enabled(&renderer, false);
    common::render_frame(&device, &renderer, &texture, 1.25).expect("render");

    let events = events.lock().expect("events").clone();
    let first = |name: &str| events.iter().position(|event| *event == name);
    let update = first("update").expect("update");
    let will_render = first("will-render").expect("will-render");
    let did_render = first("did-render").expect("did-render");
    assert!(
        update < will_render && will_render < did_render,
        "{events:?}"
    );
}
