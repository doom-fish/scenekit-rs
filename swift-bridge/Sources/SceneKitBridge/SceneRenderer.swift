import AVFAudio
import SceneKit
import SpriteKit

public typealias SceneRendererTimeCallback = @convention(c) (UnsafeMutableRawPointer?, Double) -> Void
public typealias SceneRendererSceneCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, Double) -> Void

private func scnBorrowSceneRenderer(_ handle: UnsafeMutableRawPointer?) -> (NSObjectProtocol & SCNSceneRenderer)? {
    guard let handle else { return nil }
    let object = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue()
    return object as? (NSObjectProtocol & SCNSceneRenderer)
}

@_cdecl("scn_array_count")
public func scn_array_count(_ arrayHandle: UnsafeMutableRawPointer?) -> Int {
    guard let array: NSArray = scnBorrow(arrayHandle) else { return 0 }
    return array.count
}

@_cdecl("scn_array_get")
public func scn_array_get(_ arrayHandle: UnsafeMutableRawPointer?, _ index: Int) -> UnsafeMutableRawPointer? {
    guard let array: NSArray = scnBorrow(arrayHandle), index >= 0, index < array.count else { return nil }
    return scnRetain(array[index] as AnyObject)
}

private final class SceneRendererDelegateBox: NSObject, SCNSceneRendererDelegate {
    let context: UnsafeMutableRawPointer?
    let releaseContext: ScnReleaseContextCallback?
    let update: SceneRendererTimeCallback
    let didApplyAnimations: SceneRendererTimeCallback
    let didSimulatePhysics: SceneRendererTimeCallback
    let didApplyConstraints: SceneRendererTimeCallback
    let willRenderScene: SceneRendererSceneCallback
    let didRenderScene: SceneRendererSceneCallback

    init(
        context: UnsafeMutableRawPointer?,
        releaseContext: ScnReleaseContextCallback?,
        update: @escaping SceneRendererTimeCallback,
        didApplyAnimations: @escaping SceneRendererTimeCallback,
        didSimulatePhysics: @escaping SceneRendererTimeCallback,
        didApplyConstraints: @escaping SceneRendererTimeCallback,
        willRenderScene: @escaping SceneRendererSceneCallback,
        didRenderScene: @escaping SceneRendererSceneCallback
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.update = update
        self.didApplyAnimations = didApplyAnimations
        self.didSimulatePhysics = didSimulatePhysics
        self.didApplyConstraints = didApplyConstraints
        self.willRenderScene = willRenderScene
        self.didRenderScene = didRenderScene
    }

    deinit {
        releaseContext?(context)
    }

    func renderer(_ renderer: any SCNSceneRenderer, updateAtTime time: TimeInterval) {
        update(context, time)
    }

    func renderer(_ renderer: any SCNSceneRenderer, didApplyAnimationsAtTime time: TimeInterval) {
        didApplyAnimations(context, time)
    }

    func renderer(_ renderer: any SCNSceneRenderer, didSimulatePhysicsAtTime time: TimeInterval) {
        didSimulatePhysics(context, time)
    }

    func renderer(_ renderer: any SCNSceneRenderer, didApplyConstraintsAtTime time: TimeInterval) {
        didApplyConstraints(context, time)
    }

    func renderer(_ renderer: any SCNSceneRenderer, willRenderScene scene: SCNScene, atTime time: TimeInterval) {
        willRenderScene(context, Unmanaged.passUnretained(scene).toOpaque(), time)
    }

    func renderer(_ renderer: any SCNSceneRenderer, didRenderScene scene: SCNScene, atTime time: TimeInterval) {
        didRenderScene(context, Unmanaged.passUnretained(scene).toOpaque(), time)
    }
}

@_cdecl("scn_scene_renderer_delegate_new")
public func scn_scene_renderer_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: ScnReleaseContextCallback?,
    _ update: @escaping SceneRendererTimeCallback,
    _ didApplyAnimations: @escaping SceneRendererTimeCallback,
    _ didSimulatePhysics: @escaping SceneRendererTimeCallback,
    _ didApplyConstraints: @escaping SceneRendererTimeCallback,
    _ willRenderScene: @escaping SceneRendererSceneCallback,
    _ didRenderScene: @escaping SceneRendererSceneCallback
) -> UnsafeMutableRawPointer? {
    scnRetain(SceneRendererDelegateBox(
        context: context,
        releaseContext: releaseContext,
        update: update,
        didApplyAnimations: didApplyAnimations,
        didSimulatePhysics: didSimulatePhysics,
        didApplyConstraints: didApplyConstraints,
        willRenderScene: willRenderScene,
        didRenderScene: didRenderScene
    ))
}

@_cdecl("scn_scene_renderer_get_scene")
public func scn_scene_renderer_get_scene(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle),
          let scene = renderer.scene
    else { return nil }
    return scnRetain(scene)
}

@_cdecl("scn_scene_renderer_set_scene")
public func scn_scene_renderer_set_scene(_ rendererHandle: UnsafeMutableRawPointer?, _ sceneHandle: UnsafeMutableRawPointer?) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.scene = scnBorrow(sceneHandle)
}

@_cdecl("scn_scene_renderer_present_scene")
public func scn_scene_renderer_present_scene(
    _ rendererHandle: UnsafeMutableRawPointer?,
    _ sceneHandle: UnsafeMutableRawPointer?,
    _ transitionHandle: UnsafeMutableRawPointer?,
    _ pointOfViewHandle: UnsafeMutableRawPointer?
) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let scene: SCNScene = scnBorrow(sceneHandle) else { return }
    let transition: SKTransition? = scnBorrow(transitionHandle)
    let pointOfView: SCNNode? = scnBorrow(pointOfViewHandle)
    renderer.present(scene, with: transition ?? SKTransition.crossFade(withDuration: 0), incomingPointOfView: pointOfView, completionHandler: nil)
}

@_cdecl("scn_scene_renderer_get_scene_time")
public func scn_scene_renderer_get_scene_time(_ rendererHandle: UnsafeMutableRawPointer?) -> Double {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return 0 }
    return renderer.sceneTime
}

@_cdecl("scn_scene_renderer_set_scene_time")
public func scn_scene_renderer_set_scene_time(_ rendererHandle: UnsafeMutableRawPointer?, _ sceneTime: Double) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.sceneTime = sceneTime
}

@_cdecl("scn_scene_renderer_get_point_of_view")
public func scn_scene_renderer_get_point_of_view(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle),
          let pointOfView = renderer.pointOfView
    else { return nil }
    return scnRetain(pointOfView)
}

@_cdecl("scn_scene_renderer_set_point_of_view")
public func scn_scene_renderer_set_point_of_view(_ rendererHandle: UnsafeMutableRawPointer?, _ nodeHandle: UnsafeMutableRawPointer?) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.pointOfView = scnBorrow(nodeHandle)
}

@_cdecl("scn_scene_renderer_hit_test")
public func scn_scene_renderer_hit_test(_ rendererHandle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return nil }
    return scnRetain(renderer.hitTest(CGPoint(x: x, y: y), options: nil) as NSArray)
}

@_cdecl("scn_scene_renderer_is_node_inside_frustum")
public func scn_scene_renderer_is_node_inside_frustum(
    _ rendererHandle: UnsafeMutableRawPointer?,
    _ nodeHandle: UnsafeMutableRawPointer?,
    _ pointOfViewHandle: UnsafeMutableRawPointer?
) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let node: SCNNode = scnBorrow(nodeHandle), let pointOfView: SCNNode = scnBorrow(pointOfViewHandle) else { return false }
    return renderer.isNode(node, insideFrustumOf: pointOfView)
}

@_cdecl("scn_scene_renderer_nodes_inside_frustum")
public func scn_scene_renderer_nodes_inside_frustum(
    _ rendererHandle: UnsafeMutableRawPointer?,
    _ pointOfViewHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let pointOfView: SCNNode = scnBorrow(pointOfViewHandle) else { return nil }
    return scnRetain(renderer.nodesInsideFrustum(of: pointOfView) as NSArray)
}

@_cdecl("scn_scene_renderer_project_point")
public func scn_scene_renderer_project_point(
    _ rendererHandle: UnsafeMutableRawPointer?,
    _ pointHandle: UnsafeMutableRawPointer?,
    _ outPointHandle: UnsafeMutableRawPointer?
) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let point = scnReadVector3(pointHandle), let outPointHandle else { return false }
    let projected = renderer.projectPoint(point)
    return scnWriteVector3(projected, out: outPointHandle)
}

@_cdecl("scn_scene_renderer_unproject_point")
public func scn_scene_renderer_unproject_point(
    _ rendererHandle: UnsafeMutableRawPointer?,
    _ pointHandle: UnsafeMutableRawPointer?,
    _ outPointHandle: UnsafeMutableRawPointer?
) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let point = scnReadVector3(pointHandle), let outPointHandle else { return false }
    let unprojected = renderer.unprojectPoint(point)
    return scnWriteVector3(unprojected, out: outPointHandle)
}

@_cdecl("scn_scene_renderer_get_playing")
public func scn_scene_renderer_get_playing(_ rendererHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return false }
    return renderer.isPlaying
}

@_cdecl("scn_scene_renderer_set_playing")
public func scn_scene_renderer_set_playing(_ rendererHandle: UnsafeMutableRawPointer?, _ playing: Bool) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.isPlaying = playing
}

@_cdecl("scn_scene_renderer_get_loops")
public func scn_scene_renderer_get_loops(_ rendererHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return false }
    return renderer.loops
}

@_cdecl("scn_scene_renderer_set_loops")
public func scn_scene_renderer_set_loops(_ rendererHandle: UnsafeMutableRawPointer?, _ loops: Bool) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.loops = loops
}

@_cdecl("scn_scene_renderer_get_autoenables_default_lighting")
public func scn_scene_renderer_get_autoenables_default_lighting(_ rendererHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return false }
    return renderer.autoenablesDefaultLighting
}

@_cdecl("scn_scene_renderer_set_autoenables_default_lighting")
public func scn_scene_renderer_set_autoenables_default_lighting(_ rendererHandle: UnsafeMutableRawPointer?, _ autoenablesDefaultLighting: Bool) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.autoenablesDefaultLighting = autoenablesDefaultLighting
}

@_cdecl("scn_scene_renderer_get_jittering_enabled")
public func scn_scene_renderer_get_jittering_enabled(_ rendererHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return false }
    return renderer.isJitteringEnabled
}

@_cdecl("scn_scene_renderer_set_jittering_enabled")
public func scn_scene_renderer_set_jittering_enabled(_ rendererHandle: UnsafeMutableRawPointer?, _ jitteringEnabled: Bool) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.isJitteringEnabled = jitteringEnabled
}

@_cdecl("scn_scene_renderer_get_temporal_antialiasing_enabled")
public func scn_scene_renderer_get_temporal_antialiasing_enabled(_ rendererHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return false }
    return renderer.isTemporalAntialiasingEnabled
}

@_cdecl("scn_scene_renderer_set_temporal_antialiasing_enabled")
public func scn_scene_renderer_set_temporal_antialiasing_enabled(_ rendererHandle: UnsafeMutableRawPointer?, _ temporalAntialiasingEnabled: Bool) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.isTemporalAntialiasingEnabled = temporalAntialiasingEnabled
}

@_cdecl("scn_scene_renderer_get_shows_statistics")
public func scn_scene_renderer_get_shows_statistics(_ rendererHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return false }
    return renderer.showsStatistics
}

@_cdecl("scn_scene_renderer_set_shows_statistics")
public func scn_scene_renderer_set_shows_statistics(_ rendererHandle: UnsafeMutableRawPointer?, _ showsStatistics: Bool) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.showsStatistics = showsStatistics
}

@_cdecl("scn_scene_renderer_get_debug_options")
public func scn_scene_renderer_get_debug_options(_ rendererHandle: UnsafeMutableRawPointer?) -> UInt {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return 0 }
    return renderer.debugOptions.rawValue
}

@_cdecl("scn_scene_renderer_set_debug_options")
public func scn_scene_renderer_set_debug_options(_ rendererHandle: UnsafeMutableRawPointer?, _ debugOptions: UInt) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.debugOptions = SCNDebugOptions(rawValue: debugOptions)
}

@_cdecl("scn_scene_renderer_get_rendering_api")
public func scn_scene_renderer_get_rendering_api(_ rendererHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return -1 }
    return Int32(renderer.renderingAPI.rawValue)
}

@_cdecl("scn_scene_renderer_get_delegate")
public func scn_scene_renderer_get_delegate(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let delegate = renderer.delegate else { return nil }
    return scnRetain(delegate)
}

@_cdecl("scn_scene_renderer_prepare_object")
public func scn_scene_renderer_prepare_object(_ rendererHandle: UnsafeMutableRawPointer?, _ objectHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let object = scnBorrow(objectHandle) as AnyObject? else { return false }
    return renderer.prepare(object, shouldAbortBlock: nil)
}

@_cdecl("scn_scene_renderer_prepare_objects")
public func scn_scene_renderer_prepare_objects(
    _ rendererHandle: UnsafeMutableRawPointer?,
    _ objectsHandle: UnsafeMutableRawPointer?,
    _ count: Int
) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return false }
    let ptr = objectsHandle?.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    let objects = (0..<count).compactMap { index -> AnyObject? in
        guard let ptr else { return nil }
        return scnBorrow(ptr[index]) as AnyObject?
    }
    return objects.allSatisfy { renderer.prepare($0, shouldAbortBlock: nil) }
}

@_cdecl("scn_scene_renderer_get_overlay_sk_scene")
public func scn_scene_renderer_get_overlay_sk_scene(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let scene = renderer.overlaySKScene else { return nil }
    return scnRetain(scene)
}

@_cdecl("scn_scene_renderer_set_overlay_sk_scene")
public func scn_scene_renderer_set_overlay_sk_scene(_ rendererHandle: UnsafeMutableRawPointer?, _ sceneHandle: UnsafeMutableRawPointer?) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    let scene: SKScene? = scnBorrow(sceneHandle)
    renderer.overlaySKScene = scene
}

@_cdecl("scn_scene_renderer_get_working_color_space")
public func scn_scene_renderer_get_working_color_space(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return nil }
    if #available(macOS 14.0, *) {
        return Unmanaged.passRetained(renderer.workingColorSpace).toOpaque()
    }
    return nil
}

@_cdecl("scn_scene_renderer_get_current_render_command_encoder")
public func scn_scene_renderer_get_current_render_command_encoder(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let encoder = renderer.currentRenderCommandEncoder else { return nil }
    return scnRetain(encoder)
}

@_cdecl("scn_scene_renderer_get_current_render_pass_descriptor")
public func scn_scene_renderer_get_current_render_pass_descriptor(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return nil }
    guard let descriptor = (renderer as AnyObject).value(forKey: "currentRenderPassDescriptor") as? MTLRenderPassDescriptor else { return nil }
    return scnRetain(descriptor)
}

@_cdecl("scn_scene_renderer_get_device")
public func scn_scene_renderer_get_device(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let device = renderer.device else { return nil }
    return scnRetain(device)
}

@_cdecl("scn_scene_renderer_get_color_pixel_format")
public func scn_scene_renderer_get_color_pixel_format(_ rendererHandle: UnsafeMutableRawPointer?) -> Int {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return 0 }
    return Int(renderer.colorPixelFormat.rawValue)
}

@_cdecl("scn_scene_renderer_get_depth_pixel_format")
public func scn_scene_renderer_get_depth_pixel_format(_ rendererHandle: UnsafeMutableRawPointer?) -> Int {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return 0 }
    return Int(renderer.depthPixelFormat.rawValue)
}

@_cdecl("scn_scene_renderer_get_stencil_pixel_format")
public func scn_scene_renderer_get_stencil_pixel_format(_ rendererHandle: UnsafeMutableRawPointer?) -> Int {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return 0 }
    return Int(renderer.stencilPixelFormat.rawValue)
}

@_cdecl("scn_scene_renderer_get_command_queue")
public func scn_scene_renderer_get_command_queue(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let queue = renderer.commandQueue else { return nil }
    return scnRetain(queue)
}

@_cdecl("scn_scene_renderer_get_audio_engine")
public func scn_scene_renderer_get_audio_engine(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return nil }
    return scnRetain(renderer.audioEngine)
}

@_cdecl("scn_scene_renderer_get_audio_environment_node")
public func scn_scene_renderer_get_audio_environment_node(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return nil }
    return scnRetain(renderer.audioEnvironmentNode)
}

@_cdecl("scn_scene_renderer_get_audio_listener")
public func scn_scene_renderer_get_audio_listener(_ rendererHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let listener = renderer.audioListener else { return nil }
    return scnRetain(listener)
}

@_cdecl("scn_scene_renderer_set_audio_listener")
public func scn_scene_renderer_set_audio_listener(_ rendererHandle: UnsafeMutableRawPointer?, _ listenerHandle: UnsafeMutableRawPointer?) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    let listener: SCNNode? = scnBorrow(listenerHandle)
    renderer.audioListener = listener
}

@_cdecl("scn_scene_renderer_get_current_viewport")
public func scn_scene_renderer_get_current_viewport(_ rendererHandle: UnsafeMutableRawPointer?, _ outRect: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle), let outRect else { return false }
    outRect.assumingMemoryBound(to: CGRect.self).pointee = renderer.currentViewport
    return true
}

@_cdecl("scn_scene_renderer_get_current_time")
public func scn_scene_renderer_get_current_time(_ rendererHandle: UnsafeMutableRawPointer?) -> Double {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return 0 }
    return renderer.currentTime
}

@_cdecl("scn_scene_renderer_set_current_time")
public func scn_scene_renderer_set_current_time(_ rendererHandle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.currentTime = value
}

@_cdecl("scn_scene_renderer_get_uses_reverse_z")
public func scn_scene_renderer_get_uses_reverse_z(_ rendererHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return false }
    return renderer.usesReverseZ
}

@_cdecl("scn_scene_renderer_set_uses_reverse_z")
public func scn_scene_renderer_set_uses_reverse_z(_ rendererHandle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.usesReverseZ = value
}

@_cdecl("scn_scene_renderer_set_delegate")
public func scn_scene_renderer_set_delegate(_ rendererHandle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle) else { return }
    renderer.delegate = scnBorrow(delegateHandle)
}

@_cdecl("scn_scene_renderer_test_invoke_delegate_update")
public func scn_scene_renderer_test_invoke_delegate_update(_ rendererHandle: UnsafeMutableRawPointer?, _ time: Double) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle),
          let delegate = renderer.delegate else { return }
    delegate.renderer?(renderer, updateAtTime: time)
}

@_cdecl("scn_scene_renderer_test_invoke_delegate_will_render_scene")
public func scn_scene_renderer_test_invoke_delegate_will_render_scene(_ rendererHandle: UnsafeMutableRawPointer?, _ time: Double) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle),
          let delegate = renderer.delegate,
          let scene = renderer.scene else { return }
    delegate.renderer?(renderer, willRenderScene: scene, atTime: time)
}

@_cdecl("scn_scene_renderer_test_invoke_delegate_did_render_scene")
public func scn_scene_renderer_test_invoke_delegate_did_render_scene(_ rendererHandle: UnsafeMutableRawPointer?, _ time: Double) {
    guard let renderer = scnBorrowSceneRenderer(rendererHandle),
          let delegate = renderer.delegate,
          let scene = renderer.scene else { return }
    delegate.renderer?(renderer, didRenderScene: scene, atTime: time)
}

@_cdecl("scn_view_get_antialiasing_mode")
public func scn_view_get_antialiasing_mode(_ viewHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let view: SCNView = scnBorrow(viewHandle) else { return -1 }
    return Int32(view.antialiasingMode.rawValue)
}

@_cdecl("scn_view_set_antialiasing_mode")
public func scn_view_set_antialiasing_mode(_ viewHandle: UnsafeMutableRawPointer?, _ antialiasingMode: Int32) {
    guard let view: SCNView = scnBorrow(viewHandle) else { return }
    view.antialiasingMode = SCNAntialiasingMode(rawValue: UInt(antialiasingMode)) ?? .none
}
