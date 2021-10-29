#!/usr/bin/env bash

# create protobuf code for the programming language swift

protoc \
    --swift_out=../protobuf_generated/swift \
    \
    --proto_path=../.. \
    \
    rpc/qaul_rpc.proto \
    node/node.proto \
    node/user_accounts.proto \
    router/users.proto \
    router/router.proto \
    services/feed/feed.proto \
    \
    connections/ble/manager/ble.proto\
