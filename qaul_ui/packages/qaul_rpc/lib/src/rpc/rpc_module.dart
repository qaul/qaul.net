import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:protobuf/protobuf.dart';

import '../generated/rpc/qaul_rpc.pb.dart';
import 'protobuf.dart';

class UnhandledRpcMessageException implements Exception {
  UnhandledRpcMessageException._(this.message);

  String message;

  factory UnhandledRpcMessageException.value(String value, [String? type]) {
    var m = 'Message: $value';
    if (type != null) m += ', thrown by runtimeType: $type';
    return UnhandledRpcMessageException._(m);
  }
}

abstract class RpcModule {
  RpcModule(this.read);

  final Reader read;

  @protected
  Modules get type;

  /// decode a binary protobuf message and process it
  @protected
  Future<void> decodeReceivedMessage(List<int> bytes);

  /// encode and send a message
  @protected
  Future<void> encodeAndSendMessage(GeneratedMessage message) async {
    await Rpc(read).encodeAndSendMessage(type, message.writeToBuffer());
  }
}
