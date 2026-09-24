use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use apple_metal::{pixel_format, MetalDevice};
use scenekit::{
    Action, AvoidOccluderConstraint, AvoidOccluderConstraintDelegate,
    AvoidOccluderConstraintDelegateCallbacks, CameraController, CameraControllerDelegate,
    CameraControllerDelegateCallbacks, Geometry, Node, NodeRendererDelegate,
    NodeRendererDelegateCallbacks, Program, ProgramDelegate, Renderer, SceneRenderer,
    SceneRendererDelegate, SceneRendererDelegateCallbacks, Transaction,
};

mod common;

fn counting_renderer_delegate(calls: &Arc<AtomicUsize>) -> NodeRendererDelegate {
    let calls = Arc::clone(calls);
    NodeRendererDelegate::new(NodeRendererDelegateCallbacks::new().on_render(move |_, _| {
        calls.fetch_add(1, Ordering::SeqCst);
    }))
    .expect("node renderer delegate")
}

#[test]
fn node_keeps_its_renderer_delegate_until_the_node_is_freed() {
    let device = MetalDevice::system_default().expect("device");
    let texture = common::render_target(&device, 16, pixel_format::BGRA8UNORM).expect("texture");
    let calls = Arc::new(AtomicUsize::new(0));

    common::autoreleasepool(|| {
        let (scene, cube_node, camera_node) = common::green_cube_scene().expect("scene");
        let renderer = Renderer::new(Some(&device)).expect("renderer");
        renderer.set_scene(Some(&scene));
        renderer.set_point_of_view(Some(&camera_node));
        let delegate = counting_renderer_delegate(&calls);
        cube_node.set_renderer_delegate(Some(&delegate));
        Transaction::flush();
        common::render_frame(&device, &renderer, &texture, 0.0).expect("render");
        let calls_seen = calls.load(Ordering::SeqCst);
        assert!(calls_seen >= 1, "a real render calls the delegate");

        drop(delegate);
        assert_eq!(
            Arc::strong_count(&calls),
            2,
            "the node still owns the callbacks"
        );
        assert!(cube_node.renderer_delegate().is_some());
        common::render_frame(&device, &renderer, &texture, 0.1).expect("render");
        assert_eq!(
            calls.load(Ordering::SeqCst),
            calls_seen,
            "a delegate whose handle was dropped no longer runs"
        );

        cube_node.set_renderer_delegate(None);
        assert!(cube_node.renderer_delegate().is_none());
        assert_eq!(
            Arc::strong_count(&calls),
            2,
            "a cleared delegate stays alive while SceneKit may still be using it"
        );
        common::render_frame(&device, &renderer, &texture, 0.2).expect("render");
        assert_eq!(calls.load(Ordering::SeqCst), calls_seen);
        Transaction::flush();
    });
    assert_eq!(
        Arc::strong_count(&calls),
        1,
        "freeing the node frees the callbacks"
    );
}

#[test]
fn replacing_a_renderer_delegate_keeps_the_previous_one_until_the_node_is_freed() {
    let first_calls = Arc::new(AtomicUsize::new(0));
    let second_calls = Arc::new(AtomicUsize::new(0));

    common::autoreleasepool(|| {
        let node = Node::new().expect("node");
        node.set_renderer_delegate(Some(&counting_renderer_delegate(&first_calls)));
        let second = counting_renderer_delegate(&second_calls);
        node.set_renderer_delegate(Some(&second));
        assert_eq!(Arc::strong_count(&first_calls), 2);
        assert_eq!(
            node.renderer_delegate().map(|delegate| delegate.as_ptr()),
            Some(second.as_ptr())
        );
        node.set_renderer_delegate(Some(&second));
        node.set_renderer_delegate(None);
        drop(second);
        assert_eq!(Arc::strong_count(&second_calls), 2);
        Transaction::flush();
    });
    assert_eq!(Arc::strong_count(&first_calls), 1);
    assert_eq!(Arc::strong_count(&second_calls), 1);
}

#[test]
fn only_the_current_renderer_delegate_runs() {
    let device = MetalDevice::system_default().expect("device");
    let texture = common::render_target(&device, 16, pixel_format::BGRA8UNORM).expect("texture");
    let (scene, cube_node, camera_node) = common::green_cube_scene().expect("scene");
    let renderer = Renderer::new(Some(&device)).expect("renderer");
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));
    let first_calls = Arc::new(AtomicUsize::new(0));
    let second_calls = Arc::new(AtomicUsize::new(0));
    let first = counting_renderer_delegate(&first_calls);
    let second = counting_renderer_delegate(&second_calls);

    cube_node.set_renderer_delegate(Some(&first));
    Transaction::flush();
    common::render_frame(&device, &renderer, &texture, 0.0).expect("render");
    assert!(first_calls.load(Ordering::SeqCst) >= 1);

    cube_node.set_renderer_delegate(Some(&second));
    Transaction::flush();
    let first_before = first_calls.load(Ordering::SeqCst);
    common::render_frame(&device, &renderer, &texture, 0.1).expect("render");
    assert_eq!(first_calls.load(Ordering::SeqCst), first_before);
    assert!(second_calls.load(Ordering::SeqCst) >= 1);
    cube_node.set_renderer_delegate(None);
}

#[test]
fn render_callbacks_do_not_retain_the_node_or_renderer() {
    let device = MetalDevice::system_default().expect("device");
    let texture = common::render_target(&device, 16, pixel_format::BGRA8UNORM).expect("texture");
    let (scene, cube_node, camera_node) = common::green_cube_scene().expect("scene");
    let renderer = Renderer::new(Some(&device)).expect("renderer");
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));
    let calls = Arc::new(AtomicUsize::new(0));
    let delegate = counting_renderer_delegate(&calls);
    cube_node.set_renderer_delegate(Some(&delegate));
    Transaction::flush();
    for frame in 0..2 {
        common::autoreleasepool(|| {
            common::render_frame(&device, &renderer, &texture, f64::from(frame) / 60.0)
                .expect("render");
        });
    }

    let node_before = common::retain_count(cube_node.as_ptr());
    let renderer_before = common::retain_count(renderer.as_ptr());
    let calls_before = calls.load(Ordering::SeqCst);
    for frame in 2..18 {
        common::autoreleasepool(|| {
            common::render_frame(&device, &renderer, &texture, f64::from(frame) / 60.0)
                .expect("render");
        });
    }
    assert!(calls.load(Ordering::SeqCst) >= calls_before + 16);
    assert_eq!(common::retain_count(cube_node.as_ptr()), node_before);
    assert_eq!(common::retain_count(renderer.as_ptr()), renderer_before);
    cube_node.set_renderer_delegate(None);
}

#[test]
fn offline_rendering_calls_the_node_renderer_delegate() {
    let device = MetalDevice::system_default().expect("device");
    let texture = common::render_target(&device, 32, pixel_format::BGRA8UNORM).expect("texture");
    let (scene, cube_node, camera_node) = common::green_cube_scene().expect("scene");
    let calls = Arc::new(AtomicUsize::new(0));
    let seen_name = Arc::new(Mutex::new(None));
    cube_node.set_name("custom");
    let delegate = NodeRendererDelegate::new(NodeRendererDelegateCallbacks::new().on_render({
        let calls = Arc::clone(&calls);
        let seen_name = Arc::clone(&seen_name);
        move |node, _renderer| {
            calls.fetch_add(1, Ordering::SeqCst);
            *seen_name.lock().expect("name lock") = node.name();
        }
    }))
    .expect("delegate");
    cube_node.set_renderer_delegate(Some(&delegate));
    drop(delegate);

    let renderer = Renderer::new(Some(&device)).expect("renderer");
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));
    common::render_frame(&device, &renderer, &texture, 0.0).expect("render");
    assert_eq!(
        calls.load(Ordering::SeqCst),
        0,
        "the dropped delegate stays inactive during a real render"
    );

    let delegate = NodeRendererDelegate::new(NodeRendererDelegateCallbacks::new().on_render({
        let calls = Arc::clone(&calls);
        let seen_name = Arc::clone(&seen_name);
        move |node, _renderer| {
            calls.fetch_add(1, Ordering::SeqCst);
            *seen_name.lock().expect("name lock") = node.name();
        }
    }))
    .expect("delegate");
    cube_node.set_renderer_delegate(Some(&delegate));
    common::render_frame(&device, &renderer, &texture, 0.1).expect("render");
    assert!(calls.load(Ordering::SeqCst) >= 1);
    assert_eq!(
        seen_name.lock().expect("name lock").as_deref(),
        Some("custom")
    );
    cube_node.set_renderer_delegate(None);
}

#[test]
fn clones_keep_a_copied_renderer_delegate_alive() {
    let device = MetalDevice::system_default().expect("device");
    let texture = common::render_target(&device, 16, pixel_format::BGRA8UNORM).expect("texture");
    let calls = Arc::new(AtomicUsize::new(0));

    common::autoreleasepool(|| {
        let (scene, _cube_node, camera_node) = common::green_cube_scene().expect("scene");
        let delegate = counting_renderer_delegate(&calls);
        let parent = Node::new().expect("parent");
        let sphere = Geometry::sphere(0.5).expect("sphere");
        let child = Node::with_geometry(Some(&sphere)).expect("child");
        child.set_name("child");
        child.set_renderer_delegate(Some(&delegate));
        parent.add_child_node(&child);
        let clone = parent.clone_node().expect("clone");
        let cloned_child = clone
            .child_node_with_name("child", false)
            .expect("cloned child");
        assert!(
            cloned_child.renderer_delegate().is_some(),
            "SceneKit copies the renderer delegate into clones"
        );
        scene.root_node().add_child_node(&clone);
        let renderer = Renderer::new(Some(&device)).expect("renderer");
        renderer.set_scene(Some(&scene));
        renderer.set_point_of_view(Some(&camera_node));
        common::render_frame(&device, &renderer, &texture, 0.0).expect("render");
        let calls_seen = calls.load(Ordering::SeqCst);
        assert!(
            calls_seen >= 1,
            "rendering the clone calls the copied delegate"
        );

        child.set_renderer_delegate(None);
        drop(delegate);
        drop(child);
        drop(parent);
        assert_eq!(
            Arc::strong_count(&calls),
            2,
            "the clone still owns the callbacks"
        );
        common::render_frame(&device, &renderer, &texture, 0.1).expect("render");
        assert_eq!(calls.load(Ordering::SeqCst), calls_seen);
        Transaction::flush();
    });
    assert_eq!(Arc::strong_count(&calls), 1);
}

#[test]
fn constraint_keeps_its_avoid_occluder_delegate_alive() {
    let should_calls = Arc::new(AtomicUsize::new(0));
    let did_calls = Arc::new(AtomicUsize::new(0));

    common::autoreleasepool(|| {
        let target = Node::new().expect("target");
        let occluder = Node::new().expect("occluder");
        let subject = Node::new().expect("subject");
        let constraint = AvoidOccluderConstraint::new(Some(&target)).expect("constraint");
        let should = |constraint: &AvoidOccluderConstraint| {
            common::send_with_three_objects(
                common::send_object(constraint.as_ptr(), c"delegate"),
                c"avoidOccluderConstraint:shouldAvoidOccluder:forNode:",
                constraint.as_ptr(),
                occluder.as_ptr(),
                subject.as_ptr(),
            )
        };
        let did = |constraint: &AvoidOccluderConstraint| {
            common::send_void_with_three_objects(
                common::send_object(constraint.as_ptr(), c"delegate"),
                c"avoidOccluderConstraint:didAvoidOccluder:forNode:",
                constraint.as_ptr(),
                occluder.as_ptr(),
                subject.as_ptr(),
            );
        };
        let delegate = AvoidOccluderConstraintDelegate::new(
            AvoidOccluderConstraintDelegateCallbacks::new()
                .on_should_avoid_occluder({
                    let should_calls = Arc::clone(&should_calls);
                    move |_, _| {
                        should_calls.fetch_add(1, Ordering::SeqCst);
                        false
                    }
                })
                .on_did_avoid_occluder({
                    let did_calls = Arc::clone(&did_calls);
                    move |_, _| {
                        did_calls.fetch_add(1, Ordering::SeqCst);
                    }
                }),
        )
        .expect("delegate");
        constraint.set_delegate(Some(&delegate));
        assert!(!should(&constraint));
        did(&constraint);
        assert_eq!(should_calls.load(Ordering::SeqCst), 1);
        assert_eq!(did_calls.load(Ordering::SeqCst), 1);

        drop(delegate);
        assert!(constraint.delegate().is_some());
        assert!(
            should(&constraint),
            "an inactive delegate answers with SceneKit's default"
        );
        assert_eq!(should_calls.load(Ordering::SeqCst), 1);

        constraint.set_delegate(None);
        assert!(constraint.delegate().is_none());
        assert_eq!(Arc::strong_count(&should_calls), 2);
        Transaction::flush();
    });
    assert_eq!(Arc::strong_count(&should_calls), 1);
    assert_eq!(Arc::strong_count(&did_calls), 1);
}

#[test]
fn camera_controller_keeps_its_delegate_alive() {
    let events = Arc::new(Mutex::new(Vec::new()));

    common::autoreleasepool(|| {
        let controller = CameraController::new().expect("camera controller");
        let invoke = || {
            let delegate = common::send_object(controller.as_ptr(), c"delegate");
            if !delegate.is_null() {
                common::send_with_object(
                    delegate,
                    c"cameraInertiaWillStartForController:",
                    controller.as_ptr(),
                );
                common::send_with_object(
                    delegate,
                    c"cameraInertiaDidEndForController:",
                    controller.as_ptr(),
                );
            }
        };
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
        invoke();
        assert_eq!(
            events.lock().expect("events").as_slice(),
            ["will-start", "did-end"]
        );

        drop(delegate);
        assert!(controller.delegate().is_some());
        invoke();
        assert_eq!(events.lock().expect("events").len(), 2);

        controller.set_delegate(None);
        assert!(controller.delegate().is_none());
        invoke();
        assert_eq!(Arc::strong_count(&events), 3);
    });
    assert_eq!(Arc::strong_count(&events), 1);
}

#[test]
fn program_keeps_its_delegate_alive() {
    let errors = Arc::new(Mutex::new(Vec::new()));

    common::autoreleasepool(|| {
        let program = Program::new().expect("program");
        let invoke = || {
            let delegate = common::send_object(program.as_ptr(), c"delegate");
            if !delegate.is_null() {
                common::send_with_two_objects(
                    delegate,
                    c"program:handleError:",
                    program.as_ptr(),
                    common::ns_error(c"scenekit-rs-tests"),
                );
            }
        };
        let delegate = ProgramDelegate::new({
            let errors = Arc::clone(&errors);
            move |error| errors.lock().expect("errors").push(error.to_string())
        })
        .expect("delegate");
        program.set_delegate(Some(&delegate));
        invoke();
        let seen = errors.lock().expect("errors").clone();
        assert_eq!(seen.len(), 1);
        assert!(seen[0].contains("scenekit-rs-tests"), "{seen:?}");

        drop(delegate);
        assert!(program.delegate().is_some());
        invoke();
        assert_eq!(errors.lock().expect("errors").len(), 1);

        program.set_delegate(None);
        assert!(program.delegate().is_none());
        assert_eq!(Arc::strong_count(&errors), 2);
    });
    assert_eq!(Arc::strong_count(&errors), 1);
}

#[test]
fn offline_rendering_calls_the_scene_renderer_delegate_on_any_thread() {
    let device = MetalDevice::system_default().expect("device");
    let texture = common::render_target(&device, 32, pixel_format::BGRA8UNORM).expect("texture");
    let (scene, _cube_node, camera_node) = common::green_cube_scene().expect("scene");
    let events = Arc::new(Mutex::new(Vec::new()));
    let renderer = Renderer::new(Some(&device)).expect("renderer");
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));

    let delegate = SceneRendererDelegate::new(
        SceneRendererDelegateCallbacks::new()
            .on_update({
                let events = Arc::clone(&events);
                move |_| events.lock().expect("events").push("update")
            })
            .on_will_render_scene({
                let events = Arc::clone(&events);
                move |scene, _| {
                    assert!(scene.root_node().child_nodes().len() >= 2);
                    events.lock().expect("events").push("will-render");
                }
            })
            .on_did_render_scene({
                let events = Arc::clone(&events);
                move |_, _| events.lock().expect("events").push("did-render")
            }),
    )
    .expect("delegate");
    SceneRenderer::set_delegate(&renderer, Some(&delegate));
    common::render_frame(&device, &renderer, &texture, 0.0).expect("render");
    let seen = events.lock().expect("events").clone();
    assert!(seen.contains(&"update"), "{seen:?}");
    assert!(seen.contains(&"will-render"), "{seen:?}");
    assert!(seen.contains(&"did-render"), "{seen:?}");

    drop(delegate);
    let before = events.lock().expect("events").len();
    common::render_frame(&device, &renderer, &texture, 0.1).expect("render");
    assert_eq!(events.lock().expect("events").len(), before);
}

#[test]
fn custom_actions_receive_a_borrowed_node() {
    let device = MetalDevice::system_default().expect("device");
    let texture = common::render_target(&device, 32, pixel_format::BGRA8UNORM).expect("texture");
    let (scene, _cube_node, camera_node) = common::green_cube_scene().expect("scene");
    let sphere = Geometry::sphere(0.25).expect("sphere");
    let actor = Node::with_geometry(Some(&sphere)).expect("actor");
    actor.set_name("actor");
    scene.root_node().add_child_node(&actor);

    let names = Arc::new(Mutex::new(Vec::new()));
    let action = Action::custom(10.0, {
        let names = Arc::clone(&names);
        move |node, elapsed| {
            assert!(elapsed >= 0.0);
            names.lock().expect("names").push(node.name());
        }
    })
    .expect("custom action");
    actor.run_action(&action);
    drop(action);

    let renderer = Renderer::new(Some(&device)).expect("renderer");
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));
    SceneRenderer::set_playing(&renderer, true);
    for frame in 0..4 {
        common::render_frame(&device, &renderer, &texture, f64::from(frame) * 0.1).expect("render");
    }
    let seen = names.lock().expect("names").clone();
    assert!(!seen.is_empty(), "SceneKit never ran the custom action");
    assert!(seen.iter().all(|name| name.as_deref() == Some("actor")));
    assert_eq!(
        Arc::strong_count(&names),
        2,
        "the running action owns its callback"
    );
}
