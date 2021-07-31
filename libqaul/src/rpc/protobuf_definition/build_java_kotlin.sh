#!/usr/bin/env bash

protoc --java_out=../protobuf_generated --kotlin_out=../protobuf_generated qaul_rpc.proto from_libqaul.proto to_libqaul.proto
