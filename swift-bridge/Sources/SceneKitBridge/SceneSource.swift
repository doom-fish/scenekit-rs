import AppKit
import Foundation
import QuartzCore
import SceneKit

private func scnSceneSourceEntryClass(_ rawValue: Int32) -> AnyClass? {
    switch rawValue {
    case 0: return SCNMaterial.self
    case 1: return SCNGeometry.self
    case 2: return SCNScene.self
    case 3: return SCNNode.self
    case 4: return CAAnimation.self
    case 5: return SCNLight.self
    case 6: return SCNCamera.self
    case 7: return SCNSkinner.self
    case 8: return SCNMorpher.self
    case 9: return NSImage.self
    default: return nil
    }
}

@_cdecl("scn_scene_source_new_url")
public func scn_scene_source_new_url(
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    guard let sceneSource = SCNSceneSource(url: URL(fileURLWithPath: String(cString: path)), options: nil) else {
        outError?.pointee = scnDup("SCNSceneSource(url:options:) returned nil")
        return nil
    }
    outError?.pointee = nil
    return scnRetain(sceneSource)
}

@_cdecl("scn_scene_source_new_data")
public func scn_scene_source_new_data(
    _ bytes: UnsafeRawPointer?,
    _ length: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let bytes else { return nil }
    guard let sceneSource = SCNSceneSource(data: Data(bytes: bytes, count: length), options: nil) else {
        outError?.pointee = scnDup("SCNSceneSource(data:options:) returned nil")
        return nil
    }
    outError?.pointee = nil
    return scnRetain(sceneSource)
}

@_cdecl("scn_scene_source_copy_url")
public func scn_scene_source_copy_url(_ sceneSourceHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let sceneSource: SCNSceneSource = scnBorrow(sceneSourceHandle) else { return nil }
    return scnDup(sceneSource.url?.absoluteString)
}

@_cdecl("scn_scene_source_new_scene")
public func scn_scene_source_new_scene(
    _ sceneSourceHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let sceneSource: SCNSceneSource = scnBorrow(sceneSourceHandle) else { return nil }
    do {
        let scene = try sceneSource.scene(options: nil)
        outError?.pointee = nil
        return scnRetain(scene)
    } catch {
        outError?.pointee = scnDup(error.localizedDescription)
        return nil
    }
}

@_cdecl("scn_scene_source_copy_property_for_key")
public func scn_scene_source_copy_property_for_key(
    _ sceneSourceHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let sceneSource: SCNSceneSource = scnBorrow(sceneSourceHandle), let key else { return nil }
    let value = sceneSource.property(forKey: String(cString: key))
    if let string = value as? String {
        return scnDup(string)
    }
    return scnDup(value.map { String(describing: $0) })
}

@_cdecl("scn_scene_source_copy_identifiers_of_entries")
public func scn_scene_source_copy_identifiers_of_entries(
    _ sceneSourceHandle: UnsafeMutableRawPointer?,
    _ entryClass: Int32
) -> UnsafeMutablePointer<CChar>? {
    guard let sceneSource: SCNSceneSource = scnBorrow(sceneSourceHandle),
          let entryClass = scnSceneSourceEntryClass(entryClass)
    else { return nil }
    let identifiers = sceneSource.identifiersOfEntries(withClass: entryClass)
    return scnDup(identifiers.joined(separator: "\n"))
}
