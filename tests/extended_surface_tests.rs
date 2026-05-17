use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use apple_metal::MetalDevice;
use scenekit::{
    AvoidOccluderConstraint, AvoidOccluderConstraintDelegate,
    AvoidOccluderConstraintDelegateCallbacks, Geometry, Node, NodeRendererDelegate,
    NodeRendererDelegateCallbacks, PhysicsBallSocketJoint, PhysicsBody, PhysicsField,
    PhysicsFieldScope, PhysicsHingeJoint, PhysicsShape, PhysicsSliderJoint, PhysicsVehicle,
    PhysicsVehicleWheel, Prepareable, Renderer, Scene, SceneExportDelegate, SceneRenderer,
    SpriteScene, SpriteTransition, Vector3, View,
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
fn test_extended_scene_renderer_surface() {
    let (scene, _root, camera_node, cube, cube_node) = scene_with_cube();
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

    let hits = SceneRenderer::hit_test(
        &view,
        scenekit::CGPoint::new(projected.x.into(), projected.y.into()),
    )
    .expect("hit test");
    let _ = hits.count();
    assert!(SceneRenderer::is_node_inside_frustum(
        &view,
        &cube_node,
        &camera_node
    ));
    assert!(!SceneRenderer::nodes_inside_frustum(&view, &camera_node).is_empty());

    SceneRenderer::set_current_time(&view, 0.75);
    let _ = SceneRenderer::current_time(&view);

    SceneRenderer::set_audio_listener(&view, Some(&camera_node));
    let _ = SceneRenderer::audio_listener(&view);
    let _ = SceneRenderer::audio_engine(&view);
    let _ = SceneRenderer::audio_environment_node(&view);

    let overlay = SpriteScene::new(32.0, 32.0).expect("overlay scene");
    SceneRenderer::set_overlay_scene(&view, Some(&overlay));
    assert!(SceneRenderer::overlay_scene(&view).is_some());
    let _ = SceneRenderer::working_color_space(&view);

    let replacement = Scene::new().expect("replacement scene");
    let transition = SpriteTransition::fade(0.0).expect("transition");
    SceneRenderer::present_scene(&view, &replacement, Some(&transition), None);
    assert!(SceneRenderer::scene(&view).is_some());
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
    let calls = Rc::new(RefCell::new(0usize));

    let delegate = NodeRendererDelegate::new(NodeRendererDelegateCallbacks::new().on_render({
        let calls = Rc::clone(&calls);
        move |_, _| *calls.borrow_mut() += 1
    }))
    .expect("node renderer delegate");

    node.set_renderer_delegate(Some(&delegate));
    assert!(node.renderer_delegate().is_some());
    node.test_invoke_renderer_delegate(&renderer);
    assert_eq!(*calls.borrow(), 1);
}

#[test]
fn test_avoid_occluder_delegate_bridge() {
    let target = Node::new().expect("target");
    let occluder = Node::new().expect("occluder");
    let subject = Node::new().expect("subject");
    let constraint = AvoidOccluderConstraint::new(Some(&target)).expect("constraint");
    let events = Rc::new(RefCell::new(Vec::new()));

    let delegate = AvoidOccluderConstraintDelegate::new(
        AvoidOccluderConstraintDelegateCallbacks::new()
            .on_should_avoid_occluder({
                let events = Rc::clone(&events);
                move |_, _| {
                    events.borrow_mut().push("should");
                    false
                }
            })
            .on_did_avoid_occluder({
                let events = Rc::clone(&events);
                move |_, _| events.borrow_mut().push("did")
            }),
    )
    .expect("avoid delegate");

    constraint.set_delegate(Some(&delegate));
    assert!(constraint.delegate().is_some());
    assert!(!constraint.test_invoke_should_avoid_occluder(&occluder, &subject));
    constraint.test_invoke_did_avoid_occluder(&occluder, &subject);

    let events = events.borrow();
    assert!(events.contains(&"should"));
    assert!(events.contains(&"did"));
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

    let export_delegate =
        SceneExportDelegate::new(|document_url, _original_image_url| Some(document_url.to_owned()))
            .expect("scene export delegate");
    assert!(scene.write_to_url(&export_path, Some(&export_delegate)));
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
