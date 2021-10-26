// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// protobuf handling of protobuf's node RPC messages
///
/// serializes and deserializes protobuf's node messages

import 'package:flutter/foundation.dart';
import 'package:qaul_rpc/src/generated/node/node.pb.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'protobuf.dart';

class RpcNode {
  /// decode a binary node protobuf message and process it
  decodeReceivedMessage(List<int> bytes) {
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
        debugPrint('UNHANDLED RpcNode protobuf message received');
        break;
    }
  }

  /// encode and send a message
  Future<void> encodeAndSendMessage(Node message) async {
    // send message via qaul RPC
    Rpc rpc = Rpc();
    await rpc.encodeAndSendMessage(Modules.NODE, message.writeToBuffer());
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
