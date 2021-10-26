#!/usr/bin/env bash

protoc --dart_out=../qaul_ui/packages/qaul_rpc/lib/src/generated \
    --proto_path=../rust/libqaul/src \
    \
    rpc/qaul_rpc.proto \
    node/node.proto \
    node/user_accounts.proto \
    router/router.proto \
    services/feed/feed.proto \
