#!/usr/bin/env bash

# the proto files
PROTO_FILES=$(tr '\n' ' ' < files.txt)

protoc \
    --java_out=../generated/java \
    --kotlin_out=../generated/kotlin \
    \
    --proto_path="../../protobuf/proto_definitions" $PROTO_FILES
