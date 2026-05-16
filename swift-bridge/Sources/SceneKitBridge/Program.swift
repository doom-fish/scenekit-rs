import Foundation
import ObjectiveC
import SceneKit

public typealias ProgramErrorCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutablePointer<CChar>?) -> Void
public typealias ProgramBufferBindingCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void

private var programBindingStoreAssociationKey: UInt8 = 0

private enum ShadableObject {
    case material(SCNMaterial)
    case geometry(SCNGeometry)
}

private func scnBorrowShadable(_ handle: UnsafeMutableRawPointer?) -> ShadableObject? {
    if let material: SCNMaterial = scnBorrow(handle) {
        return .material(material)
    }
    if let geometry: SCNGeometry = scnBorrow(handle) {
        return .geometry(geometry)
    }
    return nil
}

private func scnProgram(for shadable: ShadableObject) -> SCNProgram? {
    switch shadable {
    case .material(let material):
        return material.program
    case .geometry(let geometry):
        return geometry.program
    }
}

private func scnSetProgram(_ program: SCNProgram?, on shadable: ShadableObject) {
    switch shadable {
    case .material(let material):
        material.program = program
    case .geometry(let geometry):
        geometry.program = program
    }
}

private func scnShaderModifiers(for shadable: ShadableObject) -> [SCNShaderModifierEntryPoint: String]? {
    switch shadable {
    case .material(let material):
        return material.shaderModifiers
    case .geometry(let geometry):
        return geometry.shaderModifiers
    }
}

private func scnSetShaderModifiers(
    _ shaderModifiers: [SCNShaderModifierEntryPoint: String]?,
    on shadable: ShadableObject
) {
    switch shadable {
    case .material(let material):
        material.shaderModifiers = shaderModifiers
    case .geometry(let geometry):
        geometry.shaderModifiers = shaderModifiers
    }
}

private func scnShaderModifierEntryPoint(from rawValue: String) -> SCNShaderModifierEntryPoint? {
    switch rawValue {
    case SCNShaderModifierEntryPoint.surface.rawValue:
        return .surface
    case SCNShaderModifierEntryPoint.geometry.rawValue:
        return .geometry
    case SCNShaderModifierEntryPoint.lightingModel.rawValue:
        return .lightingModel
    case SCNShaderModifierEntryPoint.fragment.rawValue:
        return .fragment
    default:
        return nil
    }
}

private func scnCopyShaderModifier(
    _ shaderModifiers: [SCNShaderModifierEntryPoint: String]?,
    entryPoint: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let entryPoint else { return nil }
    let rawValue = String(cString: entryPoint)
    guard scnShaderModifierEntryPoint(from: rawValue) != nil else { return nil }
    let value = shaderModifiers?.first { $0.key.rawValue == rawValue }?.value
    return scnDup(value)
}

private func scnUpdatedShaderModifiers(
    _ shaderModifiers: [SCNShaderModifierEntryPoint: String]?,
    entryPoint: UnsafePointer<CChar>?,
    shaderModifier: UnsafePointer<CChar>?
) -> [SCNShaderModifierEntryPoint: String]? {
    guard let entryPoint else { return shaderModifiers }
    let rawValue = String(cString: entryPoint)
    guard let key = scnShaderModifierEntryPoint(from: rawValue) else { return shaderModifiers }
    var updated = shaderModifiers ?? [:]
    updated = Dictionary(uniqueKeysWithValues: updated.filter { $0.key.rawValue != rawValue })
    if let shaderModifier {
        updated[key] = String(cString: shaderModifier)
    }
    return updated.isEmpty ? nil : updated
}

private final class ProgramBindingStore: NSObject {
    var bindings: [String: ProgramBufferBindingBox] = [:]
}

private func programBindingStore(for program: SCNProgram) -> ProgramBindingStore {
    if let store = objc_getAssociatedObject(program, &programBindingStoreAssociationKey) as? ProgramBindingStore {
        return store
    }
    let store = ProgramBindingStore()
    objc_setAssociatedObject(program, &programBindingStoreAssociationKey, store, .OBJC_ASSOCIATION_RETAIN_NONATOMIC)
    return store
}

private final class ProgramDelegateBox: NSObject, SCNProgramDelegate {
    let context: UnsafeMutableRawPointer?
    let releaseContext: ScnReleaseContextCallback?
    let handleError: ProgramErrorCallback

    init(
        context: UnsafeMutableRawPointer?,
        releaseContext: ScnReleaseContextCallback?,
        handleError: @escaping ProgramErrorCallback
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.handleError = handleError
    }

    deinit {
        releaseContext?(context)
    }

    func program(_ program: SCNProgram, handleError error: Error) {
        handleError(context, scnDup(error.localizedDescription))
    }
}

private final class ProgramBufferBindingBox {
    let context: UnsafeMutableRawPointer?
    let releaseContext: ScnReleaseContextCallback?
    let callback: ProgramBufferBindingCallback

    init(
        context: UnsafeMutableRawPointer?,
        releaseContext: ScnReleaseContextCallback?,
        callback: @escaping ProgramBufferBindingCallback
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.callback = callback
    }

    deinit {
        releaseContext?(context)
    }

    func invoke(bufferStream: any SCNBufferStream) {
        callback(context, Unmanaged.passUnretained(bufferStream as AnyObject).toOpaque())
    }
}

private final class ProgramTestBufferStream: NSObject, SCNBufferStream {
    private(set) var storage = Data()

    func writeBytes(_ bytes: UnsafeRawPointer, count: Int) {
        storage.append(bytes.assumingMemoryBound(to: UInt8.self), count: count)
    }
}

@_cdecl("scn_program_delegate_new")
public func scn_program_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: ScnReleaseContextCallback?,
    _ handleError: @escaping ProgramErrorCallback
) -> UnsafeMutableRawPointer? {
    scnRetain(ProgramDelegateBox(
        context: context,
        releaseContext: releaseContext,
        handleError: handleError
    ))
}

@_cdecl("scn_program_buffer_binding_new")
public func scn_program_buffer_binding_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: ScnReleaseContextCallback?,
    _ callback: @escaping ProgramBufferBindingCallback
) -> UnsafeMutableRawPointer? {
    scnRetain(ProgramBufferBindingBox(
        context: context,
        releaseContext: releaseContext,
        callback: callback
    ) as AnyObject)
}

@_cdecl("scn_program_new")
public func scn_program_new() -> UnsafeMutableRawPointer? {
    scnRetain(SCNProgram())
}

@_cdecl("scn_program_copy_vertex_shader")
public func scn_program_copy_vertex_shader(_ programHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return nil }
    return scnDup(program.vertexShader)
}

@_cdecl("scn_program_set_vertex_shader")
public func scn_program_set_vertex_shader(_ programHandle: UnsafeMutableRawPointer?, _ vertexShader: UnsafePointer<CChar>?) {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return }
    program.vertexShader = vertexShader.map { String(cString: $0) }
}

@_cdecl("scn_program_copy_fragment_shader")
public func scn_program_copy_fragment_shader(_ programHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return nil }
    return scnDup(program.fragmentShader)
}

@_cdecl("scn_program_set_fragment_shader")
public func scn_program_set_fragment_shader(_ programHandle: UnsafeMutableRawPointer?, _ fragmentShader: UnsafePointer<CChar>?) {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return }
    program.fragmentShader = fragmentShader.map { String(cString: $0) }
}

@_cdecl("scn_program_copy_geometry_shader")
public func scn_program_copy_geometry_shader(_ programHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return nil }
    return scnDup(program.geometryShader)
}

@_cdecl("scn_program_set_geometry_shader")
public func scn_program_set_geometry_shader(_ programHandle: UnsafeMutableRawPointer?, _ geometryShader: UnsafePointer<CChar>?) {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return }
    program.geometryShader = geometryShader.map { String(cString: $0) }
}

@_cdecl("scn_program_copy_tessellation_control_shader")
public func scn_program_copy_tessellation_control_shader(_ programHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return nil }
    return scnDup(program.tessellationControlShader)
}

@_cdecl("scn_program_set_tessellation_control_shader")
public func scn_program_set_tessellation_control_shader(_ programHandle: UnsafeMutableRawPointer?, _ tessellationControlShader: UnsafePointer<CChar>?) {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return }
    program.tessellationControlShader = tessellationControlShader.map { String(cString: $0) }
}

@_cdecl("scn_program_copy_tessellation_evaluation_shader")
public func scn_program_copy_tessellation_evaluation_shader(_ programHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return nil }
    return scnDup(program.tessellationEvaluationShader)
}

@_cdecl("scn_program_set_tessellation_evaluation_shader")
public func scn_program_set_tessellation_evaluation_shader(_ programHandle: UnsafeMutableRawPointer?, _ tessellationEvaluationShader: UnsafePointer<CChar>?) {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return }
    program.tessellationEvaluationShader = tessellationEvaluationShader.map { String(cString: $0) }
}

@_cdecl("scn_program_copy_vertex_function_name")
public func scn_program_copy_vertex_function_name(_ programHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return nil }
    return scnDup(program.vertexFunctionName)
}

@_cdecl("scn_program_set_vertex_function_name")
public func scn_program_set_vertex_function_name(_ programHandle: UnsafeMutableRawPointer?, _ vertexFunctionName: UnsafePointer<CChar>?) {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return }
    program.vertexFunctionName = vertexFunctionName.map { String(cString: $0) }
}

@_cdecl("scn_program_copy_fragment_function_name")
public func scn_program_copy_fragment_function_name(_ programHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return nil }
    return scnDup(program.fragmentFunctionName)
}

@_cdecl("scn_program_set_fragment_function_name")
public func scn_program_set_fragment_function_name(_ programHandle: UnsafeMutableRawPointer?, _ fragmentFunctionName: UnsafePointer<CChar>?) {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return }
    program.fragmentFunctionName = fragmentFunctionName.map { String(cString: $0) }
}

@_cdecl("scn_program_get_opaque")
public func scn_program_get_opaque(_ programHandle: UnsafeMutableRawPointer?) -> Bool {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return false }
    return program.isOpaque
}

@_cdecl("scn_program_set_opaque")
public func scn_program_set_opaque(_ programHandle: UnsafeMutableRawPointer?, _ opaque: Bool) {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return }
    program.isOpaque = opaque
}

@_cdecl("scn_program_set_semantic")
public func scn_program_set_semantic(
    _ programHandle: UnsafeMutableRawPointer?,
    _ semantic: UnsafePointer<CChar>?,
    _ symbol: UnsafePointer<CChar>?,
    _ mappingChannel: Int,
    _ hasMappingChannel: Bool
) {
    guard let program: SCNProgram = scnBorrow(programHandle), let symbol else { return }
    let semantic = semantic.map { String(cString: $0) }
    let options: [String: Any]? = hasMappingChannel ? [SCNProgramMappingChannelKey: NSNumber(value: mappingChannel)] : nil
    program.setSemantic(semantic, forSymbol: String(cString: symbol), options: options)
}

@_cdecl("scn_program_copy_semantic_for_symbol")
public func scn_program_copy_semantic_for_symbol(_ programHandle: UnsafeMutableRawPointer?, _ symbol: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let program: SCNProgram = scnBorrow(programHandle), let symbol else { return nil }
    return scnDup(program.semantic(forSymbol: String(cString: symbol)))
}

@_cdecl("scn_program_set_delegate")
public func scn_program_set_delegate(_ programHandle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return }
    program.delegate = scnBorrow(delegateHandle)
}

@_cdecl("scn_program_set_buffer_binding")
public func scn_program_set_buffer_binding(_ programHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?, _ frequency: Int32, _ bindingHandle: UnsafeMutableRawPointer?) {
    guard let program: SCNProgram = scnBorrow(programHandle), let name else { return }
    let frequency = SCNBufferFrequency(rawValue: Int(frequency)) ?? .perFrame
    let bindingName = String(cString: name)
    let store = programBindingStore(for: program)

    guard let binding: ProgramBufferBindingBox = scnBorrow(bindingHandle) else {
        store.bindings.removeValue(forKey: bindingName)
        program.handleBinding(ofBufferNamed: bindingName, frequency: frequency) { _, _, _, _ in }
        return
    }

    store.bindings[bindingName] = binding
    program.handleBinding(ofBufferNamed: bindingName, frequency: frequency) { buffer, _, _, _ in
        binding.invoke(bufferStream: buffer)
    }
}

@_cdecl("scn_material_get_program")
public func scn_material_get_program(_ materialHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let material: SCNMaterial = scnBorrow(materialHandle),
          let program = material.program else { return nil }
    return scnRetain(program)
}

@_cdecl("scn_material_set_program")
public func scn_material_set_program(_ materialHandle: UnsafeMutableRawPointer?, _ programHandle: UnsafeMutableRawPointer?) {
    guard let material: SCNMaterial = scnBorrow(materialHandle) else { return }
    let program: SCNProgram? = scnBorrow(programHandle)
    material.program = program
}

@_cdecl("scn_material_copy_shader_modifier")
public func scn_material_copy_shader_modifier(_ materialHandle: UnsafeMutableRawPointer?, _ entryPoint: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let material: SCNMaterial = scnBorrow(materialHandle) else { return nil }
    return scnCopyShaderModifier(material.shaderModifiers, entryPoint: entryPoint)
}

@_cdecl("scn_material_set_shader_modifier")
public func scn_material_set_shader_modifier(_ materialHandle: UnsafeMutableRawPointer?, _ entryPoint: UnsafePointer<CChar>?, _ shaderModifier: UnsafePointer<CChar>?) {
    guard let material: SCNMaterial = scnBorrow(materialHandle) else { return }
    material.shaderModifiers = scnUpdatedShaderModifiers(
        material.shaderModifiers,
        entryPoint: entryPoint,
        shaderModifier: shaderModifier
    )
}

@_cdecl("scn_geometry_get_program")
public func scn_geometry_get_program(_ geometryHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle),
          let program = geometry.program else { return nil }
    return scnRetain(program)
}

@_cdecl("scn_geometry_set_program")
public func scn_geometry_set_program(_ geometryHandle: UnsafeMutableRawPointer?, _ programHandle: UnsafeMutableRawPointer?) {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle) else { return }
    let program: SCNProgram? = scnBorrow(programHandle)
    geometry.program = program
}

@_cdecl("scn_geometry_copy_shader_modifier")
public func scn_geometry_copy_shader_modifier(_ geometryHandle: UnsafeMutableRawPointer?, _ entryPoint: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle) else { return nil }
    return scnCopyShaderModifier(geometry.shaderModifiers, entryPoint: entryPoint)
}

@_cdecl("scn_geometry_set_shader_modifier")
public func scn_geometry_set_shader_modifier(_ geometryHandle: UnsafeMutableRawPointer?, _ entryPoint: UnsafePointer<CChar>?, _ shaderModifier: UnsafePointer<CChar>?) {
    guard let geometry: SCNGeometry = scnBorrow(geometryHandle) else { return }
    geometry.shaderModifiers = scnUpdatedShaderModifiers(
        geometry.shaderModifiers,
        entryPoint: entryPoint,
        shaderModifier: shaderModifier
    )
}

@_cdecl("scn_buffer_stream_write_bytes")
public func scn_buffer_stream_write_bytes(_ bufferStreamHandle: UnsafeMutableRawPointer?, _ bytes: UnsafeRawPointer?, _ length: Int) {
    guard let bytes else { return }
    guard let bufferStreamHandle else { return }
    let object = Unmanaged<AnyObject>.fromOpaque(bufferStreamHandle).takeUnretainedValue()
    guard let bufferStream = object as? SCNBufferStream else { return }
    bufferStream.writeBytes(bytes, count: length)
}

@_cdecl("scn_program_test_invoke_delegate_handle_error")
public func scn_program_test_invoke_delegate_handle_error(_ programHandle: UnsafeMutableRawPointer?, _ message: UnsafePointer<CChar>?) {
    guard let program: SCNProgram = scnBorrow(programHandle),
          let delegate = program.delegate,
          let message
    else { return }
    let error = NSError(domain: "scenekit-rs-tests", code: -1, userInfo: [NSLocalizedDescriptionKey: String(cString: message)])
    delegate.program?(program, handleError: error)
}

@_cdecl("scn_program_test_invoke_buffer_binding")
public func scn_program_test_invoke_buffer_binding(_ programHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) -> Int {
    guard let program: SCNProgram = scnBorrow(programHandle), let name else { return -1 }
    let bindingName = String(cString: name)
    let store = programBindingStore(for: program)
    guard let binding = store.bindings[bindingName] else { return -1 }
    let bufferStream = ProgramTestBufferStream()
    binding.invoke(bufferStream: bufferStream)
    return bufferStream.storage.count
}
