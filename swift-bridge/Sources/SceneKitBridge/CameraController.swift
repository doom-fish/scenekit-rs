import SceneKit

public typealias CameraControllerCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

private func scnBorrowCameraControlConfiguration(
    _ handle: UnsafeMutableRawPointer?
) -> (NSObjectProtocol & SCNCameraControlConfiguration)? {
    guard let handle else { return nil }
    let object = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue()
    return object as? (NSObjectProtocol & SCNCameraControlConfiguration)
}

private final class CameraControllerDelegateBox: NSObject, SCNCameraControllerDelegate {
    let context: UnsafeMutableRawPointer?
    let releaseContext: ScnReleaseContextCallback?
    let inertiaWillStart: CameraControllerCallback
    let inertiaDidEnd: CameraControllerCallback

    init(
        context: UnsafeMutableRawPointer?,
        releaseContext: ScnReleaseContextCallback?,
        inertiaWillStart: @escaping CameraControllerCallback,
        inertiaDidEnd: @escaping CameraControllerCallback
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.inertiaWillStart = inertiaWillStart
        self.inertiaDidEnd = inertiaDidEnd
    }

    deinit {
        releaseContext?(context)
    }

    func cameraInertiaWillStart(for cameraController: SCNCameraController) {
        inertiaWillStart(context)
    }

    func cameraInertiaDidEnd(for cameraController: SCNCameraController) {
        inertiaDidEnd(context)
    }

    func invokeInertiaWillStart() {
        inertiaWillStart(context)
    }

    func invokeInertiaDidEnd() {
        inertiaDidEnd(context)
    }
}

@_cdecl("scn_camera_controller_delegate_new")
public func scn_camera_controller_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: ScnReleaseContextCallback?,
    _ inertiaWillStart: @escaping CameraControllerCallback,
    _ inertiaDidEnd: @escaping CameraControllerCallback
) -> UnsafeMutableRawPointer? {
    scnRetain(CameraControllerDelegateBox(
        context: context,
        releaseContext: releaseContext,
        inertiaWillStart: inertiaWillStart,
        inertiaDidEnd: inertiaDidEnd
    ))
}

@_cdecl("scn_view_camera_control_configuration")
public func scn_view_camera_control_configuration(_ viewHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: SCNView = scnBorrow(viewHandle) else { return nil }
    return scnRetain(view.cameraControlConfiguration as AnyObject)
}

@_cdecl("scn_view_default_camera_controller")
public func scn_view_default_camera_controller(_ viewHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: SCNView = scnBorrow(viewHandle) else { return nil }
    return scnRetain(view.defaultCameraController)
}

@_cdecl("scn_camera_control_configuration_get_auto_switch_to_free_camera")
public func scn_camera_control_configuration_get_auto_switch_to_free_camera(_ configurationHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return false }
    return configuration.autoSwitchToFreeCamera
}

@_cdecl("scn_camera_control_configuration_set_auto_switch_to_free_camera")
public func scn_camera_control_configuration_set_auto_switch_to_free_camera(_ configurationHandle: UnsafeMutableRawPointer?, _ autoSwitchToFreeCamera: Bool) {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return }
    configuration.autoSwitchToFreeCamera = autoSwitchToFreeCamera
}

@_cdecl("scn_camera_control_configuration_get_allows_translation")
public func scn_camera_control_configuration_get_allows_translation(_ configurationHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return false }
    return configuration.allowsTranslation
}

@_cdecl("scn_camera_control_configuration_set_allows_translation")
public func scn_camera_control_configuration_set_allows_translation(_ configurationHandle: UnsafeMutableRawPointer?, _ allowsTranslation: Bool) {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return }
    configuration.allowsTranslation = allowsTranslation
}

@_cdecl("scn_camera_control_configuration_get_fly_mode_velocity")
public func scn_camera_control_configuration_get_fly_mode_velocity(_ configurationHandle: UnsafeMutableRawPointer?) -> Double {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return 0 }
    return configuration.flyModeVelocity
}

@_cdecl("scn_camera_control_configuration_set_fly_mode_velocity")
public func scn_camera_control_configuration_set_fly_mode_velocity(_ configurationHandle: UnsafeMutableRawPointer?, _ flyModeVelocity: Double) {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return }
    configuration.flyModeVelocity = flyModeVelocity
}

@_cdecl("scn_camera_control_configuration_get_pan_sensitivity")
public func scn_camera_control_configuration_get_pan_sensitivity(_ configurationHandle: UnsafeMutableRawPointer?) -> Double {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return 0 }
    return configuration.panSensitivity
}

@_cdecl("scn_camera_control_configuration_set_pan_sensitivity")
public func scn_camera_control_configuration_set_pan_sensitivity(_ configurationHandle: UnsafeMutableRawPointer?, _ panSensitivity: Double) {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return }
    configuration.panSensitivity = panSensitivity
}

@_cdecl("scn_camera_control_configuration_get_truck_sensitivity")
public func scn_camera_control_configuration_get_truck_sensitivity(_ configurationHandle: UnsafeMutableRawPointer?) -> Double {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return 0 }
    return configuration.truckSensitivity
}

@_cdecl("scn_camera_control_configuration_set_truck_sensitivity")
public func scn_camera_control_configuration_set_truck_sensitivity(_ configurationHandle: UnsafeMutableRawPointer?, _ truckSensitivity: Double) {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return }
    configuration.truckSensitivity = truckSensitivity
}

@_cdecl("scn_camera_control_configuration_get_rotation_sensitivity")
public func scn_camera_control_configuration_get_rotation_sensitivity(_ configurationHandle: UnsafeMutableRawPointer?) -> Double {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return 0 }
    return configuration.rotationSensitivity
}

@_cdecl("scn_camera_control_configuration_set_rotation_sensitivity")
public func scn_camera_control_configuration_set_rotation_sensitivity(_ configurationHandle: UnsafeMutableRawPointer?, _ rotationSensitivity: Double) {
    guard let configuration = scnBorrowCameraControlConfiguration(configurationHandle) else { return }
    configuration.rotationSensitivity = rotationSensitivity
}

@_cdecl("scn_camera_controller_set_delegate")
public func scn_camera_controller_set_delegate(_ controllerHandle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.delegate = scnBorrow(delegateHandle)
}

@_cdecl("scn_camera_controller_get_point_of_view")
public func scn_camera_controller_get_point_of_view(_ controllerHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle),
          let pointOfView = controller.pointOfView
    else { return nil }
    return scnRetain(pointOfView)
}

@_cdecl("scn_camera_controller_set_point_of_view")
public func scn_camera_controller_set_point_of_view(_ controllerHandle: UnsafeMutableRawPointer?, _ nodeHandle: UnsafeMutableRawPointer?) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.pointOfView = scnBorrow(nodeHandle)
}

@_cdecl("scn_camera_controller_get_interaction_mode")
public func scn_camera_controller_get_interaction_mode(_ controllerHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return -1 }
    return Int32(controller.interactionMode.rawValue)
}

@_cdecl("scn_camera_controller_set_interaction_mode")
public func scn_camera_controller_set_interaction_mode(_ controllerHandle: UnsafeMutableRawPointer?, _ interactionMode: Int32) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.interactionMode = SCNInteractionMode(rawValue: Int(interactionMode)) ?? .fly
}

@_cdecl("scn_camera_controller_get_target")
public func scn_camera_controller_get_target(_ controllerHandle: UnsafeMutableRawPointer?, _ outVector: UnsafeMutableRawPointer?) -> Bool {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return false }
    return scnWriteVector3(controller.target, out: outVector)
}

@_cdecl("scn_camera_controller_set_target")
public func scn_camera_controller_set_target(_ controllerHandle: UnsafeMutableRawPointer?, _ target: UnsafeMutableRawPointer?) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle),
          let target = scnReadVector3(target)
    else { return }
    controller.target = target
}

@_cdecl("scn_camera_controller_get_automatic_target")
public func scn_camera_controller_get_automatic_target(_ controllerHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return false }
    return controller.automaticTarget
}

@_cdecl("scn_camera_controller_set_automatic_target")
public func scn_camera_controller_set_automatic_target(_ controllerHandle: UnsafeMutableRawPointer?, _ automaticTarget: Bool) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.automaticTarget = automaticTarget
}

@_cdecl("scn_camera_controller_get_world_up")
public func scn_camera_controller_get_world_up(_ controllerHandle: UnsafeMutableRawPointer?, _ outVector: UnsafeMutableRawPointer?) -> Bool {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return false }
    return scnWriteVector3(controller.worldUp, out: outVector)
}

@_cdecl("scn_camera_controller_set_world_up")
public func scn_camera_controller_set_world_up(_ controllerHandle: UnsafeMutableRawPointer?, _ worldUp: UnsafeMutableRawPointer?) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle),
          let worldUp = scnReadVector3(worldUp)
    else { return }
    controller.worldUp = worldUp
}

@_cdecl("scn_camera_controller_get_inertia_enabled")
public func scn_camera_controller_get_inertia_enabled(_ controllerHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return false }
    return controller.inertiaEnabled
}

@_cdecl("scn_camera_controller_set_inertia_enabled")
public func scn_camera_controller_set_inertia_enabled(_ controllerHandle: UnsafeMutableRawPointer?, _ inertiaEnabled: Bool) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.inertiaEnabled = inertiaEnabled
}

@_cdecl("scn_camera_controller_get_inertia_friction")
public func scn_camera_controller_get_inertia_friction(_ controllerHandle: UnsafeMutableRawPointer?) -> Float {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return 0 }
    return controller.inertiaFriction
}

@_cdecl("scn_camera_controller_set_inertia_friction")
public func scn_camera_controller_set_inertia_friction(_ controllerHandle: UnsafeMutableRawPointer?, _ inertiaFriction: Float) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.inertiaFriction = inertiaFriction
}

@_cdecl("scn_camera_controller_get_inertia_running")
public func scn_camera_controller_get_inertia_running(_ controllerHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return false }
    return controller.isInertiaRunning
}

@_cdecl("scn_camera_controller_get_minimum_vertical_angle")
public func scn_camera_controller_get_minimum_vertical_angle(_ controllerHandle: UnsafeMutableRawPointer?) -> Float {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return 0 }
    return controller.minimumVerticalAngle
}

@_cdecl("scn_camera_controller_set_minimum_vertical_angle")
public func scn_camera_controller_set_minimum_vertical_angle(_ controllerHandle: UnsafeMutableRawPointer?, _ minimumVerticalAngle: Float) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.minimumVerticalAngle = minimumVerticalAngle
}

@_cdecl("scn_camera_controller_get_maximum_vertical_angle")
public func scn_camera_controller_get_maximum_vertical_angle(_ controllerHandle: UnsafeMutableRawPointer?) -> Float {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return 0 }
    return controller.maximumVerticalAngle
}

@_cdecl("scn_camera_controller_set_maximum_vertical_angle")
public func scn_camera_controller_set_maximum_vertical_angle(_ controllerHandle: UnsafeMutableRawPointer?, _ maximumVerticalAngle: Float) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.maximumVerticalAngle = maximumVerticalAngle
}

@_cdecl("scn_camera_controller_get_minimum_horizontal_angle")
public func scn_camera_controller_get_minimum_horizontal_angle(_ controllerHandle: UnsafeMutableRawPointer?) -> Float {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return 0 }
    return controller.minimumHorizontalAngle
}

@_cdecl("scn_camera_controller_set_minimum_horizontal_angle")
public func scn_camera_controller_set_minimum_horizontal_angle(_ controllerHandle: UnsafeMutableRawPointer?, _ minimumHorizontalAngle: Float) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.minimumHorizontalAngle = minimumHorizontalAngle
}

@_cdecl("scn_camera_controller_get_maximum_horizontal_angle")
public func scn_camera_controller_get_maximum_horizontal_angle(_ controllerHandle: UnsafeMutableRawPointer?) -> Float {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return 0 }
    return controller.maximumHorizontalAngle
}

@_cdecl("scn_camera_controller_set_maximum_horizontal_angle")
public func scn_camera_controller_set_maximum_horizontal_angle(_ controllerHandle: UnsafeMutableRawPointer?, _ maximumHorizontalAngle: Float) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.maximumHorizontalAngle = maximumHorizontalAngle
}

@_cdecl("scn_camera_controller_translate_in_camera_space")
public func scn_camera_controller_translate_in_camera_space(_ controllerHandle: UnsafeMutableRawPointer?, _ deltaX: Float, _ deltaY: Float, _ deltaZ: Float) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.translateInCameraSpaceBy(x: deltaX, y: deltaY, z: deltaZ)
}

@_cdecl("scn_camera_controller_frame_nodes")
public func scn_camera_controller_frame_nodes(_ controllerHandle: UnsafeMutableRawPointer?, _ nodes: UnsafeMutableRawPointer?, _ count: Int) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    guard count > 0, let nodes else {
        controller.frameNodes([])
        return
    }

    let handles = nodes.assumingMemoryBound(to: Optional<UnsafeMutableRawPointer>.self)
    let resolvedNodes = (0..<count).compactMap { index -> SCNNode? in
        guard let handle = handles[index] else { return nil }
        return scnBorrow(handle)
    }
    controller.frameNodes(resolvedNodes)
}

@_cdecl("scn_camera_controller_rotate_by")
public func scn_camera_controller_rotate_by(_ controllerHandle: UnsafeMutableRawPointer?, _ deltaX: Float, _ deltaY: Float) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.rotateBy(x: deltaX, y: deltaY)
}

@_cdecl("scn_camera_controller_roll_by")
public func scn_camera_controller_roll_by(_ controllerHandle: UnsafeMutableRawPointer?, _ delta: Float, _ pointX: Double, _ pointY: Double, _ viewportWidth: Double, _ viewportHeight: Double) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.roll(by: delta, aroundScreenPoint: CGPoint(x: pointX, y: pointY), viewport: CGSize(width: viewportWidth, height: viewportHeight))
}

@_cdecl("scn_camera_controller_dolly_by")
public func scn_camera_controller_dolly_by(_ controllerHandle: UnsafeMutableRawPointer?, _ delta: Float, _ pointX: Double, _ pointY: Double, _ viewportWidth: Double, _ viewportHeight: Double) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.dolly(by: delta, onScreenPoint: CGPoint(x: pointX, y: pointY), viewport: CGSize(width: viewportWidth, height: viewportHeight))
}

@_cdecl("scn_camera_controller_roll_around_target")
public func scn_camera_controller_roll_around_target(_ controllerHandle: UnsafeMutableRawPointer?, _ delta: Float) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.rollAroundTarget(delta)
}

@_cdecl("scn_camera_controller_dolly_to_target")
public func scn_camera_controller_dolly_to_target(_ controllerHandle: UnsafeMutableRawPointer?, _ delta: Float) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.dolly(toTarget: delta)
}

@_cdecl("scn_camera_controller_clear_roll")
public func scn_camera_controller_clear_roll(_ controllerHandle: UnsafeMutableRawPointer?) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.clearRoll()
}

@_cdecl("scn_camera_controller_stop_inertia")
public func scn_camera_controller_stop_inertia(_ controllerHandle: UnsafeMutableRawPointer?) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.stopInertia()
}

@_cdecl("scn_camera_controller_begin_interaction")
public func scn_camera_controller_begin_interaction(_ controllerHandle: UnsafeMutableRawPointer?, _ locationX: Double, _ locationY: Double, _ viewportWidth: Double, _ viewportHeight: Double) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.beginInteraction(CGPoint(x: locationX, y: locationY), withViewport: CGSize(width: viewportWidth, height: viewportHeight))
}

@_cdecl("scn_camera_controller_continue_interaction")
public func scn_camera_controller_continue_interaction(_ controllerHandle: UnsafeMutableRawPointer?, _ locationX: Double, _ locationY: Double, _ viewportWidth: Double, _ viewportHeight: Double, _ sensitivity: Double) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.continueInteraction(CGPoint(x: locationX, y: locationY), withViewport: CGSize(width: viewportWidth, height: viewportHeight), sensitivity: sensitivity)
}

@_cdecl("scn_camera_controller_end_interaction")
public func scn_camera_controller_end_interaction(_ controllerHandle: UnsafeMutableRawPointer?, _ locationX: Double, _ locationY: Double, _ viewportWidth: Double, _ viewportHeight: Double, _ velocityX: Double, _ velocityY: Double) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle) else { return }
    controller.endInteraction(CGPoint(x: locationX, y: locationY), withViewport: CGSize(width: viewportWidth, height: viewportHeight), velocity: CGPoint(x: velocityX, y: velocityY))
}

@_cdecl("scn_camera_controller_test_invoke_delegate_inertia_will_start")
public func scn_camera_controller_test_invoke_delegate_inertia_will_start(_ controllerHandle: UnsafeMutableRawPointer?) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle),
          let delegate = controller.delegate as? CameraControllerDelegateBox
    else { return }
    delegate.invokeInertiaWillStart()
}

@_cdecl("scn_camera_controller_test_invoke_delegate_inertia_did_end")
public func scn_camera_controller_test_invoke_delegate_inertia_did_end(_ controllerHandle: UnsafeMutableRawPointer?) {
    guard let controller: SCNCameraController = scnBorrow(controllerHandle),
          let delegate = controller.delegate as? CameraControllerDelegateBox
    else { return }
    delegate.invokeInertiaDidEnd()
}
