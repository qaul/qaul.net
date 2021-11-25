// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// deserialize received protobuf rpc packages and provide them to the modules.
/// serialize protobuf packages and send them to libqaul.

import 'dart:typed_data';
import 'package:convert/convert.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'node.dart';

class Rpc {
  Rpc(this.reader);
  final Reader reader;


  /// decode a binary protobuf message and send it to
  /// the responsible module.
  decodeReceivedMessage(List<int> bytes) {
    // decode bytes to message
    QaulRpc message = QaulRpc.fromBuffer(bytes);

    // send message to the appropriate module
    switch (message.module) {
      case Modules.NODE:
        debugPrint('NODE message received');
        RpcNode(reader).decodeReceivedMessage(message.data);
        break;
      case Modules.USERACCOUNTS:
        debugPrint('USERACCOUNTS message received');
        RpcUserAccounts(reader).decodeReceivedMessage(message.data);
        break;
      case Modules.FEED:
        debugPrint('FEED message received');
        RpcFeed(reader).decodeReceivedMessage(message.data);
        break;
      case Modules.ROUTER:
        debugPrint('ROUTER message received');
        RpcRouter(reader).decodeReceivedMessage(message.data);
        break;
      case Modules.USERS:
        debugPrint('USERS message received: IGNORED -> LibqaulWorker resp.');
        break;
      case Modules.CONNECTIONS:
        debugPrint('CONNECTIONS message received');
        RpcConnections(reader).decodeReceivedMessage(message.data);
        break;
      default:
        throw('UNHANDLED protobuf message received: $message from MODULE: ${message.module}');
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

    final user = reader(defaultUserProvider).state;
    if (user != null) message.userId = user.id;

    // encode it
    Uint8List messageEncoded = message.writeToBuffer();
    debugPrint("encodeAndSendMessage final length: ${messageEncoded.length}");
    debugPrint("message to send: ${hex.encode(messageEncoded)}");

    // send it
    final libqaul = reader(libqaulProvider);
    await libqaul.sendRpc(messageEncoded);
  }
}
