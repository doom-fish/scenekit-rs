use std::sync::{Arc, Mutex};

use scenekit::{
    CGPoint, CGSize, CameraController, CameraControllerDelegate, CameraControllerDelegateCallbacks,
    InteractionMode, Vector3,
};

mod common;

#[test]
fn test_camera_controller_properties_and_delegate_round_trip() {
    let (scene, _root, camera_node) = common::scene_with_camera().expect("scene setup");
    let controller = CameraController::new().expect("camera controller");
    controller.set_point_of_view(Some(&camera_node));
    controller.set_interaction_mode(InteractionMode::Pan);
    controller.set_target(Vector3::new(0.0, 0.0, 0.0));
    controller.set_automatic_target(true);
    assert!(controller.automatic_target());
    controller.set_automatic_target(false);
    controller.set_world_up(Vector3::new(0.0, 1.0, 0.0));
    controller.set_inertia_enabled(true);
    controller.set_inertia_friction(0.15);
    controller.set_minimum_vertical_angle(-30.0);
    controller.set_maximum_vertical_angle(30.0);
    controller.set_minimum_horizontal_angle(-90.0);
    controller.set_maximum_horizontal_angle(90.0);

    controller.translate_in_camera_space(Vector3::new(0.0, 0.0, -1.0));
    controller.frame_nodes(&[scene.root_node()]);
    controller.rotate_by(10.0, -5.0);
    controller.roll_by(3.0, CGPoint::new(10.0, 10.0), CGSize::new(80.0, 60.0));
    controller.dolly_by(1.0, CGPoint::new(20.0, 20.0), CGSize::new(80.0, 60.0));
    controller.roll_around_target(1.5);
    controller.dolly_to_target(0.5);
    controller.clear_roll();
    controller.begin_interaction(CGPoint::new(10.0, 10.0), CGSize::new(80.0, 60.0));
    controller.continue_interaction(CGPoint::new(15.0, 15.0), CGSize::new(80.0, 60.0), 0.8);
    controller.end_interaction(
        CGPoint::new(20.0, 20.0),
        CGSize::new(80.0, 60.0),
        CGPoint::new(3.0, 2.0),
    );
    controller.stop_inertia();

    assert_eq!(controller.interaction_mode(), Some(InteractionMode::Pan));
    assert_eq!(controller.target(), Vector3::new(0.0, 0.0, 0.0));
    assert!(!controller.automatic_target());
    assert_eq!(controller.world_up(), Vector3::new(0.0, 1.0, 0.0));
    assert!(controller.inertia_enabled());
    assert!((controller.inertia_friction() - 0.15).abs() < f32::EPSILON);
    assert!(controller.minimum_vertical_angle() <= controller.maximum_vertical_angle());
    assert!(controller.minimum_horizontal_angle() <= controller.maximum_horizontal_angle());
    assert!(controller
        .point_of_view()
        .and_then(|node| node.camera())
        .is_some());

    let delegate_events = Arc::new(Mutex::new(Vec::new()));
    let delegate = CameraControllerDelegate::new(
        CameraControllerDelegateCallbacks::new()
            .on_inertia_will_start({
                let delegate_events = Arc::clone(&delegate_events);
                move || delegate_events.lock().expect("events").push("will-start")
            })
            .on_inertia_did_end({
                let delegate_events = Arc::clone(&delegate_events);
                move || delegate_events.lock().expect("events").push("did-end")
            }),
    )
    .expect("camera controller delegate");
    controller.set_delegate(Some(&delegate));

    unsafe {
        common::scn_camera_controller_test_invoke_delegate_inertia_will_start(controller.as_ptr());
        common::scn_camera_controller_test_invoke_delegate_inertia_did_end(controller.as_ptr());
    }

    assert_eq!(
        delegate_events.lock().expect("events").as_slice(),
        ["will-start", "did-end"]
    );
}
