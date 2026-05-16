import Foundation
import SceneKit

@_cdecl("scn_scene_new")
public func scn_scene_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNScene())
}

@_cdecl("scn_scene_new_named")
public func scn_scene_new_named(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else { return nil }
    guard let scene = SCNScene(named: String(cString: name)) else { return nil }
    return scnRetain(scene)
}

@_cdecl("scn_scene_new_url")
public func scn_scene_new_url(
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    do {
        let scene = try SCNScene(url: URL(fileURLWithPath: String(cString: path)), options: nil)
        return scnRetain(scene)
    } catch {
        outError?.pointee = scnDup(error.localizedDescription)
        return nil
    }
}

@_cdecl("scn_scene_root_node")
public func scn_scene_root_node(_ sceneHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let scene: SCNScene = scnBorrow(sceneHandle) else { return nil }
    return scnRetain(scene.rootNode)
}

@_cdecl("scn_scene_background")
public func scn_scene_background(_ sceneHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let scene: SCNScene = scnBorrow(sceneHandle) else { return nil }
    return scnRetain(scene.background)
}

@_cdecl("scn_scene_lighting_environment")
public func scn_scene_lighting_environment(_ sceneHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let scene: SCNScene = scnBorrow(sceneHandle) else { return nil }
    return scnRetain(scene.lightingEnvironment)
}

@_cdecl("scn_scene_set_fog_color")
public func scn_scene_set_fog_color(
    _ sceneHandle: UnsafeMutableRawPointer?,
    _ r: Float,
    _ g: Float,
    _ b: Float,
    _ a: Float
) {
    guard let scene: SCNScene = scnBorrow(sceneHandle) else { return }
    scene.fogColor = scnMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("scn_scene_copy_fog_color")
public func scn_scene_copy_fog_color(
    _ sceneHandle: UnsafeMutableRawPointer?,
    _ outRGBA: UnsafeMutableRawPointer?
) -> Bool {
    guard let scene: SCNScene = scnBorrow(sceneHandle) else { return false }
    return scnWriteColor(scene.fogColor, outRGBA: outRGBA)
}
