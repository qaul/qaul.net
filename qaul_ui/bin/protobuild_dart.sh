#!/usr/bin/env bash

protoc --dart_out=../packages/qaul_rpc/lib/src/generated \
    --proto_path=../../rust/libqaul/src \
    \
    rpc/qaul_rpc.proto \
    connections/connections.proto \
    node/node.proto \
    node/user_accounts.proto \
    router/router.proto \
    router/users.proto \
    services/feed/feed.proto \
