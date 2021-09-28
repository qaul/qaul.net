// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// deserialize received protobuf rpc packages and provide them to the modules.
/// serialize protobuf packages and send them to libqaul.

import 'dart:ffi';
import 'dart:typed_data';
import 'package:convert/convert.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'node.dart';
import '../libqaul/libqaul.dart';

// import pre-generated protobuf file
import 'protobuf_generated/rpc/qaul_rpc.pb.dart';

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
        print('NODE message received');
        RpcNode rpcNode = RpcNode();
        rpcNode.decodeReceivedMessage(message.data);
        break;
      case Modules.USERACCOUNTS:
        print('USERACCOUNTS message received');
        break;
      case Modules.FEED:
        print('FEED message received');
        break;
      case Modules.ROUTER:
        print('ROUTER message received');
        break;
      default:
        print('UNHANDLED protobuf message received');
        break;
    }
  }

  /// encode and send a message
  Future<void> encodeAndSendMessage(Modules module, Uint8List data) async {
    final data_length = data.length;
    print("encodeAndSendMessage: module $module, bytes $data_length");

    // create message
    QaulRpc message = QaulRpc();
    message.module = module;
    message.data = data;

    // encode it
    Uint8List message_encoded = message.writeToBuffer();
    final message_encoded_length = message_encoded.length;
    print("encodeAndSendMessage final length: $message_encoded_length");
    final message_hex = hex.encode(message_encoded);
    print("message to send: $message_hex");

    // send it
    final libqaul = container.read (libqaulProvider);
    await libqaul.sendRpc(message_encoded);
  }
}
