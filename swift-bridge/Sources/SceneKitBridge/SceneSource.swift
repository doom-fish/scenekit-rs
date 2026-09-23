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

func scnSceneSourceOptions(
    _ keys: UnsafePointer<Int32>?,
    _ values: UnsafePointer<Double>?,
    _ count: Int,
    _ directories: UnsafePointer<UnsafePointer<CChar>?>?,
    _ directoryCount: Int
) -> [SCNSceneSource.LoadingOption: Any] {
    var options: [SCNSceneSource.LoadingOption: Any] = [:]
    if let keys, let values, count > 0 {
        for index in 0..<count {
            let value = values[index]
            switch keys[index] {
            case 0: options[.checkConsistency] = NSNumber(value: value != 0)
            case 1: options[.convertToYUp] = NSNumber(value: value != 0)
            case 2: options[.convertUnitsToMeters] = NSNumber(value: value)
            case 3: options[.createNormalsIfAbsent] = NSNumber(value: value != 0)
            case 4: options[.flattenScene] = NSNumber(value: value != 0)
            case 5: options[.overrideAssetURLs] = NSNumber(value: value != 0)
            case 6: options[.preserveOriginalTopology] = NSNumber(value: value != 0)
            case 7: options[.strictConformance] = NSNumber(value: value != 0)
            case 8:
                let policies: [SCNSceneSource.AnimationImportPolicy] = [.play, .playRepeatedly, .doNotPlay, .playUsingSceneTimeBase]
                if let position = Int(exactly: value), policies.indices.contains(position) {
                    options[.animationImportPolicy] = policies[position].rawValue
                }
            default:
                continue
            }
        }
    }
    if let directories, directoryCount > 0 {
        options[.assetDirectoryURLs] = (0..<directoryCount).compactMap { index -> URL? in
            guard let path = directories[index] else { return nil }
            return URL(fileURLWithPath: String(cString: path), isDirectory: true)
        }
    }
    return options
}

@_cdecl("scn_scene_source_new_url")
public func scn_scene_source_new_url(
    _ path: UnsafePointer<CChar>?,
    _ keys: UnsafePointer<Int32>?,
    _ values: UnsafePointer<Double>?,
    _ count: Int,
    _ directories: UnsafePointer<UnsafePointer<CChar>?>?,
    _ directoryCount: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    let options = scnSceneSourceOptions(keys, values, count, directories, directoryCount)
    guard let sceneSource = SCNSceneSource(url: URL(fileURLWithPath: String(cString: path)), options: options) else {
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
    _ keys: UnsafePointer<Int32>?,
    _ values: UnsafePointer<Double>?,
    _ count: Int,
    _ directories: UnsafePointer<UnsafePointer<CChar>?>?,
    _ directoryCount: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let bytes, length >= 0 else { return nil }
    let options = scnSceneSourceOptions(keys, values, count, directories, directoryCount)
    guard let sceneSource = SCNSceneSource(data: Data(bytes: bytes, count: length), options: options) else {
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
    _ keys: UnsafePointer<Int32>?,
    _ values: UnsafePointer<Double>?,
    _ count: Int,
    _ directories: UnsafePointer<UnsafePointer<CChar>?>?,
    _ directoryCount: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let sceneSource: SCNSceneSource = scnBorrow(sceneSourceHandle) else { return nil }
    do {
        let scene = try sceneSource.scene(options: scnSceneSourceOptions(keys, values, count, directories, directoryCount))
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
