import Foundation
import SceneKit

public typealias SceneActionCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, Double) -> Void
public typealias SceneActionDropCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

final class RustSceneActionCallbackBox {
    let context: UnsafeMutableRawPointer?
    let callback: SceneActionCallback?
    let dropCallback: SceneActionDropCallback?

    init(
        context: UnsafeMutableRawPointer?,
        callback: SceneActionCallback?,
        dropCallback: SceneActionDropCallback?
    ) {
        self.context = context
        self.callback = callback
        self.dropCallback = dropCallback
    }

    func invoke(node: SCNNode, elapsedTime: CGFloat) {
        callback?(context, Unmanaged.passUnretained(node).toOpaque(), Double(elapsedTime))
    }

    deinit {
        dropCallback?(context)
    }
}

func scnActions(from rawActions: UnsafeMutableRawPointer?, count: Int) -> [SCNAction] {
    guard let rawActions else { return [] }
    let pointers = rawActions.assumingMemoryBound(to: UnsafeMutableRawPointer?.self)
    var actions: [SCNAction] = []
    actions.reserveCapacity(count)
    for index in 0..<count {
        if let action: SCNAction = scnBorrow(pointers[index]) {
            actions.append(action)
        }
    }
    return actions
}

@_cdecl("scn_action_move_to")
public func scn_action_move_to(_ x: Float, _ y: Float, _ z: Float, _ duration: Double) -> UnsafeMutableRawPointer? {
    scnRetain(
        SCNAction.move(
            to: SCNVector3(x: CGFloat(x), y: CGFloat(y), z: CGFloat(z)),
            duration: duration
        )
    )
}

@_cdecl("scn_action_move_by")
public func scn_action_move_by(_ x: Float, _ y: Float, _ z: Float, _ duration: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNAction.moveBy(x: CGFloat(x), y: CGFloat(y), z: CGFloat(z), duration: duration))
}

@_cdecl("scn_action_rotate_by")
public func scn_action_rotate_by(_ x: Float, _ y: Float, _ z: Float, _ duration: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNAction.rotateBy(x: CGFloat(x), y: CGFloat(y), z: CGFloat(z), duration: duration))
}

@_cdecl("scn_action_scale_by")
public func scn_action_scale_by(_ scale: Float, _ duration: Double) -> UnsafeMutableRawPointer? {
    scnRetain(SCNAction.scale(by: CGFloat(scale), duration: duration))
}

@_cdecl("scn_action_sequence")
public func scn_action_sequence(_ rawActions: UnsafeMutableRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    scnRetain(SCNAction.sequence(scnActions(from: rawActions, count: count)))
}

@_cdecl("scn_action_group")
public func scn_action_group(_ rawActions: UnsafeMutableRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    scnRetain(SCNAction.group(scnActions(from: rawActions, count: count)))
}

@_cdecl("scn_action_repeat")
public func scn_action_repeat(_ actionHandle: UnsafeMutableRawPointer?, _ count: Int) -> UnsafeMutableRawPointer? {
    guard let action: SCNAction = scnBorrow(actionHandle) else { return nil }
    return scnRetain(SCNAction.repeat(action, count: count))
}

@_cdecl("scn_action_repeat_forever")
public func scn_action_repeat_forever(_ actionHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let action: SCNAction = scnBorrow(actionHandle) else { return nil }
    return scnRetain(SCNAction.repeatForever(action))
}

@_cdecl("scn_action_custom")
public func scn_action_custom(
    _ duration: Double,
    _ context: UnsafeMutableRawPointer?,
    _ callback: SceneActionCallback?,
    _ dropCallback: SceneActionDropCallback?
) -> UnsafeMutableRawPointer? {
    let box = RustSceneActionCallbackBox(context: context, callback: callback, dropCallback: dropCallback)
    let action = SCNAction.customAction(duration: duration) { node, elapsedTime in
        box.invoke(node: node, elapsedTime: elapsedTime)
    }
    return scnRetain(action)
}
