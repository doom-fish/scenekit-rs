import Foundation
import SceneKit

func scnMinimalTechniqueDictionary() -> [String: Any] {
    [
        "sequence": ["pass_scene"],
        "passes": [
            "pass_scene": [
                "draw": "DRAW_SCENE"
            ]
        ],
        "symbols": [
            "amplitude": [
                "type": "float"
            ]
        ]
    ]
}

@_cdecl("scn_technique_new_minimal_draw_scene")
public func scn_technique_new_minimal_draw_scene() -> UnsafeMutableRawPointer? {
    guard let technique = SCNTechnique(dictionary: scnMinimalTechniqueDictionary()) else { return nil }
    return scnRetain(technique)
}

@_cdecl("scn_technique_dictionary_key_count")
public func scn_technique_dictionary_key_count(_ techniqueHandle: UnsafeMutableRawPointer?) -> Int {
    guard let technique: SCNTechnique = scnBorrow(techniqueHandle) else { return 0 }
    return technique.dictionaryRepresentation.count
}

@_cdecl("scn_technique_set_float_symbol")
public func scn_technique_set_float_symbol(
    _ techniqueHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ value: Double
) {
    guard let technique: SCNTechnique = scnBorrow(techniqueHandle), let key else { return }
    technique.setValue(NSNumber(value: value), forKey: String(cString: key))
}

@_cdecl("scn_technique_get_float_symbol")
public func scn_technique_get_float_symbol(
    _ techniqueHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<Double>?
) -> Bool {
    guard let technique: SCNTechnique = scnBorrow(techniqueHandle),
          let key,
          let outValue,
          let value = technique.value(forKey: String(cString: key)) as? NSNumber
    else { return false }
    outValue.pointee = value.doubleValue
    return true
}

@_cdecl("scn_view_set_technique")
public func scn_view_set_technique(_ viewHandle: UnsafeMutableRawPointer?, _ techniqueHandle: UnsafeMutableRawPointer?) {
    guard let view: SCNView = scnBorrow(viewHandle) else { return }
    let technique: SCNTechnique? = scnBorrow(techniqueHandle)
    view.technique = technique
}

@_cdecl("scn_view_technique")
public func scn_view_technique(_ viewHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let view: SCNView = scnBorrow(viewHandle), let technique = view.technique else { return nil }
    return scnRetain(technique)
}
