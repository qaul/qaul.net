#!/usr/bin/env bash

protoc \
    --java_out=../protobuf_generated/java \
    --kotlin_out=../protobuf_generated/kotlin \
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
