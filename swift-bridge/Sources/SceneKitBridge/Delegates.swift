import Foundation
import JavaScriptCore
import SceneKit

private final class NodeRendererDelegateBox: NSObject, SCNNodeRendererDelegate {
    let context: UnsafeMutableRawPointer
    let renderCallback: @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
    let releaseContext: @convention(c) (UnsafeMutableRawPointer?) -> Void

    init(
        context: UnsafeMutableRawPointer,
        releaseContext: @escaping @convention(c) (UnsafeMutableRawPointer?) -> Void,
        renderCallback: @escaping @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.renderCallback = renderCallback
    }

    deinit {
        releaseContext(context)
    }

    func renderNode(_ node: SCNNode, renderer: SCNRenderer, arguments: [String : Any]) {
        renderCallback(context, scnRetain(node), scnRetain(renderer))
    }
}

@_cdecl("scn_node_renderer_delegate_new")
public func scn_node_renderer_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: @escaping @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ renderCallback: @escaping @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
) -> UnsafeMutableRawPointer? {
    guard let context else { return nil }
    return scnRetain(NodeRendererDelegateBox(context: context, releaseContext: releaseContext, renderCallback: renderCallback))
}

@_cdecl("scn_node_get_renderer_delegate")
public func scn_node_get_renderer_delegate(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle), let delegate = node.rendererDelegate else { return nil }
    return scnRetain(delegate)
}

@_cdecl("scn_node_set_renderer_delegate")
public func scn_node_set_renderer_delegate(_ nodeHandle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    let delegate: SCNNodeRendererDelegate? = scnBorrow(delegateHandle)
    node.rendererDelegate = delegate
}

@_cdecl("scn_node_test_invoke_renderer_delegate")
public func scn_node_test_invoke_renderer_delegate(_ nodeHandle: UnsafeMutableRawPointer?, _ rendererHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle), let renderer: SCNRenderer = scnBorrow(rendererHandle), let delegate = node.rendererDelegate else { return }
    delegate.renderNode?(node, renderer: renderer, arguments: [:])
}

private final class AvoidOccluderConstraintDelegateBox: NSObject, SCNAvoidOccluderConstraintDelegate {
    let context: UnsafeMutableRawPointer
    let shouldAvoidCallback: @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Bool
    let didAvoidCallback: @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
    let releaseContext: @convention(c) (UnsafeMutableRawPointer?) -> Void

    init(
        context: UnsafeMutableRawPointer,
        releaseContext: @escaping @convention(c) (UnsafeMutableRawPointer?) -> Void,
        shouldAvoidCallback: @escaping @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Bool,
        didAvoidCallback: @escaping @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.shouldAvoidCallback = shouldAvoidCallback
        self.didAvoidCallback = didAvoidCallback
    }

    deinit {
        releaseContext(context)
    }

    func avoidOccluderConstraint(_ constraint: SCNAvoidOccluderConstraint, shouldAvoidOccluder occluder: SCNNode, for node: SCNNode) -> Bool {
        shouldAvoidCallback(context, scnRetain(occluder), scnRetain(node))
    }

    func avoidOccluderConstraint(_ constraint: SCNAvoidOccluderConstraint, didAvoidOccluder occluder: SCNNode, for node: SCNNode) {
        didAvoidCallback(context, scnRetain(occluder), scnRetain(node))
    }
}

@_cdecl("scn_avoid_occluder_constraint_delegate_new")
public func scn_avoid_occluder_constraint_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: @escaping @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ shouldAvoidCallback: @escaping @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Bool,
    _ didAvoidCallback: @escaping @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
) -> UnsafeMutableRawPointer? {
    guard let context else { return nil }
    return scnRetain(
        AvoidOccluderConstraintDelegateBox(
            context: context,
            releaseContext: releaseContext,
            shouldAvoidCallback: shouldAvoidCallback,
            didAvoidCallback: didAvoidCallback
        )
    )
}

@_cdecl("scn_avoid_occluder_constraint_get_delegate")
public func scn_avoid_occluder_constraint_get_delegate(_ constraintHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle) else { return nil }
    guard let delegate = constraint.value(forKey: "delegate") as? SCNAvoidOccluderConstraintDelegate else { return nil }
    return scnRetain(delegate)
}

@_cdecl("scn_avoid_occluder_constraint_set_delegate")
public func scn_avoid_occluder_constraint_set_delegate(_ constraintHandle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle) else { return }
    let delegate: SCNAvoidOccluderConstraintDelegate? = scnBorrow(delegateHandle)
    constraint.setValue(delegate, forKey: "delegate")
}

@_cdecl("scn_avoid_occluder_constraint_test_invoke_should")
public func scn_avoid_occluder_constraint_test_invoke_should(
    _ constraintHandle: UnsafeMutableRawPointer?,
    _ occluderHandle: UnsafeMutableRawPointer?,
    _ nodeHandle: UnsafeMutableRawPointer?
) -> Bool {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle), let occluder: SCNNode = scnBorrow(occluderHandle), let node: SCNNode = scnBorrow(nodeHandle) else { return true }
    let delegate = constraint.value(forKey: "delegate") as? SCNAvoidOccluderConstraintDelegate
    return delegate?.avoidOccluderConstraint?(constraint, shouldAvoidOccluder: occluder, for: node) ?? true
}

@_cdecl("scn_avoid_occluder_constraint_test_invoke_did")
public func scn_avoid_occluder_constraint_test_invoke_did(
    _ constraintHandle: UnsafeMutableRawPointer?,
    _ occluderHandle: UnsafeMutableRawPointer?,
    _ nodeHandle: UnsafeMutableRawPointer?
) {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle), let occluder: SCNNode = scnBorrow(occluderHandle), let node: SCNNode = scnBorrow(nodeHandle) else { return }
    let delegate = constraint.value(forKey: "delegate") as? SCNAvoidOccluderConstraintDelegate
    delegate?.avoidOccluderConstraint?(constraint, didAvoidOccluder: occluder, for: node)
}

private final class SceneExportDelegateBox: NSObject, SCNSceneExportDelegate {
    let context: UnsafeMutableRawPointer
    let writeImageCallback: @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, UnsafePointer<CChar>?) -> UnsafePointer<CChar>?
    let releaseContext: @convention(c) (UnsafeMutableRawPointer?) -> Void

    init(
        context: UnsafeMutableRawPointer,
        releaseContext: @escaping @convention(c) (UnsafeMutableRawPointer?) -> Void,
        writeImageCallback: @escaping @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, UnsafePointer<CChar>?) -> UnsafePointer<CChar>?
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.writeImageCallback = writeImageCallback
    }

    deinit {
        releaseContext(context)
    }

    func write(_ image: NSImage, withSceneDocumentURL documentURL: URL, originalImageURL: URL?) -> URL? {
        let documentCString = documentURL.path.withCString { strdup($0) }
        let originalCString = originalImageURL?.path.withCString { strdup($0) }
        defer {
            if let documentCString { free(documentCString) }
            if let originalCString { free(originalCString) }
        }
        guard let resolved = writeImageCallback(context, documentCString, originalCString) else {
            return nil
        }
        return URL(fileURLWithPath: String(cString: resolved))
    }
}

@_cdecl("scn_scene_export_delegate_new")
public func scn_scene_export_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: @escaping @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ writeImageCallback: @escaping @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, UnsafePointer<CChar>?) -> UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let context else { return nil }
    return scnRetain(SceneExportDelegateBox(context: context, releaseContext: releaseContext, writeImageCallback: writeImageCallback))
}

@_cdecl("scn_scene_write_to_url")
public func scn_scene_write_to_url(_ sceneHandle: UnsafeMutableRawPointer?, _ path: UnsafePointer<CChar>?, _ delegateHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let scene: SCNScene = scnBorrow(sceneHandle), let path else { return false }
    let delegate: SCNSceneExportDelegate? = scnBorrow(delegateHandle)
    let url = URL(fileURLWithPath: String(cString: path))
    scene.write(to: url, options: nil, delegate: delegate, progressHandler: nil)
    return FileManager.default.fileExists(atPath: url.path)
}

@_cdecl("scn_export_javascript_module")
public func scn_export_javascript_module(_ contextHandle: UnsafeMutableRawPointer?) {
    guard let contextHandle else { return }
    let object = Unmanaged<AnyObject>.fromOpaque(contextHandle).takeUnretainedValue()
    guard let context = object as? JSContext else { return }
    SCNExportJavaScriptModule(context)
}
