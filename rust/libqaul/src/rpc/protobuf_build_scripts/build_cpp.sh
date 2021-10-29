#!/usr/bin/env bash

# create protobuf code for the programming language C++

protoc \
    --cpp_out=../protobuf_generated/cpp \
    \
    --proto_path=../.. \
    \
    rpc/qaul_rpc.proto \
    connections/connections.proto \
    node/node.proto \
    node/user_accounts.proto \
    router/users.proto \
    router/router.proto \
    services/feed/feed.proto \
    \
    connections/ble/manager/ble.proto\
