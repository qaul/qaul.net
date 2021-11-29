import 'dart:typed_data';

import 'package:fast_base58/fast_base58.dart';
import 'package:meta/meta.dart';

import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'package:qaul_rpc/src/generated/connections/connections.pb.dart';
import 'package:qaul_rpc/src/generated/node/node.pb.dart';
import 'package:qaul_rpc/src/generated/node/user_accounts.pb.dart';
import 'package:qaul_rpc/src/generated/router/users.pb.dart';
import 'package:qaul_rpc/src/generated/router/router.pb.dart';
import 'package:qaul_rpc/src/generated/services/feed/feed.pb.dart';
import 'package:qaul_rpc/src/models/models.dart';

part 'connection_translator.dart';

part 'feed_translator.dart';

part 'node_translator.dart';

part 'user_accounts_translator.dart';

part 'users_translator.dart';

part 'router_translator.dart';

class UnhandledRpcMessageException implements Exception {
  UnhandledRpcMessageException._(this.message);

  String message;

  factory UnhandledRpcMessageException.value(String value, [String? type]) {
    var m = 'Message: $value';
    if (type != null) m += ', thrown by runtimeType: $type';
    return UnhandledRpcMessageException._(m);
  }
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
}
