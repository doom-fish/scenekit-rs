import QuartzCore
import SceneKit

@_cdecl("scn_animation_new_opacity")
public func scn_animation_new_opacity(
    _ from: Float,
    _ to: Float,
    _ duration: Double
) -> UnsafeMutableRawPointer? {
    let animation = CABasicAnimation(keyPath: "opacity")
    animation.fromValue = from
    animation.toValue = to
    animation.duration = duration
    return scnRetain(SCNAnimation(caAnimation: animation))
}

@_cdecl("scn_animation_get_duration")
public func scn_animation_get_duration(_ animationHandle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: SCNAnimation = scnBorrow(animationHandle) else { return 0 }
    return animation.duration
}

@_cdecl("scn_animation_set_duration")
public func scn_animation_set_duration(_ animationHandle: UnsafeMutableRawPointer?, _ duration: Double) {
    guard let animation: SCNAnimation = scnBorrow(animationHandle) else { return }
    animation.duration = duration
}

@_cdecl("scn_animation_get_repeat_count")
public func scn_animation_get_repeat_count(_ animationHandle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: SCNAnimation = scnBorrow(animationHandle) else { return 0 }
    return animation.repeatCount
}

@_cdecl("scn_animation_set_repeat_count")
public func scn_animation_set_repeat_count(_ animationHandle: UnsafeMutableRawPointer?, _ repeatCount: Double) {
    guard let animation: SCNAnimation = scnBorrow(animationHandle) else { return }
    animation.repeatCount = CGFloat(repeatCount)
}

@_cdecl("scn_animation_get_autoreverses")
public func scn_animation_get_autoreverses(_ animationHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let animation: SCNAnimation = scnBorrow(animationHandle) else { return false }
    return animation.autoreverses
}

@_cdecl("scn_animation_set_autoreverses")
public func scn_animation_set_autoreverses(_ animationHandle: UnsafeMutableRawPointer?, _ autoreverses: Bool) {
    guard let animation: SCNAnimation = scnBorrow(animationHandle) else { return }
    animation.autoreverses = autoreverses
}

@_cdecl("scn_animation_get_uses_scene_time_base")
public func scn_animation_get_uses_scene_time_base(_ animationHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let animation: SCNAnimation = scnBorrow(animationHandle) else { return false }
    return animation.usesSceneTimeBase
}

@_cdecl("scn_animation_set_uses_scene_time_base")
public func scn_animation_set_uses_scene_time_base(
    _ animationHandle: UnsafeMutableRawPointer?,
    _ usesSceneTimeBase: Bool
) {
    guard let animation: SCNAnimation = scnBorrow(animationHandle) else { return }
    animation.usesSceneTimeBase = usesSceneTimeBase
}

@_cdecl("scn_animation_player_new")
public func scn_animation_player_new(_ animationHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let animation: SCNAnimation = scnBorrow(animationHandle) else { return nil }
    return scnRetain(SCNAnimationPlayer(animation: animation))
}

@_cdecl("scn_animation_player_animation")
public func scn_animation_player_animation(_ playerHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let player: SCNAnimationPlayer = scnBorrow(playerHandle) else { return nil }
    return scnRetain(player.animation)
}

@_cdecl("scn_animation_player_get_speed")
public func scn_animation_player_get_speed(_ playerHandle: UnsafeMutableRawPointer?) -> Double {
    guard let player: SCNAnimationPlayer = scnBorrow(playerHandle) else { return 0 }
    return player.speed
}

@_cdecl("scn_animation_player_set_speed")
public func scn_animation_player_set_speed(_ playerHandle: UnsafeMutableRawPointer?, _ speed: Double) {
    guard let player: SCNAnimationPlayer = scnBorrow(playerHandle) else { return }
    player.speed = CGFloat(speed)
}

@_cdecl("scn_animation_player_get_paused")
public func scn_animation_player_get_paused(_ playerHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let player: SCNAnimationPlayer = scnBorrow(playerHandle) else { return false }
    return player.paused
}

@_cdecl("scn_animation_player_set_paused")
public func scn_animation_player_set_paused(_ playerHandle: UnsafeMutableRawPointer?, _ paused: Bool) {
    guard let player: SCNAnimationPlayer = scnBorrow(playerHandle) else { return }
    player.paused = paused
}

@_cdecl("scn_animation_player_play")
public func scn_animation_player_play(_ playerHandle: UnsafeMutableRawPointer?) {
    guard let player: SCNAnimationPlayer = scnBorrow(playerHandle) else { return }
    player.play()
}

@_cdecl("scn_animation_player_stop")
public func scn_animation_player_stop(_ playerHandle: UnsafeMutableRawPointer?) {
    guard let player: SCNAnimationPlayer = scnBorrow(playerHandle) else { return }
    player.stop()
}

@_cdecl("scn_node_add_animation_player")
public func scn_node_add_animation_player(
    _ nodeHandle: UnsafeMutableRawPointer?,
    _ playerHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) {
    guard let node: SCNNode = scnBorrow(nodeHandle),
          let player: SCNAnimationPlayer = scnBorrow(playerHandle)
    else { return }
    node.addAnimationPlayer(player, forKey: key.map(String.init(cString:)))
}

@_cdecl("scn_node_animation_player")
public func scn_node_animation_player(
    _ nodeHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let key else { return nil }
    guard let player = node.animationPlayer(forKey: String(cString: key)) else { return nil }
    return scnRetain(player)
}
