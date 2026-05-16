import Metal
import SceneKit

final class SceneRenderPassDescriptorBox: NSObject {
    let descriptor: MTLRenderPassDescriptor

    init(texture: MTLTexture, clearColor: MTLClearColor, loadAction: MTLLoadAction, storeAction: MTLStoreAction) {
        let descriptor = MTLRenderPassDescriptor()
        descriptor.colorAttachments[0].texture = texture
        descriptor.colorAttachments[0].loadAction = loadAction
        descriptor.colorAttachments[0].storeAction = storeAction
        descriptor.colorAttachments[0].clearColor = clearColor
        self.descriptor = descriptor
    }
}

func scnBorrowRenderPassDescriptor(_ handle: UnsafeMutableRawPointer?) -> MTLRenderPassDescriptor? {
    guard let box: SceneRenderPassDescriptorBox = scnBorrow(handle) else { return nil }
    return box.descriptor
}

@_cdecl("scn_render_pass_descriptor_new_for_texture")
public func scn_render_pass_descriptor_new_for_texture(
    _ textureHandle: UnsafeMutableRawPointer?,
    _ clearR: Double,
    _ clearG: Double,
    _ clearB: Double,
    _ clearA: Double,
    _ loadAction: Int32,
    _ storeAction: Int32
) -> UnsafeMutableRawPointer? {
    guard let texture: MTLTexture = scnBorrow(textureHandle) else { return nil }
    let box = SceneRenderPassDescriptorBox(
        texture: texture,
        clearColor: MTLClearColor(red: clearR, green: clearG, blue: clearB, alpha: clearA),
        loadAction: scnLoadAction(loadAction),
        storeAction: scnStoreAction(storeAction)
    )
    return scnRetain(box)
}

@_cdecl("scn_renderer_new")
public func scn_renderer_new(_ deviceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let device: MTLDevice? = scnBorrow(deviceHandle)
    return scnRetain(SCNRenderer(device: device, options: nil))
}

@_cdecl("scn_renderer_set_scene")
public func scn_renderer_set_scene(_ rendererHandle: UnsafeMutableRawPointer?, _ sceneHandle: UnsafeMutableRawPointer?) {
    guard let renderer: SCNRenderer = scnBorrow(rendererHandle) else { return }
    let scene: SCNScene? = scnBorrow(sceneHandle)
    renderer.scene = scene
}

@_cdecl("scn_renderer_set_point_of_view")
public func scn_renderer_set_point_of_view(_ rendererHandle: UnsafeMutableRawPointer?, _ nodeHandle: UnsafeMutableRawPointer?) {
    guard let renderer: SCNRenderer = scnBorrow(rendererHandle) else { return }
    let node: SCNNode? = scnBorrow(nodeHandle)
    renderer.pointOfView = node
}

@_cdecl("scn_renderer_render")
public func scn_renderer_render(
    _ rendererHandle: UnsafeMutableRawPointer?,
    _ time: Double,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ commandBufferHandle: UnsafeMutableRawPointer?,
    _ passDescriptorHandle: UnsafeMutableRawPointer?
) {
    guard let renderer: SCNRenderer = scnBorrow(rendererHandle),
          let commandBuffer: MTLCommandBuffer = scnBorrow(commandBufferHandle),
          let passDescriptor = scnBorrowRenderPassDescriptor(passDescriptorHandle)
    else { return }
    renderer.render(atTime: time, viewport: CGRect(x: x, y: y, width: width, height: height), commandBuffer: commandBuffer, passDescriptor: passDescriptor)
}

@_cdecl("scn_texture_copy_bytes")
public func scn_texture_copy_bytes(
    _ textureHandle: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutableRawPointer?,
    _ bytesPerRow: Int
) -> Bool {
    guard let texture: MTLTexture = scnBorrow(textureHandle), let outBytes else { return false }
    let region = MTLRegionMake2D(0, 0, texture.width, texture.height)
    texture.getBytes(outBytes, bytesPerRow: bytesPerRow, from: region, mipmapLevel: 0)
    return true
}
