import AppKit
import Foundation
import JavaScriptCore
import ObjectiveC
import SceneKit

public typealias ScnNodePairCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void
public typealias ScnNodePairPredicate = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Bool
public typealias ScnWriteImageCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafePointer<CChar>?, UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?

private var nodeRendererDelegateKey: UInt8 = 0
private var avoidOccluderDelegateKey: UInt8 = 0

private final class NodeRendererDelegateBox: NSObject, SCNNodeRendererDelegate {
    let context: UnsafeMutableRawPointer
    let releaseContext: ScnReleaseContextCallback
    let renderCallback: ScnNodePairCallback

    init(context: UnsafeMutableRawPointer, releaseContext: @escaping ScnReleaseContextCallback, renderCallback: @escaping ScnNodePairCallback) {
        self.context = context
        self.releaseContext = releaseContext
        self.renderCallback = renderCallback
    }

    deinit {
        releaseContext(context)
    }

    func renderNode(_ node: SCNNode, renderer: SCNRenderer, arguments: [String : Any]) {
        renderCallback(context, Unmanaged.passUnretained(node).toOpaque(), Unmanaged.passUnretained(renderer).toOpaque())
    }
}

@_cdecl("scn_node_renderer_delegate_new")
public func scn_node_renderer_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: @escaping ScnReleaseContextCallback,
    _ renderCallback: @escaping ScnNodePairCallback
) -> UnsafeMutableRawPointer? {
    guard let context else { return nil }
    return scnRetain(NodeRendererDelegateBox(context: context, releaseContext: releaseContext, renderCallback: renderCallback))
}

@_cdecl("scn_node_get_renderer_delegate")
public func scn_node_get_renderer_delegate(_ nodeHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return nil }
    return scnRetainedDelegate(of: node, key: &nodeRendererDelegateKey)
}

@_cdecl("scn_node_set_renderer_delegate")
public func scn_node_set_renderer_delegate(_ nodeHandle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle) else { return }
    let delegate: NodeRendererDelegateBox? = scnBorrow(delegateHandle)
    scnRetainDelegate(delegate, by: node, key: &nodeRendererDelegateKey) {
        node.rendererDelegate = delegate
    }
}

func scnAdoptRendererDelegates(from original: SCNNode, to clone: SCNNode) {
    if let delegate = objc_getAssociatedObject(original, &nodeRendererDelegateKey) as? NodeRendererDelegateBox,
       (clone.rendererDelegate as AnyObject?) === delegate {
        objc_setAssociatedObject(clone, &nodeRendererDelegateKey, delegate, .OBJC_ASSOCIATION_RETAIN)
    }
    for (originalChild, cloneChild) in zip(original.childNodes, clone.childNodes) {
        scnAdoptRendererDelegates(from: originalChild, to: cloneChild)
    }
}

@_cdecl("scn_node_test_invoke_renderer_delegate")
public func scn_node_test_invoke_renderer_delegate(_ nodeHandle: UnsafeMutableRawPointer?, _ rendererHandle: UnsafeMutableRawPointer?) {
    guard let node: SCNNode = scnBorrow(nodeHandle), let renderer: SCNRenderer = scnBorrow(rendererHandle), let delegate = node.rendererDelegate else { return }
    delegate.renderNode?(node, renderer: renderer, arguments: [:])
}

private final class AvoidOccluderConstraintDelegateBox: NSObject, SCNAvoidOccluderConstraintDelegate {
    let context: UnsafeMutableRawPointer
    let releaseContext: ScnReleaseContextCallback
    let shouldAvoidCallback: ScnNodePairPredicate
    let didAvoidCallback: ScnNodePairCallback

    init(
        context: UnsafeMutableRawPointer,
        releaseContext: @escaping ScnReleaseContextCallback,
        shouldAvoidCallback: @escaping ScnNodePairPredicate,
        didAvoidCallback: @escaping ScnNodePairCallback
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
        shouldAvoidCallback(context, Unmanaged.passUnretained(occluder).toOpaque(), Unmanaged.passUnretained(node).toOpaque())
    }

    func avoidOccluderConstraint(_ constraint: SCNAvoidOccluderConstraint, didAvoidOccluder occluder: SCNNode, for node: SCNNode) {
        didAvoidCallback(context, Unmanaged.passUnretained(occluder).toOpaque(), Unmanaged.passUnretained(node).toOpaque())
    }
}

@_cdecl("scn_avoid_occluder_constraint_delegate_new")
public func scn_avoid_occluder_constraint_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: @escaping ScnReleaseContextCallback,
    _ shouldAvoidCallback: @escaping ScnNodePairPredicate,
    _ didAvoidCallback: @escaping ScnNodePairCallback
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
    return scnRetainedDelegate(of: constraint, key: &avoidOccluderDelegateKey)
}

@_cdecl("scn_avoid_occluder_constraint_set_delegate")
public func scn_avoid_occluder_constraint_set_delegate(_ constraintHandle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let constraint: SCNAvoidOccluderConstraint = scnBorrow(constraintHandle) else { return }
    let delegate: AvoidOccluderConstraintDelegateBox? = scnBorrow(delegateHandle)
    scnRetainDelegate(delegate, by: constraint, key: &avoidOccluderDelegateKey) {
        constraint.setValue(delegate, forKey: "delegate")
    }
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
    let releaseContext: ScnReleaseContextCallback
    let writeImageCallback: ScnWriteImageCallback

    init(context: UnsafeMutableRawPointer, releaseContext: @escaping ScnReleaseContextCallback, writeImageCallback: @escaping ScnWriteImageCallback) {
        self.context = context
        self.releaseContext = releaseContext
        self.writeImageCallback = writeImageCallback
    }

    deinit {
        releaseContext(context)
    }

    func write(_ image: NSImage, withSceneDocumentURL documentURL: URL, originalImageURL: URL?) -> URL? {
        guard let cgImage = image.cgImage(forProposedRect: nil, context: nil, hints: nil) else { return nil }
        let imageHandle = Unmanaged.passRetained(cgImage).toOpaque()
        let resolved: UnsafeMutablePointer<CChar>? = documentURL.path.withCString { documentPath in
            guard let originalPath = originalImageURL?.path else {
                return writeImageCallback(context, imageHandle, documentPath, nil)
            }
            return originalPath.withCString { writeImageCallback(context, imageHandle, documentPath, $0) }
        }
        guard let resolved else { return nil }
        defer { free(resolved) }
        return URL(fileURLWithPath: String(cString: resolved))
    }
}

@_cdecl("scn_scene_export_delegate_new")
public func scn_scene_export_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: @escaping ScnReleaseContextCallback,
    _ writeImageCallback: @escaping ScnWriteImageCallback
) -> UnsafeMutableRawPointer? {
    guard let context else { return nil }
    return scnRetain(SceneExportDelegateBox(context: context, releaseContext: releaseContext, writeImageCallback: writeImageCallback))
}

private final class SceneExportErrorBox {
    private let lock = NSLock()
    private var message: String?

    func record(_ error: Error) {
        lock.lock()
        defer { lock.unlock() }
        message = error.localizedDescription
    }

    var recorded: String? {
        lock.lock()
        defer { lock.unlock() }
        return message
    }
}

@_cdecl("scn_scene_write_to_url")
public func scn_scene_write_to_url(
    _ sceneHandle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ delegateHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    outError?.pointee = nil
    guard let scene: SCNScene = scnBorrow(sceneHandle), let path else {
        outError?.pointee = scnDup("missing scene or path")
        return false
    }
    let delegate: SCNSceneExportDelegate? = scnBorrow(delegateHandle)
    let url = URL(fileURLWithPath: String(cString: path))
    let errors = SceneExportErrorBox()
    let written = scene.write(to: url, options: nil, delegate: delegate) { _, error, _ in
        if let error {
            errors.record(error)
        }
    }
    if !written {
        outError?.pointee = scnDup(errors.recorded ?? "SCNScene.write(to:options:delegate:progressHandler:) returned false")
    }
    return written
}

@_cdecl("scn_export_javascript_module")
public func scn_export_javascript_module(_ contextHandle: UnsafeMutableRawPointer?) {
    guard let contextHandle else { return }
    let object = Unmanaged<AnyObject>.fromOpaque(contextHandle).takeUnretainedValue()
    guard let context = object as? JSContext else { return }
    SCNExportJavaScriptModule(context)
}
