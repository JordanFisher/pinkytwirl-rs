// swift-tools-version:5.5
import PackageDescription

let package = Package(
    name: "PinkyTwirl",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "PinkyTwirl",
            targets: ["PinkyTwirl"]),
    ],
    targets: [
        .target(
            name: "PinkyTwirl",
            dependencies: ["PinkyTwirlCore"]),
        .binaryTarget(
            name: "PinkyTwirlCore",
            path: "PinkyTwirl.xcframework")
    ]
)