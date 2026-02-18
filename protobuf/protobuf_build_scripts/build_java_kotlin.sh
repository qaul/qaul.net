#!/usr/bin/env bash

# the proto files
PROTO_FILES=$(tr '\n' ' ' < files.txt)

protoc \
    --java_out=../protobuf_generated/java \
    --kotlin_out=../protobuf_generated/kotlin \
    \
    --proto_path="../../" $PROTO_FILES
