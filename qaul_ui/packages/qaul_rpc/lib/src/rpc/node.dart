// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// protobuf handling of protobuf's node RPC messages
///
/// serializes and deserializes protobuf's node messages

import 'package:flutter/foundation.dart';
import 'package:qaul_rpc/src/generated/node/node.pb.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'package:qaul_rpc/src/rpc/rpc_module.dart';
import 'package:riverpod/src/framework.dart';
import 'protobuf.dart';

class RpcNode extends RpcModule {
  RpcNode(Reader read) : super(read);

  @override
  Modules get type => Modules.NODE;

  @override
  Future<void> decodeReceivedMessage(List<int> bytes) async {
    // decode bytes to message
    Node message = Node.fromBuffer(bytes);

    // send message to the appropriate module
    switch (message.whichMessage()) {
      case Node_Message.info:
        debugPrint('RpcNode info message received');
        final nodeInformation = message.ensureInfo();
        final nodeId = nodeInformation.idBase58;
        debugPrint('RpcNode node id: $nodeId');
        break;
      default:
        throw UnhandledRpcMessageException.value(
          message.whichMessage().toString(),
          runtimeType.toString(),
        );
    }
  }

  /// send request node info message
  Future<void> getNodeInfo() async {
    // create message
    Node message = Node();
    message.getNodeInfo = true;

    // send message
    await encodeAndSendMessage(message);
  }
}
