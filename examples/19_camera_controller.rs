use scenekit::{
    CGPoint, CGSize, CameraControllerDelegate, CameraControllerDelegateCallbacks, InteractionMode,
    Scene, Vector3, View,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = Scene::new().ok_or("missing scene")?;
    let view = View::new(80.0, 60.0).ok_or("missing view")?;
    view.set_scene(Some(&scene));
    view.set_allows_camera_control(true);

    let configuration = view
        .camera_control_configuration()
        .ok_or("missing camera control configuration")?;
    configuration.set_allows_translation(true);
    configuration.set_fly_mode_velocity(3.0);

    let controller = view
        .default_camera_controller()
        .ok_or("missing default camera controller")?;
    controller.set_interaction_mode(InteractionMode::OrbitTurntable);
    controller.set_target(Vector3::new(0.0, 0.0, 0.0));
    controller.begin_interaction(CGPoint::new(10.0, 10.0), CGSize::new(80.0, 60.0));
    controller.continue_interaction(CGPoint::new(20.0, 20.0), CGSize::new(80.0, 60.0), 1.0);
    controller.end_interaction(
        CGPoint::new(30.0, 30.0),
        CGSize::new(80.0, 60.0),
        CGPoint::new(1.0, 1.0),
    );

    let delegate = CameraControllerDelegate::new(
        CameraControllerDelegateCallbacks::new().on_inertia_will_start(|| ()),
    )
    .ok_or("missing camera controller delegate")?;
    controller.set_delegate(Some(&delegate));
    println!("✅ configured camera controller");
    Ok(())
}
