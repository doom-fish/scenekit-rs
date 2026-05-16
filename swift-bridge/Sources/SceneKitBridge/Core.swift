import AppKit
import CoreGraphics
import Foundation
import Metal
import ModelIO
import QuartzCore
import SceneKit

@inline(__always)
func scnRetain(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func scnReleaseHandle(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@inline(__always)
func scnBorrow<T>(_ handle: UnsafeMutableRawPointer?) -> T? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? T
}

@inline(__always)
func scnDup(_ value: String?) -> UnsafeMutablePointer<CChar>? {
    guard let value else { return nil }
    return strdup(value)
}

func scnColor(from color: Any?) -> NSColor? {
    guard let color = color as? NSColor else { return nil }
    return color.usingColorSpace(.sRGB) ?? color
}

func scnMakeColor(r: Float, g: Float, b: Float, a: Float) -> NSColor {
    NSColor(srgbRed: CGFloat(r), green: CGFloat(g), blue: CGFloat(b), alpha: CGFloat(a))
}

func scnWriteColor(_ color: Any?, outRGBA: UnsafeMutableRawPointer?) -> Bool {
    guard let outRGBA, let color = scnColor(from: color) else { return false }
    var red: CGFloat = 0
    var green: CGFloat = 0
    var blue: CGFloat = 0
    var alpha: CGFloat = 0
    color.getRed(&red, green: &green, blue: &blue, alpha: &alpha)
    let out = outRGBA.assumingMemoryBound(to: Float.self)
    out[0] = Float(red)
    out[1] = Float(green)
    out[2] = Float(blue)
    out[3] = Float(alpha)
    return true
}

func scnWriteVector3(_ vector: SCNVector3, out: UnsafeMutableRawPointer?) -> Bool {
    guard let out else { return false }
    let ptr = out.assumingMemoryBound(to: Float.self)
    ptr[0] = Float(vector.x)
    ptr[1] = Float(vector.y)
    ptr[2] = Float(vector.z)
    return true
}

func scnWriteVector4(_ vector: SCNVector4, out: UnsafeMutableRawPointer?) -> Bool {
    guard let out else { return false }
    let ptr = out.assumingMemoryBound(to: Float.self)
    ptr[0] = Float(vector.x)
    ptr[1] = Float(vector.y)
    ptr[2] = Float(vector.z)
    ptr[3] = Float(vector.w)
    return true
}

func scnWriteMatrix4(_ matrix: SCNMatrix4, out: UnsafeMutableRawPointer?) -> Bool {
    guard let out else { return false }
    let ptr = out.assumingMemoryBound(to: Float.self)
    ptr[0] = Float(matrix.m11)
    ptr[1] = Float(matrix.m12)
    ptr[2] = Float(matrix.m13)
    ptr[3] = Float(matrix.m14)
    ptr[4] = Float(matrix.m21)
    ptr[5] = Float(matrix.m22)
    ptr[6] = Float(matrix.m23)
    ptr[7] = Float(matrix.m24)
    ptr[8] = Float(matrix.m31)
    ptr[9] = Float(matrix.m32)
    ptr[10] = Float(matrix.m33)
    ptr[11] = Float(matrix.m34)
    ptr[12] = Float(matrix.m41)
    ptr[13] = Float(matrix.m42)
    ptr[14] = Float(matrix.m43)
    ptr[15] = Float(matrix.m44)
    return true
}

func scnReadVector3(_ raw: UnsafeMutableRawPointer?) -> SCNVector3? {
    guard let raw else { return nil }
    let ptr = raw.assumingMemoryBound(to: Float.self)
    return SCNVector3(x: CGFloat(ptr[0]), y: CGFloat(ptr[1]), z: CGFloat(ptr[2]))
}

func scnReadVector4(_ raw: UnsafeMutableRawPointer?) -> SCNVector4? {
    guard let raw else { return nil }
    let ptr = raw.assumingMemoryBound(to: Float.self)
    return SCNVector4(x: CGFloat(ptr[0]), y: CGFloat(ptr[1]), z: CGFloat(ptr[2]), w: CGFloat(ptr[3]))
}

func scnReadMatrix4(_ raw: UnsafeMutableRawPointer?) -> SCNMatrix4? {
    guard let raw else { return nil }
    let ptr = raw.assumingMemoryBound(to: Float.self)
    return SCNMatrix4(
        m11: CGFloat(ptr[0]), m12: CGFloat(ptr[1]), m13: CGFloat(ptr[2]), m14: CGFloat(ptr[3]),
        m21: CGFloat(ptr[4]), m22: CGFloat(ptr[5]), m23: CGFloat(ptr[6]), m24: CGFloat(ptr[7]),
        m31: CGFloat(ptr[8]), m32: CGFloat(ptr[9]), m33: CGFloat(ptr[10]), m34: CGFloat(ptr[11]),
        m41: CGFloat(ptr[12]), m42: CGFloat(ptr[13]), m43: CGFloat(ptr[14]), m44: CGFloat(ptr[15])
    )
}

func scnBorrowCGImage(_ handle: UnsafeMutableRawPointer?) -> CGImage? {
    guard let handle else { return nil }
    return Unmanaged<CGImage>.fromOpaque(handle).takeUnretainedValue()
}

func scnBorrowMDLMesh(_ handle: UnsafeMutableRawPointer?) -> MDLMesh? {
    guard let handle else { return nil }
    return Unmanaged<MDLMesh>.fromOpaque(handle).takeUnretainedValue()
}

func scnLoadAction(_ rawValue: Int32) -> MTLLoadAction {
    switch rawValue {
    case 0: return .dontCare
    case 1: return .load
    default: return .clear
    }
}

func scnStoreAction(_ rawValue: Int32) -> MTLStoreAction {
    switch rawValue {
    case 0: return .dontCare
    case 2: return .multisampleResolve
    default: return .store
    }
}

@_cdecl("scn_release")
public func scn_release(_ handle: UnsafeMutableRawPointer?) {
    scnReleaseHandle(handle)
}
