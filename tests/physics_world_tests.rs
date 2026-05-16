use std::cell::RefCell;
use std::rc::Rc;

use scenekit::{
    ffi, physics_world, PhysicsBody, PhysicsContactDelegate, PhysicsContactDelegateCallbacks,
    Scene, Vector3,
};

#[test]
fn test_physics_world_properties_and_delegate_round_trip() {
    let scene = Scene::new().expect("scene");
    let world = scene.physics_world();
    world.set_gravity(Vector3::new(0.0, -4.9, 0.0));
    world.set_speed(0.5);
    world.set_time_step(1.0 / 120.0);

    assert_eq!(world.gravity(), Vector3::new(0.0, -4.9, 0.0));
    assert!((world.speed() - 0.5).abs() < f64::EPSILON);
    assert!((world.time_step() - (1.0 / 120.0)).abs() < f64::EPSILON);
    assert!(!physics_world::physics_test_search_mode_any().is_empty());
    assert!(!physics_world::physics_test_collision_bit_mask_key().is_empty());

    let body_a = PhysicsBody::dynamic_body().expect("body a");
    let body_b = PhysicsBody::dynamic_body().expect("body b");
    assert_eq!(world.contact_test_with_body(&body_a), 0);
    assert_eq!(world.contact_test_between_bodies(&body_a, &body_b), 0);
    world.update_collision_pairs();

    let delegate_events = Rc::new(RefCell::new(Vec::new()));
    let delegate = PhysicsContactDelegate::new(
        PhysicsContactDelegateCallbacks::new()
            .on_did_begin_contact({
                let delegate_events = Rc::clone(&delegate_events);
                move |_| delegate_events.borrow_mut().push("did-begin")
            })
            .on_did_update_contact({
                let delegate_events = Rc::clone(&delegate_events);
                move |_| delegate_events.borrow_mut().push("did-update")
            })
            .on_did_end_contact({
                let delegate_events = Rc::clone(&delegate_events);
                move |_| delegate_events.borrow_mut().push("did-end")
            }),
    )
    .expect("physics contact delegate");
    world.set_contact_delegate(Some(&delegate));

    unsafe {
        ffi::scn_physics_world_test_invoke_delegate_did_begin(world.as_ptr());
        ffi::scn_physics_world_test_invoke_delegate_did_update(world.as_ptr());
        ffi::scn_physics_world_test_invoke_delegate_did_end(world.as_ptr());
    }

    let delegate_events = delegate_events.borrow();
    assert!(delegate_events.contains(&"did-begin"));
    assert!(delegate_events.contains(&"did-update"));
    assert!(delegate_events.contains(&"did-end"));
}
