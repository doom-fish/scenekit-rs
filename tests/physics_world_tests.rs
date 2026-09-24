use std::sync::{Arc, Mutex};

use apple_metal::{pixel_format, MetalDevice};
use scenekit::{
    physics_world, Camera, Geometry, Node, PhysicsBody, PhysicsContact, PhysicsContactDelegate,
    PhysicsContactDelegateCallbacks, Renderer, Scene, SceneRenderer, Vector3,
};

fn contact_names(contact: Option<&PhysicsContact>) -> Vec<Option<String>> {
    let mut names: Vec<Option<String>> = contact
        .map(|contact| {
            vec![
                contact.node_a().and_then(|node| node.name()),
                contact.node_b().and_then(|node| node.name()),
            ]
        })
        .unwrap_or_default();
    names.sort();
    names
}

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
}

#[test]
fn contact_delegate_receives_real_contacts_until_it_is_dropped() {
    let scene = Scene::new().expect("scene");
    let world = scene.physics_world();
    let delegate_events = Arc::new(Mutex::new(Vec::new()));
    let delegate = PhysicsContactDelegate::new(
        PhysicsContactDelegateCallbacks::new()
            .on_did_begin_contact({
                let delegate_events = Arc::clone(&delegate_events);
                move |contact| {
                    delegate_events
                        .lock()
                        .expect("events")
                        .push(("did-begin", contact_names(contact)));
                }
            })
            .on_did_update_contact({
                let delegate_events = Arc::clone(&delegate_events);
                move |contact| {
                    delegate_events
                        .lock()
                        .expect("events")
                        .push(("did-update", contact_names(contact)));
                }
            })
            .on_did_end_contact({
                let delegate_events = Arc::clone(&delegate_events);
                move |contact| {
                    delegate_events
                        .lock()
                        .expect("events")
                        .push(("did-end", contact_names(contact)));
                }
            }),
    )
    .expect("physics contact delegate");
    world.set_contact_delegate(Some(&delegate));
    world.set_gravity(Vector3::new(0.0, -9.8, 0.0));
    world.set_speed(1.0);
    world.set_time_step(1.0 / 60.0);

    let floor_box = Geometry::box_geometry(10.0, 0.5, 10.0, 0.0).expect("floor box");
    let floor = Node::with_geometry(Some(&floor_box)).expect("floor");
    floor.set_name("floor");
    let floor_body = PhysicsBody::static_body().expect("floor body");
    floor_body.set_category_bit_mask(1);
    floor_body.set_contact_test_bit_mask(2);
    assert_eq!(floor_body.category_bit_mask(), 1);
    assert_eq!(floor_body.contact_test_bit_mask(), 2);
    floor_body.set_collision_bit_mask(usize::MAX);
    assert_eq!(floor_body.collision_bit_mask(), usize::MAX);
    floor.set_physics_body(Some(&floor_body));
    scene.root_node().add_child_node(&floor);

    let sphere = Geometry::sphere(0.5).expect("sphere");
    let ball = Node::with_geometry(Some(&sphere)).expect("ball");
    ball.set_name("ball");
    ball.set_position(Vector3::new(0.0, 1.5, 0.0));
    let ball_body = PhysicsBody::dynamic_body().expect("ball body");
    ball_body.set_category_bit_mask(2);
    ball_body.set_contact_test_bit_mask(1);
    ball_body.set_restitution(1.0);
    ball.set_physics_body(Some(&ball_body));
    scene.root_node().add_child_node(&ball);

    let camera = Camera::new().expect("camera");
    let camera_node = Node::new().expect("camera node");
    camera_node.set_camera(Some(&camera));
    camera_node.set_position(Vector3::new(0.0, 2.0, 10.0));
    scene.root_node().add_child_node(&camera_node);
    let device = MetalDevice::system_default().expect("device");
    let texture = common::render_target(&device, 16, pixel_format::BGRA8UNORM).expect("texture");
    let renderer = Renderer::new(Some(&device)).expect("renderer");
    renderer.set_scene(Some(&scene));
    renderer.set_point_of_view(Some(&camera_node));
    SceneRenderer::set_playing(&renderer, true);
    for frame in 0..120 {
        common::autoreleasepool(|| {
            common::render_frame(&device, &renderer, &texture, f64::from(frame) / 60.0)
                .expect("render");
        });
    }
    let events = delegate_events.lock().expect("events").clone();
    let expected_names = vec![Some("ball".to_owned()), Some("floor".to_owned())];
    assert!(
        events
            .iter()
            .any(|(kind, names)| *kind == "did-begin" && *names == expected_names),
        "{events:?}"
    );

    drop(delegate);
    let seen = delegate_events.lock().expect("events").len();
    ball.set_position(Vector3::new(0.0, 1.5, 0.0));
    for frame in 120..180 {
        common::autoreleasepool(|| {
            common::render_frame(&device, &renderer, &texture, f64::from(frame) / 60.0)
                .expect("render");
        });
    }
    assert_eq!(delegate_events.lock().expect("events").len(), seen);
}
