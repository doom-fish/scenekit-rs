import CoreGraphics
import Foundation
import SceneKit

final class SceneHitTestResultsBox: NSObject {
    let results: [SCNHitTestResult]

    init(results: [SCNHitTestResult]) {
        self.results = results
    }
}

func scnBorrowHitTestResultsBox(_ handle: UnsafeMutableRawPointer?) -> SceneHitTestResultsBox? {
    scnBorrow(handle)
}

@_cdecl("scn_view_hit_test")
public func scn_view_hit_test(
    _ viewHandle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double
) -> UnsafeMutableRawPointer? {
    guard let view: SCNView = scnBorrow(viewHandle) else { return nil }
    let results = view.hitTest(CGPoint(x: x, y: y), options: nil)
    return scnRetain(SceneHitTestResultsBox(results: results))
}

@_cdecl("scn_hit_test_results_count")
public func scn_hit_test_results_count(_ resultsHandle: UnsafeMutableRawPointer?) -> Int {
    guard let results = scnBorrowHitTestResultsBox(resultsHandle) else { return 0 }
    return results.results.count
}

@_cdecl("scn_hit_test_results_get")
public func scn_hit_test_results_get(
    _ resultsHandle: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let results = scnBorrowHitTestResultsBox(resultsHandle),
          index >= 0,
          index < results.results.count
    else { return nil }
    return scnRetain(results.results[index])
}

@_cdecl("scn_hit_test_result_node")
public func scn_hit_test_result_node(_ resultHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let result: SCNHitTestResult = scnBorrow(resultHandle) else { return nil }
    return scnRetain(result.node)
}

@_cdecl("scn_hit_test_result_world_coordinates")
public func scn_hit_test_result_world_coordinates(
    _ resultHandle: UnsafeMutableRawPointer?,
    _ outVector: UnsafeMutableRawPointer?
) -> Bool {
    guard let result: SCNHitTestResult = scnBorrow(resultHandle) else { return false }
    return scnWriteVector3(result.worldCoordinates, out: outVector)
}
