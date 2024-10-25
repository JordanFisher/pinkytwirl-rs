#!/bin/bash

# Build Rust library for all required architectures
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create temporary directories for the framework
rm -rf PinkyTwirl.framework PinkyTwirl.xcframework
mkdir -p PinkyTwirl.framework/Versions/A/Headers
mkdir -p PinkyTwirl.framework/Versions/A/Resources

# Copy headers and libraries
cp target/release/libpinkytwirl.a PinkyTwirl.framework/Versions/A/PinkyTwirl
cp pinkytwirl.h PinkyTwirl.framework/Versions/A/Headers/
cp Info.plist PinkyTwirl.framework/Versions/A/Resources/

# Create symlinks
cd PinkyTwirl.framework/Versions
ln -s A Current
cd ..
ln -s Versions/Current/Headers .
ln -s Versions/Current/Resources .
ln -s Versions/Current/PinkyTwirl .
cd ..

# Create XCFramework
xcodebuild -create-xcframework \
  -framework PinkyTwirl.framework \
  -output PinkyTwirl.xcframework