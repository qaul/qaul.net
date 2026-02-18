#!/usr/bin/env bash

# Dart/Flutter code of protobuf messages

# the proto files
PROTO_FILES=$(tr '\n' ' ' < files.txt)

# create in flutter UI directory
PROTO_OUT=../../../../../qaul_ui/packages/qaul_rpc/lib/src/generated
protoc --dart_out=$PROTO_OUT --proto_path=../.. $PROTO_FILES
