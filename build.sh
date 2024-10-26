#!/bin/bash
set -e

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/pinkytwirl-generated"

cargo build --release --manifest-path pinkytwirl-rs/Cargo.toml --target aarch64-apple-darwin

swiftc -L pinkytwirl-rs/target/aarch64-apple-darwin/release \
    -lpinkytwirl -import-objc-header bridging-header.h \
    pinkytwirl-generated/SwiftBridgeCore.swift \
    pinkytwirl-generated/pinkytwirl/pinkytwirl.swift \
    pinkytwirl-swift/Sources/swift/AppDelegate.swift \
    pinkytwirl-swift/Sources/swift/main.swift \
    -o pinkytwirl-swift/Contents/MacOS/pinkytwirl -framework Cocoa