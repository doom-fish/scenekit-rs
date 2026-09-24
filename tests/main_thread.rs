use std::ffi::c_void;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use scenekit::{
    AntialiasingMode, CGPoint, CGSize, Camera, CameraController, CameraControllerDelegate,
    CameraControllerDelegateCallbacks, Color, Geometry, InteractionMode, Node, Prepareable, Scene,
    SceneRenderer, SpriteScene, SpriteTransition, Technique, Vector3, View,
};

mod common;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopDefaultMode: *const c_void;
    fn CFRunLoopRunInMode(
        mode: *const c_void,
        seconds: f64,
        return_after_source_handled: u8,
    ) -> i32;
}

fn camera_controller_inertia_reaches_the_delegate() {
    let camera = Camera::new().expect("camera");
    let camera_node = Node::new().expect("camera node");
    camera_node.set_camera(Some(&camera));
    camera_node.set_position(Vector3::new(0.0, 0.0, 5.0));
    let controller = CameraController::new().expect("camera controller");
    controller.set_point_of_view(Some(&camera_node));
    controller.set_interaction_mode(InteractionMode::OrbitTurntable);
    controller.set_inertia_enabled(true);
    let events = Arc::new(Mutex::new(Vec::new()));
    let delegate = CameraControllerDelegate::new(
        CameraControllerDelegateCallbacks::new()
            .on_inertia_will_start({
                let events = Arc::clone(&events);
                move || events.lock().expect("events").push("will-start")
            })
            .on_inertia_did_end({
                let events = Arc::clone(&events);
                move || events.lock().expect("events").push("did-end")
            }),
    )
    .expect("delegate");
    controller.set_delegate(Some(&delegate));

    let viewport = CGSize::new(100.0, 100.0);
    controller.begin_interaction(CGPoint::new(50.0, 50.0), viewport);
    controller.continue_interaction(CGPoint::new(60.0, 50.0), viewport, 1.0);
    controller.end_interaction(CGPoint::new(70.0, 50.0), viewport, CGPoint::new(500.0, 0.0));
    let deadline = Instant::now() + Duration::from_secs(5);
    while events.lock().expect("events").len() < 2 && Instant::now() < deadline {
        unsafe { CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.05, 0) };
    }
    assert_eq!(
        events.lock().expect("events").as_slice(),
        ["will-start", "did-end"]
    );
    controller.set_delegate(None);
}

fn view_snapshot_and_properties() {
    let (view, scene, camera_node) = common::view_with_camera(80.0, 60.0).expect("view setup");
    view.set_scene(Some(&scene));
    view.set_point_of_view(Some(&camera_node));
    view.set_background_color(Color::rgba(0.1, 0.2, 0.3, 1.0));
    view.set_allows_camera_control(true);
    view.set_renders_continuously(true);
    view.set_preferred_frames_per_second(30);
    let dimensions = view.snapshot_dimensions().expect("dimensions");
    assert!((dimensions.0 - 80.0).abs() < f64::EPSILON);
    assert!((dimensions.1 - 60.0).abs() < f64::EPSILON);
    assert_eq!(
        view.background_color(),
        Some(Color::rgba(0.1, 0.2, 0.3, 1.0))
    );
    assert!(view.allows_camera_control());
    assert!(view.renders_continuously());
    assert_eq!(view.preferred_frames_per_second(), 30);
    assert!(view.scene().is_some());
    assert!(view.point_of_view().is_some());
}

fn view_hit_test_returns_named_node() {
    let scene = Scene::new().expect("scene");
    let root = scene.root_node();
    let sphere = Geometry::sphere(1.0).expect("sphere");
    let sphere_node = Node::with_geometry(Some(&sphere)).expect("sphere node");
    sphere_node.set_name("target");
    root.add_child_node(&sphere_node);

    let camera = Camera::new().expect("camera");
    let camera_node = Node::new().expect("camera node");
    camera_node.set_camera(Some(&camera));
    camera_node.set_position(Vector3::new(0.0, 0.0, 5.0));
    root.add_child_node(&camera_node);

    let view = View::new(100.0, 100.0).expect("view");
    view.set_scene(Some(&scene));
    view.set_point_of_view(Some(&camera_node));
    let hits = view
        .hit_test(CGPoint::new(50.0, 50.0))
        .expect("hit results");
    assert!(!hits.is_empty());
    let first = hits.first().expect("first hit");
    assert_eq!(
        first.node().expect("node").name().as_deref(),
        Some("target")
    );
    assert!(first.world_coordinates().is_some());
}

fn technique_symbol_round_trip() {
    let view = View::new(32.0, 32.0).expect("view");
    let technique = Technique::minimal_draw_scene().expect("technique");
    technique.set_float_symbol("amplitude", 0.5);
    view.set_technique(Some(&technique));
    let attached = view.technique().expect("attached technique");
    assert_eq!(attached.dictionary_key_count(), 3);
    let amplitude = attached.float_symbol("amplitude").expect("amplitude");
    assert!((amplitude - 0.5).abs() < f64::EPSILON);
}

fn camera_control_configuration_round_trip() {
    let (view, _scene, camera_node) = common::view_with_camera(80.0, 60.0).expect("view setup");
    view.set_allows_camera_control(true);

    let configuration = view
        .camera_control_configuration()
        .expect("camera control configuration");
    configuration.set_auto_switch_to_free_camera(true);
    configuration.set_allows_translation(true);
    configuration.set_fly_mode_velocity(4.0);
    configuration.set_pan_sensitivity(0.5);
    configuration.set_truck_sensitivity(0.75);
    configuration.set_rotation_sensitivity(1.25);

    assert!(configuration.auto_switch_to_free_camera());
    assert!(configuration.allows_translation());
    assert!((configuration.fly_mode_velocity() - 4.0).abs() < f64::EPSILON);
    assert!((configuration.pan_sensitivity() - 0.5).abs() < f64::EPSILON);
    assert!((configuration.truck_sensitivity() - 0.75).abs() < f64::EPSILON);
    assert!((configuration.rotation_sensitivity() - 1.25).abs() < f64::EPSILON);

    let controller = view.default_camera_controller().expect("camera controller");
    controller.set_point_of_view(Some(&camera_node));
    controller.set_interaction_mode(InteractionMode::OrbitTurntable);
    assert_eq!(
        controller.interaction_mode(),
        Some(InteractionMode::OrbitTurntable)
    );
}

fn antialiasing_mode_round_trip() {
    let view = View::new(80.0, 60.0).expect("view");
    view.set_antialiasing_mode(AntialiasingMode::Multisampling2X);
    assert_eq!(
        view.antialiasing_mode(),
        Some(AntialiasingMode::Multisampling2X)
    );
}

fn extended_scene_renderer_surface() {
    let (scene, root, camera_node) = common::scene_with_camera().expect("scene setup");
    let cube = Geometry::box_geometry(1.0, 1.0, 1.0, 0.0).expect("cube geometry");
    let cube_node = Node::with_geometry(Some(&cube)).expect("cube node");
    root.add_child_node(&cube_node);
    let view = View::new(80.0, 60.0).expect("view");
    SceneRenderer::set_scene(&view, Some(&scene));
    SceneRenderer::set_point_of_view(&view, Some(&camera_node));

    let material = cube.first_material().expect("material");
    assert!(SceneRenderer::prepare_object(&view, &scene));
    assert!(SceneRenderer::prepare_object(&view, &cube));
    assert!(SceneRenderer::prepare_object(&view, &material));
    assert!(SceneRenderer::prepare_objects(
        &view,
        &[
            &scene as &dyn Prepareable,
            &cube_node as &dyn Prepareable,
            &cube as &dyn Prepareable,
            &material as &dyn Prepareable,
        ],
    ));

    let projected =
        SceneRenderer::project_point(&view, Vector3::new(0.0, 0.0, 0.0)).expect("project");
    let unprojected = SceneRenderer::unproject_point(&view, projected).expect("unproject");
    assert!(unprojected.z.is_finite());

    assert!(projected.z.is_finite());
    let hits =
        SceneRenderer::hit_test(&view, scenekit::CGPoint::new(40.0, 30.0)).expect("hit test");
    assert!(hits.count() >= 1, "the cube covers the view center");
    assert!(SceneRenderer::is_node_inside_frustum(
        &view,
        &cube_node,
        &camera_node
    ));
    assert!(!SceneRenderer::nodes_inside_frustum(&view, &camera_node).is_empty());

    SceneRenderer::set_audio_listener(&view, Some(&camera_node));
    let _listener = SceneRenderer::audio_listener(&view);

    let overlay = SpriteScene::new(32.0, 32.0).expect("overlay scene");
    SceneRenderer::set_overlay_scene(&view, Some(&overlay));
    assert!(SceneRenderer::overlay_scene(&view).is_some());

    let replacement = Scene::new().expect("replacement scene");
    let transition = SpriteTransition::fade(0.0).expect("transition");
    SceneRenderer::present_scene(&view, &replacement, Some(&transition), None);
    assert!(SceneRenderer::scene(&view).is_some());
}

fn main() {
    assert!(
        unsafe { libc::pthread_main_np() } != 0,
        "this harness must run on the main thread"
    );
    let tests: [(&str, fn()); 7] = [
        ("view_snapshot_and_properties", view_snapshot_and_properties),
        (
            "view_hit_test_returns_named_node",
            view_hit_test_returns_named_node,
        ),
        ("technique_symbol_round_trip", technique_symbol_round_trip),
        (
            "camera_control_configuration_round_trip",
            camera_control_configuration_round_trip,
        ),
        ("antialiasing_mode_round_trip", antialiasing_mode_round_trip),
        (
            "extended_scene_renderer_surface",
            extended_scene_renderer_surface,
        ),
        (
            "camera_controller_inertia_reaches_the_delegate",
            camera_controller_inertia_reaches_the_delegate,
        ),
    ];
    for (name, test) in tests {
        common::autoreleasepool(test);
        println!("test {name} ... ok");
    }
    println!("main_thread: {} tests passed", tests.len());
}
