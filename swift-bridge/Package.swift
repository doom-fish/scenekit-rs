// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "SceneKitBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "SceneKitBridge",
            type: .static,
            targets: ["SceneKitBridge"])
    ],
    targets: [
        .target(
            name: "SceneKitBridge",
            path: "Sources/SceneKitBridge",
            publicHeadersPath: "include")
    ]
)
