#!/usr/bin/env bash

# create protobuf code for the programming language swift

# the proto files
PROTO_FILES=$(tr '\n' ' ' < files.txt)

protoc \
    --swift_out=../protobuf_generated/swift \
    \
    --proto_path=../.. \
    \
    $PROTO_FILES
