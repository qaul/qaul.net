#!/usr/bin/env bash

echo "Building aar files..."
cd ../android && ./gradlew assemble

cd ..

BUILD_OUT_PATH="android/libqaul/build/outputs/aar"
PACKAGE_PATH="qaul_ui/android/libs"

echo "Copying aar files from $BUILD_OUT_PATH to Flutter package at $PACKAGE_PATH..."
cp "$BUILD_OUT_PATH/libqaul-debug.aar" "$PACKAGE_PATH"
cp "$BUILD_OUT_PATH/libqaul-release.aar" "$PACKAGE_PATH"
