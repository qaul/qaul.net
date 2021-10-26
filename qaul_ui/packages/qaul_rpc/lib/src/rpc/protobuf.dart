// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// deserialize received protobuf rpc packages and provide them to the modules.
/// serialize protobuf packages and send them to libqaul.

import 'dart:typed_data';
import 'package:convert/convert.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import '../qaul_rpc.dart';
import 'node.dart';

class Rpc {
  final container = ProviderContainer();

  /// decode a binary protobuf message and send it to
  /// the responsible module.
  decodeReceivedMessage(List<int> bytes) {
    // decode bytes to message
    QaulRpc message = QaulRpc.fromBuffer(bytes);

    // send message to the appropriate module
    switch (message.module) {
      case Modules.NODE:
        debugPrint('NODE message received');
        RpcNode rpcNode = RpcNode();
        rpcNode.decodeReceivedMessage(message.data);
        break;
      case Modules.USERACCOUNTS:
        debugPrint('USERACCOUNTS message received');
        break;
      case Modules.FEED:
        debugPrint('FEED message received');
        break;
      case Modules.ROUTER:
        debugPrint('ROUTER message received');
        break;
      default:
        debugPrint('UNHANDLED protobuf message received');
        break;
    }
  }

  /// encode and send a message
  Future<void> encodeAndSendMessage(Modules module, Uint8List data) async {
    final dataLength = data.length;
    debugPrint("encodeAndSendMessage: module $module, bytes $dataLength");

    // create message
    QaulRpc message = QaulRpc();
    message.module = module;
    message.data = data;

    // encode it
    Uint8List messageEncoded = message.writeToBuffer();
    debugPrint("encodeAndSendMessage final length: ${messageEncoded.length}");
    debugPrint("message to send: ${hex.encode(messageEncoded)}");

    // send it
    final libqaul = container.read(libqaulProvider);
    await libqaul.sendRpc(messageEncoded);
  }
}
