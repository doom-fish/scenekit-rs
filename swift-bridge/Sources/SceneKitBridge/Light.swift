import SceneKit

func scnLightType(from rawValue: Int32) -> SCNLight.LightType {
    switch rawValue {
    case 0: return .ambient
    case 1: return .directional
    case 2: return .omni
    case 3: return .spot
    case 4: return .area
    case 5: return .probe
    default: return .IES
    }
}

func scnLightTypeValue(_ lightType: SCNLight.LightType) -> Int32 {
    switch lightType {
    case .ambient: return 0
    case .directional: return 1
    case .omni: return 2
    case .spot: return 3
    case .area: return 4
    case .probe: return 5
    default: return 6
    }
}

@_cdecl("scn_light_new")
public func scn_light_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNLight())
}

@_cdecl("scn_light_get_type")
public func scn_light_get_type(_ lightHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let light: SCNLight = scnBorrow(lightHandle) else { return -1 }
    return scnLightTypeValue(light.type)
}

@_cdecl("scn_light_set_type")
public func scn_light_set_type(_ lightHandle: UnsafeMutableRawPointer?, _ type: Int32) {
    guard let light: SCNLight = scnBorrow(lightHandle) else { return }
    light.type = scnLightType(from: type)
}

@_cdecl("scn_light_copy_color")
public func scn_light_copy_color(
    _ lightHandle: UnsafeMutableRawPointer?,
    _ outRGBA: UnsafeMutableRawPointer?
) -> Bool {
    guard let light: SCNLight = scnBorrow(lightHandle) else { return false }
    return scnWriteColor(light.color, outRGBA: outRGBA)
}

@_cdecl("scn_light_set_color")
public func scn_light_set_color(
    _ lightHandle: UnsafeMutableRawPointer?,
    _ r: Float,
    _ g: Float,
    _ b: Float,
    _ a: Float
) {
    guard let light: SCNLight = scnBorrow(lightHandle) else { return }
    light.color = scnMakeColor(r: r, g: g, b: b, a: a)
}

@_cdecl("scn_light_get_intensity")
public func scn_light_get_intensity(_ lightHandle: UnsafeMutableRawPointer?) -> Double {
    guard let light: SCNLight = scnBorrow(lightHandle) else { return 0 }
    return light.intensity
}

@_cdecl("scn_light_set_intensity")
public func scn_light_set_intensity(_ lightHandle: UnsafeMutableRawPointer?, _ intensity: Double) {
    guard let light: SCNLight = scnBorrow(lightHandle) else { return }
    light.intensity = intensity
}

@_cdecl("scn_light_get_shadow_mode")
public func scn_light_get_shadow_mode(_ lightHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let light: SCNLight = scnBorrow(lightHandle) else { return -1 }
    return Int32(light.shadowMode.rawValue)
}

@_cdecl("scn_light_set_shadow_mode")
public func scn_light_set_shadow_mode(_ lightHandle: UnsafeMutableRawPointer?, _ shadowMode: Int32) {
    guard let light: SCNLight = scnBorrow(lightHandle) else { return }
    light.shadowMode = SCNShadowMode(rawValue: Int(shadowMode)) ?? .forward
}

@_cdecl("scn_light_get_casts_shadow")
public func scn_light_get_casts_shadow(_ lightHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let light: SCNLight = scnBorrow(lightHandle) else { return false }
    return light.castsShadow
}

@_cdecl("scn_light_set_casts_shadow")
public func scn_light_set_casts_shadow(_ lightHandle: UnsafeMutableRawPointer?, _ castsShadow: Bool) {
    guard let light: SCNLight = scnBorrow(lightHandle) else { return }
    light.castsShadow = castsShadow
}
