#!/usr/bin/env bash

# create protobuf code for the programming language swift

# the proto files
PROTO_FILES=$(tr '\n' ' ' < files.txt)

protoc \
    --swift_out=../generated/swift \
    \
    --proto_path=../../protobuf/proto_definitions \
    \
    $PROTO_FILES
