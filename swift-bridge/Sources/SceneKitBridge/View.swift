import AppKit
import SceneKit

@_cdecl("scn_view_new")
public func scn_view_new(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNView(frame: NSRect(x: 0, y: 0, width: width, height: height), options: nil))
}

@_cdecl("scn_view_set_scene")
public func scn_view_set_scene(_ viewHandle: UnsafeMutableRawPointer?, _ sceneHandle: UnsafeMutableRawPointer?) {
    guard let view: SCNView = scnBorrow(viewHandle) else { return }
    let scene: SCNScene? = scnBorrow(sceneHandle)
    view.scene = scene
}

@_cdecl("scn_view_scene")
public func scn_view_scene(_ viewHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: SCNView = scnBorrow(viewHandle), let scene = view.scene else { return nil }
    return scnRetain(scene)
}

@_cdecl("scn_view_set_point_of_view")
public func scn_view_set_point_of_view(_ viewHandle: UnsafeMutableRawPointer?, _ nodeHandle: UnsafeMutableRawPointer?) {
    guard let view: SCNView = scnBorrow(viewHandle) else { return }
    let node: SCNNode? = scnBorrow(nodeHandle)
    view.pointOfView = node
}

@_cdecl("scn_view_point_of_view")
public func scn_view_point_of_view(_ viewHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: SCNView = scnBorrow(viewHandle), let node = view.pointOfView else { return nil }
    return scnRetain(node)
}

@_cdecl("scn_view_get_allows_camera_control")
public func scn_view_get_allows_camera_control(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SCNView = scnBorrow(viewHandle) else { return false }
    return view.allowsCameraControl
}

@_cdecl("scn_view_set_allows_camera_control")
public func scn_view_set_allows_camera_control(_ viewHandle: UnsafeMutableRawPointer?, _ allowsCameraControl: Bool) {
    guard let view: SCNView = scnBorrow(viewHandle) else { return }
    view.allowsCameraControl = allowsCameraControl
}

@_cdecl("scn_view_get_renders_continuously")
public func scn_view_get_renders_continuously(_ viewHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let view: SCNView = scnBorrow(viewHandle) else { return false }
    return view.rendersContinuously
}

@_cdecl("scn_view_set_renders_continuously")
public func scn_view_set_renders_continuously(_ viewHandle: UnsafeMutableRawPointer?, _ rendersContinuously: Bool) {
    guard let view: SCNView = scnBorrow(viewHandle) else { return }
    view.rendersContinuously = rendersContinuously
}

@_cdecl("scn_view_copy_background_color")
public func scn_view_copy_background_color(
    _ viewHandle: UnsafeMutableRawPointer?,
    _ outRGBA: UnsafeMutableRawPointer?
) -> Bool {
    guard let view: SCNView = scnBorrow(viewHandle) else { return false }
    return scnWriteColor(view.backgroundColor, outRGBA: outRGBA)
}

@_cdecl("scn_view_set_background_color")
public func scn_view_set_background_color(
    _ viewHandle: UnsafeMutableRawPointer?,
    _ r: Float,
    _ g: Float,
    _ b: Float,
    _ a: Float
) {
    guard let view: SCNView = scnBorrow(viewHandle) else { return }
    view.backgroundColor = scnMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("scn_view_snapshot_size")
public func scn_view_snapshot_size(_ viewHandle: UnsafeMutableRawPointer?, _ outSize: UnsafeMutablePointer<Double>?) -> Bool {
    guard let view: SCNView = scnBorrow(viewHandle), let outSize else { return false }
    let image = view.snapshot()
    outSize[0] = image.size.width
    outSize[1] = image.size.height
    return true
}

@_cdecl("scn_view_get_preferred_frames_per_second")
public func scn_view_get_preferred_frames_per_second(_ viewHandle: UnsafeMutableRawPointer?) -> Int {
    guard let view: SCNView = scnBorrow(viewHandle) else { return 0 }
    return view.preferredFramesPerSecond
}

@_cdecl("scn_view_set_preferred_frames_per_second")
public func scn_view_set_preferred_frames_per_second(_ viewHandle: UnsafeMutableRawPointer?, _ preferredFramesPerSecond: Int) {
    guard let view: SCNView = scnBorrow(viewHandle) else { return }
    view.preferredFramesPerSecond = preferredFramesPerSecond
}
