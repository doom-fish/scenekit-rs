use std::cell::RefCell;
use std::rc::Rc;

use apple_metal::MetalDevice;
use scenekit::{
    ffi, AntialiasingMode, DebugOptions, Renderer, RenderingAPI, SceneRenderer,
    SceneRendererDelegate, SceneRendererDelegateCallbacks, View,
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

    let events = Rc::new(RefCell::new(Vec::new()));
    let delegate = SceneRendererDelegate::new(
        SceneRendererDelegateCallbacks::new()
            .on_update({
                let events = Rc::clone(&events);
                move |_| events.borrow_mut().push("update")
            })
            .on_will_render_scene({
                let events = Rc::clone(&events);
                move |_, _| events.borrow_mut().push("will-render")
            })
            .on_did_render_scene({
                let events = Rc::clone(&events);
                move |_, _| events.borrow_mut().push("did-render")
            }),
    )
    .expect("delegate");
    SceneRenderer::set_delegate(&renderer, Some(&delegate));

    unsafe {
        ffi::scn_scene_renderer_test_invoke_delegate_update(renderer.as_ptr(), 1.25);
        ffi::scn_scene_renderer_test_invoke_delegate_will_render_scene(renderer.as_ptr(), 1.25);
        ffi::scn_scene_renderer_test_invoke_delegate_did_render_scene(renderer.as_ptr(), 1.25);
    }

    let events = events.borrow();
    assert!(events.contains(&"update"));
    assert!(events.contains(&"will-render"));
    assert!(events.contains(&"did-render"));

    let view = View::new(80.0, 60.0).expect("view");
    view.set_antialiasing_mode(AntialiasingMode::Multisampling2X);
    assert_eq!(
        view.antialiasing_mode(),
        Some(AntialiasingMode::Multisampling2X)
    );
}
