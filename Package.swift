// swift-tools-version:5.5
import PackageDescription

let package = Package(
    name: "PinkyTwirl",
    platforms: [
        .macOS(.v11)  // Specify minimum macOS version
    ],
    products: [
        .library(
            name: "PinkyTwirl",
            targets: ["PinkyTwirl"]),
    ],
    targets: [
        .target(
            name: "PinkyTwirl",
            dependencies: ["PinkyTwirlCore"],
            path: "Sources/PinkyTwirl"),
        .binaryTarget(
            name: "PinkyTwirlCore",
            path: "PinkyTwirl.xcframework")
    ]
)