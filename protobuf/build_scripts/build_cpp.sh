#!/usr/bin/env bash

# create protobuf code for the programming language C++

# the proto files
PROTO_FILES=$(tr '\n' ' ' < files.txt)

protoc \
    --cpp_out=../generated/cpp \
    \
    --proto_path=../../protobuf/proto_definitions \
    \
    $PROTO_FILES