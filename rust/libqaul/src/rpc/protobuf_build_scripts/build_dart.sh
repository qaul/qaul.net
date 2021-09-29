#!/usr/bin/env bash

protoc --dart_out=../../../../../flutter/lib/rpc/protobuf_generated \
    --proto_path=../.. \
    \
    rpc/qaul_rpc.proto \
    node/node.proto \
    node/user_accounts.proto \
    router/router.proto \
    services/feed/feed.proto \
