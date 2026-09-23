use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use apple_metal::MetalDevice;
use scenekit::{
    AvoidOccluderConstraint, AvoidOccluderConstraintDelegate,
    AvoidOccluderConstraintDelegateCallbacks, Geometry, Node, NodeRendererDelegate,
    NodeRendererDelegateCallbacks, PhysicsBallSocketJoint, PhysicsBody, PhysicsField,
    PhysicsFieldScope, PhysicsHingeJoint, PhysicsShape, PhysicsSliderJoint, PhysicsVehicle,
    PhysicsVehicleWheel, Renderer, Scene, SceneExportDelegate, SceneRenderer, Vector3,
};

mod common;

fn scene_with_cube() -> (Scene, Node, Node, Geometry, Node) {
    let (scene, root, camera_node) = common::scene_with_camera().expect("scene setup");
    let cube = Geometry::box_geometry(1.0, 1.0, 1.0, 0.0).expect("cube geometry");
    let cube_node = Node::with_geometry(Some(&cube)).expect("cube node");
    root.add_child_node(&cube_node);
    (scene, root, camera_node, cube, cube_node)
}

#[test]
fn test_node_renderer_delegate_bridge() {
    let device = MetalDevice::system_default().expect("device");
    let renderer = Renderer::new(Some(&device)).expect("renderer");
    assert!(SceneRenderer::device(&renderer).is_some());
    let _ = SceneRenderer::command_queue(&renderer);
    let _ = SceneRenderer::color_pixel_format(&renderer);
    let _ = SceneRenderer::depth_pixel_format(&renderer);
    let _ = SceneRenderer::stencil_pixel_format(&renderer);
    let _ = SceneRenderer::current_render_command_encoder(&renderer);
    let _ = SceneRenderer::current_render_pass_descriptor(&renderer);
    let _ = SceneRenderer::current_viewport(&renderer);
    let reverse_z = SceneRenderer::uses_reverse_z(&renderer);
    SceneRenderer::set_uses_reverse_z(&renderer, reverse_z);
    let node = Node::new().expect("node");
    let calls = Arc::new(AtomicUsize::new(0));

    let delegate = NodeRendererDelegate::new(NodeRendererDelegateCallbacks::new().on_render({
        let calls = Arc::clone(&calls);
        move |_, _| {
            calls.fetch_add(1, Ordering::SeqCst);
        }
    }))
    .expect("node renderer delegate");

    node.set_renderer_delegate(Some(&delegate));
    assert!(node.renderer_delegate().is_some());
    unsafe { common::scn_node_test_invoke_renderer_delegate(node.as_ptr(), renderer.as_ptr()) };
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn test_avoid_occluder_delegate_bridge() {
    let target = Node::new().expect("target");
    let occluder = Node::new().expect("occluder");
    let subject = Node::new().expect("subject");
    let constraint = AvoidOccluderConstraint::new(Some(&target)).expect("constraint");
    let events = Arc::new(Mutex::new(Vec::new()));

    let delegate = AvoidOccluderConstraintDelegate::new(
        AvoidOccluderConstraintDelegateCallbacks::new()
            .on_should_avoid_occluder({
                let events = Arc::clone(&events);
                move |_, _| {
                    events.lock().expect("events").push("should");
                    false
                }
            })
            .on_did_avoid_occluder({
                let events = Arc::clone(&events);
                move |_, _| events.lock().expect("events").push("did")
            }),
    )
    .expect("avoid delegate");

    constraint.set_delegate(Some(&delegate));
    assert!(constraint.delegate().is_some());
    unsafe {
        assert!(!common::scn_avoid_occluder_constraint_test_invoke_should(
            constraint.as_ptr(),
            occluder.as_ptr(),
            subject.as_ptr(),
        ));
        common::scn_avoid_occluder_constraint_test_invoke_did(
            constraint.as_ptr(),
            occluder.as_ptr(),
            subject.as_ptr(),
        );
    }

    let events = events.lock().expect("events");
    assert_eq!(events.as_slice(), ["should", "did"]);
}

#[test]
fn test_scene_export_and_extended_physics_surface() {
    let (scene, root, _camera_node, cube, cube_node) = scene_with_cube();

    let export_dir = PathBuf::from("target/test-output");
    fs::create_dir_all(&export_dir).expect("create export dir");
    let export_path = export_dir.join(format!("scene-export-{}.scn", std::process::id()));
    if export_path.exists() {
        fs::remove_file(&export_path).expect("remove stale export");
    }

    let export_delegate = SceneExportDelegate::new(|_image, document_url, _original_image_url| {
        Some(document_url.to_owned())
    })
    .expect("scene export delegate");
    scene
        .write_to_url(&export_path, Some(&export_delegate))
        .expect("export scene");
    assert!(export_path.exists());
    fs::remove_file(&export_path).expect("cleanup export");

    let body = PhysicsBody::dynamic_body().expect("physics body");
    let shape = PhysicsShape::with_geometry(&cube).expect("physics shape");
    body.set_physics_shape(Some(&shape));
    assert!(body.physics_shape().is_some());
    cube_node.set_physics_body(Some(&body));

    let field = PhysicsField::linear_gravity().expect("physics field");
    field.set_scope(PhysicsFieldScope::OutsideExtent);
    assert_eq!(field.scope(), PhysicsFieldScope::OutsideExtent);
    let field_node = Node::new().expect("field node");
    field_node.set_physics_field(Some(&field));
    root.add_child_node(&field_node);

    let wheel_node = Node::new().expect("wheel node");
    let wheel = PhysicsVehicleWheel::with_node(&wheel_node).expect("wheel");
    let vehicle = PhysicsVehicle::new(&body, &[&wheel]).expect("vehicle");
    let world = scene.physics_world();
    world.add_behavior(&vehicle);
    vehicle.apply_engine_force(5.0, 0);
    vehicle.set_steering_angle(0.25, 0);
    vehicle.apply_braking_force(1.0, 0);
    let _ = vehicle.speed_in_kilometers_per_hour();
    world.remove_behavior(&vehicle);

    let joint_anchor = Vector3::new(0.0, 0.0, 0.0);
    assert!(PhysicsBallSocketJoint::with_anchor(&body, joint_anchor).is_some());
    assert!(
        PhysicsHingeJoint::with_anchor(&body, Vector3::new(0.0, 1.0, 0.0), joint_anchor).is_some()
    );
    assert!(
        PhysicsSliderJoint::with_anchor(&body, Vector3::new(1.0, 0.0, 0.0), joint_anchor).is_some()
    );
}
