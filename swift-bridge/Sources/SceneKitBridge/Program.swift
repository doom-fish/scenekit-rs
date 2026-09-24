import Foundation
import Metal
import ObjectiveC
import SceneKit

public typealias ProgramErrorCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutablePointer<CChar>?) -> Void
public typealias ProgramBufferBindingCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void

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

private final class ProgramDelegateBox: NSObject, SCNProgramDelegate {
    let context: UnsafeMutableRawPointer
    let releaseContext: ScnReleaseContextCallback
    let handleError: ProgramErrorCallback

    init(
        context: UnsafeMutableRawPointer,
        releaseContext: @escaping ScnReleaseContextCallback,
        handleError: @escaping ProgramErrorCallback
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.handleError = handleError
    }

    deinit {
        releaseContext(context)
    }

    func program(_ program: SCNProgram, handleError error: Error) {
        handleError(context, scnDup(error.localizedDescription))
    }
}

private final class ProgramBufferBindingBox {
    let context: UnsafeMutableRawPointer
    let releaseContext: ScnReleaseContextCallback
    let callback: ProgramBufferBindingCallback

    init(
        context: UnsafeMutableRawPointer,
        releaseContext: @escaping ScnReleaseContextCallback,
        callback: @escaping ProgramBufferBindingCallback
    ) {
        self.context = context
        self.releaseContext = releaseContext
        self.callback = callback
    }

    deinit {
        releaseContext(context)
    }

    func invoke(bufferStream: ProgramBufferStreamBox) {
        callback(context, Unmanaged.passUnretained(bufferStream).toOpaque())
    }
}

private let scnBufferPoolSlack = 0x40100

private final class ProgramBufferStreamBox: NSObject, SCNBufferStream {
    private let stream: any SCNBufferStream
    private let name: String
    let requiredLength: Int?
    private let unknownLengthReason: String
    let maximumLength: Int
    private var wrote = false

    fileprivate init(stream: any SCNBufferStream, name: String, requirement: ProgramBufferRequirement, maximumLength: Int) {
        self.stream = stream
        self.name = name
        switch requirement {
        case .length(let length):
            requiredLength = length
            unknownLengthReason = ""
        case .unknown(let reason):
            requiredLength = nil
            unknownLengthReason = reason
        }
        self.maximumLength = maximumLength
    }

    func write(_ bytes: UnsafeRawPointer, length: Int, checked: Bool) -> String? {
        guard length > 0 else { return "buffer `\(name)` needs at least one byte" }
        guard length <= maximumLength else {
            return "\(length) bytes for buffer `\(name)` exceed the \(maximumLength) bytes the Metal device can allocate"
        }
        if checked {
            guard let requiredLength else {
                return "the size of buffer `\(name)` is unknown: \(unknownLengthReason)"
            }
            guard length >= requiredLength else {
                return "buffer `\(name)` needs \(requiredLength) bytes, got \(length)"
            }
        }
        stream.writeBytes(bytes, count: length)
        wrote = true
        return nil
    }

    func writeBytes(_ bytes: UnsafeRawPointer, count: Int) {
        _ = write(bytes, length: count, checked: true)
    }

    fileprivate func finish() {
        guard !wrote, let requiredLength, requiredLength > 0, requiredLength <= maximumLength else { return }
        let zeros = [UInt8](repeating: 0, count: requiredLength)
        zeros.withUnsafeBytes { buffer in
            guard let base = buffer.baseAddress else { return }
            stream.writeBytes(base, count: requiredLength)
        }
    }
}

private enum ProgramBufferRequirement {
    case length(Int)
    case unknown(String)
}

private enum ProgramBufferReflection {
    case lengths([Int])
    case failed(String)
}

private struct ProgramReflectionKey: Equatable {
    let library: ObjectIdentifier?
    let vertexFunctionName: String?
    let fragmentFunctionName: String?
}

private func scnVertexFormat(for type: MTLDataType) -> MTLVertexFormat? {
    switch type {
    case .float: return .float
    case .float2: return .float2
    case .float3: return .float3
    case .float4: return .float4
    case .half: return .half
    case .half2: return .half2
    case .half3: return .half3
    case .half4: return .half4
    case .int: return .int
    case .int2: return .int2
    case .int3: return .int3
    case .int4: return .int4
    case .uint: return .uint
    case .uint2: return .uint2
    case .uint3: return .uint3
    case .uint4: return .uint4
    case .short: return .short
    case .short2: return .short2
    case .short3: return .short3
    case .short4: return .short4
    case .ushort: return .ushort
    case .ushort2: return .ushort2
    case .ushort3: return .ushort3
    case .ushort4: return .ushort4
    case .char: return .char
    case .char2: return .char2
    case .char3: return .char3
    case .char4: return .char4
    case .uchar: return .uchar
    case .uchar2: return .uchar2
    case .uchar3: return .uchar3
    case .uchar4: return .uchar4
    default: return nil
    }
}

private func scnPipelineBufferLengths(
    named name: String,
    library: MTLLibrary,
    vertexFunctionName: String,
    fragmentFunctionName: String
) -> ProgramBufferReflection {
    guard let vertexFunction = library.makeFunction(name: vertexFunctionName) else {
        return .failed("the library has no function `\(vertexFunctionName)`")
    }
    guard let fragmentFunction = library.makeFunction(name: fragmentFunctionName) else {
        return .failed("the library has no function `\(fragmentFunctionName)`")
    }
    let descriptor = MTLRenderPipelineDescriptor()
    descriptor.vertexFunction = vertexFunction
    descriptor.fragmentFunction = fragmentFunction
    descriptor.colorAttachments[0].pixelFormat = .bgra8Unorm
    let attributes = (vertexFunction.vertexAttributes ?? []).filter { $0.isActive }
    if !attributes.isEmpty {
        let vertexDescriptor = MTLVertexDescriptor()
        var offset = 0
        for attribute in attributes {
            guard let format = scnVertexFormat(for: attribute.attributeType) else {
                return .failed("vertex attribute `\(attribute.name)` has a type the bridge cannot describe")
            }
            vertexDescriptor.attributes[attribute.attributeIndex].format = format
            vertexDescriptor.attributes[attribute.attributeIndex].offset = offset
            vertexDescriptor.attributes[attribute.attributeIndex].bufferIndex = 30
            offset += 16
        }
        vertexDescriptor.layouts[30].stride = offset
        descriptor.vertexDescriptor = vertexDescriptor
    }
    var reflection: MTLAutoreleasedRenderPipelineReflection?
    do {
        _ = try library.device.makeRenderPipelineState(descriptor: descriptor, options: [.argumentInfo, .bufferTypeInfo], reflection: &reflection)
    } catch {
        return .failed("Metal could not reflect the program: \(error.localizedDescription)")
    }
    guard let reflection else { return .failed("Metal returned no pipeline reflection") }
    let arguments = (reflection.vertexArguments ?? []) + (reflection.fragmentArguments ?? [])
    return .lengths(arguments.filter { $0.type == .buffer && $0.name == name }.map { $0.bufferDataSize })
}

private func scnProgramBufferRequirement(program: SCNProgram, name: String) -> ProgramBufferRequirement {
    guard let vertexFunctionName = program.vertexFunctionName,
          let fragmentFunctionName = program.fragmentFunctionName
    else { return .unknown("the program has no Metal vertex and fragment function names") }
    guard let library = program.library ?? MTLCreateSystemDefaultDevice()?.makeDefaultLibrary() else {
        return .unknown("the program has no Metal library")
    }
    var lengths: [Int] = []
    if #available(macOS 26.0, *) {
        for functionName in [vertexFunctionName, fragmentFunctionName] {
            guard let reflection = library.reflection(functionName: functionName) else {
                return .unknown("Metal could not reflect function `\(functionName)`")
            }
            for binding in reflection.bindings where binding.name == name {
                if let buffer = binding as? any MTLBufferBinding {
                    lengths.append(buffer.bufferDataSize)
                }
            }
        }
    } else {
        switch scnPipelineBufferLengths(named: name, library: library, vertexFunctionName: vertexFunctionName, fragmentFunctionName: fragmentFunctionName) {
        case .lengths(let reflected): lengths = reflected
        case .failed(let reason): return .unknown(reason)
        }
    }
    guard let length = lengths.max() else {
        return .unknown("the program's functions have no buffer argument named `\(name)`")
    }
    return .length(length)
}

private final class ProgramBufferRegistration {
    private weak var program: SCNProgram?
    private let name: String
    private let binding: ProgramBufferBindingBox?
    private let lock = NSLock()
    private var cachedKey: ProgramReflectionKey?
    private var cachedRequirement: ProgramBufferRequirement = .unknown("the program has not been reflected")

    init(program: SCNProgram, name: String, binding: ProgramBufferBindingBox?) {
        self.program = program
        self.name = name
        self.binding = binding
    }

    private func requirement(for program: SCNProgram) -> ProgramBufferRequirement {
        let key = ProgramReflectionKey(
            library: program.library.map { ObjectIdentifier($0) },
            vertexFunctionName: program.vertexFunctionName,
            fragmentFunctionName: program.fragmentFunctionName
        )
        lock.lock()
        defer { lock.unlock() }
        if cachedKey != key {
            cachedRequirement = scnProgramBufferRequirement(program: program, name: name)
            cachedKey = key
        }
        return cachedRequirement
    }

    func bind(stream: any SCNBufferStream) {
        guard let program else { return }
        let device = program.library?.device ?? MTLCreateSystemDefaultDevice()
        let (maximum, overflow) = (device?.maxBufferLength ?? 0).subtractingReportingOverflow(scnBufferPoolSlack)
        let box = ProgramBufferStreamBox(
            stream: stream,
            name: name,
            requirement: requirement(for: program),
            maximumLength: overflow ? 0 : max(maximum, 0)
        )
        binding?.invoke(bufferStream: box)
        box.finish()
    }
}

@_cdecl("scn_program_delegate_new")
public func scn_program_delegate_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: @escaping ScnReleaseContextCallback,
    _ handleError: @escaping ProgramErrorCallback
) -> UnsafeMutableRawPointer? {
    guard let context else { return nil }
    return scnRetain(ProgramDelegateBox(
        context: context,
        releaseContext: releaseContext,
        handleError: handleError
    ))
}

@_cdecl("scn_program_buffer_binding_new")
public func scn_program_buffer_binding_new(
    _ context: UnsafeMutableRawPointer?,
    _ releaseContext: @escaping ScnReleaseContextCallback,
    _ callback: @escaping ProgramBufferBindingCallback
) -> UnsafeMutableRawPointer? {
    guard let context else { return nil }
    return scnRetain(ProgramBufferBindingBox(
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
    let delegate: ProgramDelegateBox? = scnBorrow(delegateHandle)
    scnKeepDelegate(delegate, by: program)
    program.delegate = delegate
}

@_cdecl("scn_program_get_delegate")
public func scn_program_get_delegate(_ programHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let program: SCNProgram = scnBorrow(programHandle),
          let delegate = program.delegate as? ProgramDelegateBox
    else { return nil }
    return scnRetain(delegate)
}

@_cdecl("scn_program_set_library")
public func scn_program_set_library(_ programHandle: UnsafeMutableRawPointer?, _ libraryHandle: UnsafeMutableRawPointer?) {
    guard let program: SCNProgram = scnBorrow(programHandle) else { return }
    let library: MTLLibrary? = scnBorrow(libraryHandle)
    program.library = library
}

@_cdecl("scn_program_set_buffer_binding")
public func scn_program_set_buffer_binding(_ programHandle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?, _ frequency: Int32, _ bindingHandle: UnsafeMutableRawPointer?) {
    guard let program: SCNProgram = scnBorrow(programHandle), let name else { return }
    let frequency = SCNBufferFrequency(rawValue: Int(frequency)) ?? .perFrame
    let bindingName = String(cString: name)
    let binding: ProgramBufferBindingBox? = scnBorrow(bindingHandle)
    let registration = ProgramBufferRegistration(program: program, name: bindingName, binding: binding)
    program.handleBinding(ofBufferNamed: bindingName, frequency: frequency) { stream, _, _, _ in
        registration.bind(stream: stream)
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

@_cdecl("scn_buffer_stream_required_length")
public func scn_buffer_stream_required_length(_ bufferStreamHandle: UnsafeMutableRawPointer?, _ outLength: UnsafeMutablePointer<Int>?) -> Bool {
    guard let stream: ProgramBufferStreamBox = scnBorrow(bufferStreamHandle),
          let length = stream.requiredLength,
          let outLength
    else { return false }
    outLength.pointee = length
    return true
}

@_cdecl("scn_buffer_stream_maximum_length")
public func scn_buffer_stream_maximum_length(_ bufferStreamHandle: UnsafeMutableRawPointer?) -> Int {
    guard let stream: ProgramBufferStreamBox = scnBorrow(bufferStreamHandle) else { return 0 }
    return stream.maximumLength
}

@_cdecl("scn_buffer_stream_write_bytes")
public func scn_buffer_stream_write_bytes(
    _ bufferStreamHandle: UnsafeMutableRawPointer?,
    _ bytes: UnsafeRawPointer?,
    _ length: Int,
    _ checked: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    outError?.pointee = nil
    guard let stream: ProgramBufferStreamBox = scnBorrow(bufferStreamHandle), let bytes else {
        outError?.pointee = scnDup("missing buffer stream or bytes")
        return false
    }
    if let message = stream.write(bytes, length: length, checked: checked) {
        outError?.pointee = scnDup(message)
        return false
    }
    return true
}
