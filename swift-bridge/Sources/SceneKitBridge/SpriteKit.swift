import SpriteKit

@_cdecl("scn_sprite_scene_new")
public func scn_sprite_scene_new(_ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SKScene(size: CGSize(width: width, height: height)))
}

@_cdecl("scn_sprite_transition_fade")
public func scn_sprite_transition_fade(_ duration: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SKTransition.fade(withDuration: duration))
}
