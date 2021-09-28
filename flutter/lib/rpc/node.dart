// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// protobuf handling of protobuf's node RPC messages
///
/// serializes and deserializes protobuf's node messages

import 'dart:typed_data';
import 'package:qaul_app/rpc/protobuf_generated/rpc/qaul_rpc.pb.dart';
import 'protobuf.dart';

// import pre-generated protobuf file
import 'protobuf_generated/node/node.pb.dart';
import 'protobuf_generated/rpc/qaul_rpc.pbenum.dart';

class RpcNode {
  /// decode a binary node protobuf message and process it
  decodeReceivedMessage(List<int> bytes) {
    // decode bytes to message
    Node message = Node.fromBuffer(bytes);

    // send message to the appropriate module
    switch (message.whichMessage()) {
      case Node_Message.info:
        print('RpcNode info message received');
        final nodeInformation = message.ensureInfo();
        final nodeId = nodeInformation.idBase58;
        print('RpcNode node id: $nodeId');
        break;
      default:
        print('UNHANDLED RpcNode protobuf message received');
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
