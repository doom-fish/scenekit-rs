import Foundation
import ObjectiveC
import QuartzCore
import SceneKit

final class StringArrayBox: NSObject {
    let values: [String]

    init(_ values: [String]) {
        self.values = values
    }
}

private func scnBorrowActionable(_ handle: UnsafeMutableRawPointer?) -> (NSObjectProtocol & SCNActionable)? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? (NSObjectProtocol & SCNActionable)
}

private func scnBorrowAnimatable(_ handle: UnsafeMutableRawPointer?) -> (NSObjectProtocol & SCNAnimatable)? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? (NSObjectProtocol & SCNAnimatable)
}

private func scnBorrowBoundingVolume(_ handle: UnsafeMutableRawPointer?) -> (NSObjectProtocol & SCNBoundingVolume)? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? (NSObjectProtocol & SCNBoundingVolume)
}

private func scnBorrowTechniqueSupport(_ handle: UnsafeMutableRawPointer?) -> (NSObjectProtocol & SCNTechniqueSupport)? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? (NSObjectProtocol & SCNTechniqueSupport)
}

private func scnBorrowStringArray(_ handle: UnsafeMutableRawPointer?) -> StringArrayBox? {
    scnBorrow(handle)
}

@_cdecl("scn_actionable_run_action_for_key")
public func scn_actionable_run_action_for_key(
    _ objectHandle: UnsafeMutableRawPointer?,
    _ actionHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) {
    guard let object = scnBorrowActionable(objectHandle), let action: SCNAction = scnBorrow(actionHandle) else { return }
    object.runAction(action, forKey: key.map(String.init(cString:)))
}

@_cdecl("scn_actionable_has_actions")
public func scn_actionable_has_actions(_ objectHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let object = scnBorrowActionable(objectHandle) else { return false }
    return object.hasActions
}

@_cdecl("scn_actionable_action_for_key")
public func scn_actionable_action_for_key(
    _ objectHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let object = scnBorrowActionable(objectHandle), let key else { return nil }
    guard let action = object.action(forKey: String(cString: key)) else { return nil }
    return scnRetain(action)
}

@_cdecl("scn_actionable_remove_action_for_key")
public func scn_actionable_remove_action_for_key(_ objectHandle: UnsafeMutableRawPointer?, _ key: UnsafePointer<CChar>?) {
    guard let object = scnBorrowActionable(objectHandle), let key else { return }
    object.removeAction(forKey: String(cString: key))
}

@_cdecl("scn_actionable_remove_all_actions")
public func scn_actionable_remove_all_actions(_ objectHandle: UnsafeMutableRawPointer?) {
    scnBorrowActionable(objectHandle)?.removeAllActions()
}

@_cdecl("scn_actionable_action_keys")
public func scn_actionable_action_keys(_ objectHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let object = scnBorrowActionable(objectHandle) else { return nil }
    return scnRetain(StringArrayBox(object.actionKeys))
}

@_cdecl("scn_animatable_add_animation")
public func scn_animatable_add_animation(
    _ objectHandle: UnsafeMutableRawPointer?,
    _ animationHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) {
    guard let object = scnBorrowAnimatable(objectHandle), let animation: SCNAnimation = scnBorrow(animationHandle) else { return }
    object.addAnimation(animation, forKey: key.map(String.init(cString:)))
}

@_cdecl("scn_animatable_add_animation_player")
public func scn_animatable_add_animation_player(
    _ objectHandle: UnsafeMutableRawPointer?,
    _ playerHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) {
    guard let object = scnBorrowAnimatable(objectHandle), let player: SCNAnimationPlayer = scnBorrow(playerHandle) else { return }
    object.addAnimationPlayer(player, forKey: key.map(String.init(cString:)))
}

@_cdecl("scn_animatable_remove_all_animations")
public func scn_animatable_remove_all_animations(_ objectHandle: UnsafeMutableRawPointer?) {
    scnBorrowAnimatable(objectHandle)?.removeAllAnimations()
}

@_cdecl("scn_animatable_remove_animation_for_key")
public func scn_animatable_remove_animation_for_key(_ objectHandle: UnsafeMutableRawPointer?, _ key: UnsafePointer<CChar>?) {
    guard let object = scnBorrowAnimatable(objectHandle), let key else { return }
    object.removeAnimation(forKey: String(cString: key))
}

@_cdecl("scn_animatable_animation_player")
public func scn_animatable_animation_player(
    _ objectHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let object = scnBorrowAnimatable(objectHandle), let key else { return nil }
    guard let player = object.animationPlayer(forKey: String(cString: key)) else { return nil }
    return scnRetain(player)
}

@_cdecl("scn_animatable_animation_keys")
public func scn_animatable_animation_keys(_ objectHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let object = scnBorrowAnimatable(objectHandle) else { return nil }
    return scnRetain(StringArrayBox(object.animationKeys))
}

@_cdecl("scn_bounding_volume_get_bounding_box")
public func scn_bounding_volume_get_bounding_box(
    _ objectHandle: UnsafeMutableRawPointer?,
    _ outMin: UnsafeMutableRawPointer?,
    _ outMax: UnsafeMutableRawPointer?
) -> Bool {
    guard let object = scnBorrowBoundingVolume(objectHandle) else { return false }
    var min = SCNVector3Zero
    var max = SCNVector3Zero
    let ok = object.__getBoundingBoxMin(&min, max: &max)
    if ok {
        _ = scnWriteVector3(min, out: outMin)
        _ = scnWriteVector3(max, out: outMax)
    }
    return ok
}

@_cdecl("scn_bounding_volume_set_bounding_box")
public func scn_bounding_volume_set_bounding_box(
    _ objectHandle: UnsafeMutableRawPointer?,
    _ minHandle: UnsafeMutableRawPointer?,
    _ maxHandle: UnsafeMutableRawPointer?
) {
    guard let object = scnBorrowBoundingVolume(objectHandle) else { return }
    if let min = scnReadVector3(minHandle), let max = scnReadVector3(maxHandle) {
        var min = min
        var max = max
        object.__setBoundingBoxMin(&min, max: &max)
    } else {
        object.__setBoundingBoxMin(nil, max: nil)
    }
}

@_cdecl("scn_bounding_volume_get_bounding_sphere")
public func scn_bounding_volume_get_bounding_sphere(
    _ objectHandle: UnsafeMutableRawPointer?,
    _ outCenter: UnsafeMutableRawPointer?,
    _ outRadius: UnsafeMutablePointer<Double>?
) -> Bool {
    guard let object = scnBorrowBoundingVolume(objectHandle), let outRadius else { return false }
    var center = SCNVector3Zero
    var radius: CGFloat = 0
    let ok = object.__getBoundingSphereCenter(&center, radius: &radius)
    if ok {
        _ = scnWriteVector3(center, out: outCenter)
        outRadius.pointee = radius
    }
    return ok
}

@_cdecl("scn_technique_support_get_technique")
public func scn_technique_support_get_technique(_ objectHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let object = scnBorrowTechniqueSupport(objectHandle), let technique = object.technique else { return nil }
    return scnRetain(technique)
}

@_cdecl("scn_technique_support_set_technique")
public func scn_technique_support_set_technique(_ objectHandle: UnsafeMutableRawPointer?, _ techniqueHandle: UnsafeMutableRawPointer?) {
    guard let object = scnBorrowTechniqueSupport(objectHandle) else { return }
    let technique: SCNTechnique? = scnBorrow(techniqueHandle)
    object.technique = technique
}

@_cdecl("scn_string_array_count")
public func scn_string_array_count(_ arrayHandle: UnsafeMutableRawPointer?) -> Int {
    guard let array = scnBorrowStringArray(arrayHandle) else { return 0 }
    return array.values.count
}

@_cdecl("scn_string_array_get")
public func scn_string_array_get(_ arrayHandle: UnsafeMutableRawPointer?, _ index: Int) -> UnsafeMutablePointer<CChar>? {
    guard let array = scnBorrowStringArray(arrayHandle), index >= 0, index < array.values.count else { return nil }
    return scnDup(array.values[index])
}

@_cdecl("scn_timing_function_new_mode")
public func scn_timing_function_new_mode(_ mode: Int32) -> UnsafeMutableRawPointer? {
    let timingMode = SCNActionTimingMode(rawValue: Int(mode)) ?? .linear
    return scnRetain(SCNTimingFunction(timingMode: timingMode))
}

public typealias AnimationEventCallback = @convention(c) (UnsafeMutableRawPointer?, Bool) -> Void

private final class AnimationEventBox: NSObject {
    let context: UnsafeMutableRawPointer?
    let releaseContext: ScnReleaseContextCallback?

    init(context: UnsafeMutableRawPointer?, releaseContext: ScnReleaseContextCallback?) {
        self.context = context
        self.releaseContext = releaseContext
    }

    deinit {
        releaseContext?(context)
    }
}

private var animationEventAssociationKey: UInt8 = 0

@_cdecl("scn_animation_event_new")
public func scn_animation_event_new(
    _ keyTime: Float,
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: ScnReleaseContextCallback?,
    _ callback: @escaping AnimationEventCallback
) -> UnsafeMutableRawPointer? {
    let event = SCNAnimationEvent(keyTime: CGFloat(keyTime)) { _, _, playingBackward in
        callback(context, playingBackward)
    }
    objc_setAssociatedObject(
        event,
        &animationEventAssociationKey,
        AnimationEventBox(context: context, releaseContext: releaseContext),
        .OBJC_ASSOCIATION_RETAIN_NONATOMIC
    )
    return scnRetain(event)
}
