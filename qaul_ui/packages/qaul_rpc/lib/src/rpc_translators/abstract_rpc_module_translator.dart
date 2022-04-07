import 'dart:typed_data';

import 'package:fast_base58/fast_base58.dart';
import 'package:meta/meta.dart';

import '../generated/connections/ble/ble_rpc.pb.dart';
import '../generated/connections/connections.pb.dart';
import '../generated/node/node.pb.dart';
import '../generated/node/user_accounts.pb.dart';
import '../generated/router/router.pb.dart';
import '../generated/router/users.pb.dart';
import '../generated/rpc/debug.pb.dart';
import '../generated/rpc/qaul_rpc.pb.dart';
import '../generated/services/chat/chat.pb.dart';
import '../generated/services/feed/feed.pb.dart';
import '../models/models.dart';

part 'ble_translator.dart';

part 'chat_translator.dart';

part 'connection_translator.dart';

part 'debug_translator.dart';

part 'feed_translator.dart';

part 'node_translator.dart';

part 'router_translator.dart';

part 'user_accounts_translator.dart';

part 'users_translator.dart';

class UnhandledRpcMessageException implements Exception {
  UnhandledRpcMessageException._(this.message);

  String message;

  factory UnhandledRpcMessageException.value(String value, [String? type]) {
    var m = 'Message: $value';
    if (type != null) m += ', thrown by runtimeType: $type';
    return UnhandledRpcMessageException._(m);
  }

  @override
  String toString() => 'UnhandledRpcMessageException{message: $message}';
}

abstract class RpcModuleTranslator {
  @protected
  Modules get type;

  @protected
  @mustCallSuper
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) {
    throw UnhandledRpcMessageException.value(type.toString());
  }
}

class RpcTranslatorResponse {
  RpcTranslatorResponse(this.module, this.data);

  final Modules module;
  final dynamic data;

  @override
  String toString() {
    return 'RpcTranslatorResponse{module: $module, data: $data}';
  }
}
