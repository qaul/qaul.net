#!/usr/bin/env bash

protoc --dart_out=../protobuf_generated qaul_rpc.proto from_libqaul.proto to_libqaul.proto
