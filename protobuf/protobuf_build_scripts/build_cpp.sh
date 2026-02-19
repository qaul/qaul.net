#!/usr/bin/env bash

# create protobuf code for the programming language C++

# the proto files
PROTO_FILES=$(tr '\n' ' ' < files.txt)

protoc \
    --cpp_out=../protobuf_generated/cpp \
    \
    --proto_path=../../rust/qaul-proto/proto \
    \
    $PROTO_FILES