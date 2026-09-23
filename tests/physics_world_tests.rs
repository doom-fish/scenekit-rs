use std::sync::{Arc, Mutex};

use scenekit::{
    physics_world, PhysicsBody, PhysicsContactDelegate, PhysicsContactDelegateCallbacks, Scene,
    Vector3,
};

mod common;

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

    let delegate_events = Arc::new(Mutex::new(Vec::new()));
    let delegate = PhysicsContactDelegate::new(
        PhysicsContactDelegateCallbacks::new()
            .on_did_begin_contact({
                let delegate_events = Arc::clone(&delegate_events);
                move |_| delegate_events.lock().expect("events").push("did-begin")
            })
            .on_did_update_contact({
                let delegate_events = Arc::clone(&delegate_events);
                move |_| delegate_events.lock().expect("events").push("did-update")
            })
            .on_did_end_contact({
                let delegate_events = Arc::clone(&delegate_events);
                move |_| delegate_events.lock().expect("events").push("did-end")
            }),
    )
    .expect("physics contact delegate");
    world.set_contact_delegate(Some(&delegate));

    let invoke = || unsafe {
        common::scn_physics_world_test_invoke_delegate_did_begin(world.as_ptr());
        common::scn_physics_world_test_invoke_delegate_did_update(world.as_ptr());
        common::scn_physics_world_test_invoke_delegate_did_end(world.as_ptr());
    };
    invoke();
    assert_eq!(
        delegate_events.lock().expect("events").as_slice(),
        ["did-begin", "did-update", "did-end"]
    );

    drop(delegate);
    common::autoreleasepool(invoke);
    assert_eq!(delegate_events.lock().expect("events").len(), 3);
}
