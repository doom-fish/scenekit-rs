import AVFAudio
import Foundation
import SceneKit

@_cdecl("scn_audio_source_new_url")
public func scn_audio_source_new_url(_ path: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let path,
          let source = SCNAudioSource(url: URL(fileURLWithPath: String(cString: path)))
    else { return nil }
    return scnRetain(source)
}

@_cdecl("scn_audio_source_get_volume")
public func scn_audio_source_get_volume(_ sourceHandle: UnsafeMutableRawPointer?) -> Float {
    guard let source: SCNAudioSource = scnBorrow(sourceHandle) else { return 0 }
    return source.volume
}

@_cdecl("scn_audio_source_set_volume")
public func scn_audio_source_set_volume(_ sourceHandle: UnsafeMutableRawPointer?, _ volume: Float) {
    guard let source: SCNAudioSource = scnBorrow(sourceHandle) else { return }
    source.volume = volume
}

@_cdecl("scn_audio_source_get_positional")
public func scn_audio_source_get_positional(_ sourceHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let source: SCNAudioSource = scnBorrow(sourceHandle) else { return false }
    return source.isPositional
}

@_cdecl("scn_audio_source_set_positional")
public func scn_audio_source_set_positional(_ sourceHandle: UnsafeMutableRawPointer?, _ positional: Bool) {
    guard let source: SCNAudioSource = scnBorrow(sourceHandle) else { return }
    source.isPositional = positional
}

@_cdecl("scn_audio_source_get_loops")
public func scn_audio_source_get_loops(_ sourceHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let source: SCNAudioSource = scnBorrow(sourceHandle) else { return false }
    return source.loops
}

@_cdecl("scn_audio_source_set_loops")
public func scn_audio_source_set_loops(_ sourceHandle: UnsafeMutableRawPointer?, _ loops: Bool) {
    guard let source: SCNAudioSource = scnBorrow(sourceHandle) else { return }
    source.loops = loops
}

@_cdecl("scn_audio_source_load")
public func scn_audio_source_load(_ sourceHandle: UnsafeMutableRawPointer?) {
    guard let source: SCNAudioSource = scnBorrow(sourceHandle) else { return }
    source.load()
}

@_cdecl("scn_audio_player_new")
public func scn_audio_player_new(_ sourceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let source: SCNAudioSource = scnBorrow(sourceHandle) else { return nil }
    return scnRetain(SCNAudioPlayer(source: source))
}

@_cdecl("scn_audio_player_source")
public func scn_audio_player_source(_ playerHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let player: SCNAudioPlayer = scnBorrow(playerHandle), let source = player.audioSource else { return nil }
    return scnRetain(source)
}

@_cdecl("scn_node_add_audio_player")
public func scn_node_add_audio_player(_ nodeHandle: UnsafeMutableRawPointer?, _ playerHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle),
          let player: SCNAudioPlayer = scnBorrow(playerHandle)
    else { return }
    node.addAudioPlayer(player)
}

@_cdecl("scn_node_remove_all_audio_players")
public func scn_node_remove_all_audio_players(_ nodeHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    node.removeAllAudioPlayers()
}

@_cdecl("scn_node_audio_player_count")
public func scn_node_audio_player_count(_ nodeHandle: UnsafeMutableRawPointer?) -> Int {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return 0 }
    return node.audioPlayers.count
}
