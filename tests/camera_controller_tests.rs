use std::cell::RefCell;
use std::rc::Rc;

use scenekit::{
    ffi, CGPoint, CGSize, CameraControllerDelegate, CameraControllerDelegateCallbacks,
    InteractionMode, Vector3,
};

mod common;

#[test]
fn test_camera_controller_configuration_and_delegate_round_trip() {
    let (view, scene, camera_node) = common::view_with_camera(80.0, 60.0).expect("view setup");
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

    let delegate_events = Rc::new(RefCell::new(Vec::new()));
    let delegate = CameraControllerDelegate::new(
        CameraControllerDelegateCallbacks::new()
            .on_inertia_will_start({
                let delegate_events = Rc::clone(&delegate_events);
                move || delegate_events.borrow_mut().push("will-start")
            })
            .on_inertia_did_end({
                let delegate_events = Rc::clone(&delegate_events);
                move || delegate_events.borrow_mut().push("did-end")
            }),
    )
    .expect("camera controller delegate");
    controller.set_delegate(Some(&delegate));

    unsafe {
        ffi::scn_camera_controller_test_invoke_delegate_inertia_will_start(controller.as_ptr());
        ffi::scn_camera_controller_test_invoke_delegate_inertia_did_end(controller.as_ptr());
    }

    let delegate_events = delegate_events.borrow();
    assert!(delegate_events.contains(&"will-start"));
    assert!(delegate_events.contains(&"did-end"));
}
