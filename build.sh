#!/bin/bash
set -e

# Build the Rust code and generate the Swift bridge.
cargo build --release --manifest-path pinkytwirl-rs/Cargo.toml --target aarch64-apple-darwin

# Make sure the output directory exists.
mkdir -p pinkytwirl-swift/Contents/MacOS

# Compile the Swift code.
swiftc -L pinkytwirl-rs/target/aarch64-apple-darwin/release \
    -lpinkytwirl -import-objc-header bridging-header.h \
    pinkytwirl-generated/SwiftBridgeCore.swift \
    pinkytwirl-generated/pinkytwirl/pinkytwirl.swift \
    pinkytwirl-swift/Sources/swift/AppDelegate.swift \
    pinkytwirl-swift/Sources/swift/main.swift \
    -o pinkytwirl-swift/Contents/MacOS/pinkytwirl -framework Cocoa