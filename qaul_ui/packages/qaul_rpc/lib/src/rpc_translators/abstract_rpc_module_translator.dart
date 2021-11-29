import 'dart:typed_data';

import 'package:fast_base58/fast_base58.dart';
import 'package:meta/meta.dart';

import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'package:qaul_rpc/src/generated/router/users.pb.dart';
import 'package:qaul_rpc/src/generated/router/router.pb.dart';
import 'package:qaul_rpc/src/generated/services/feed/feed.pb.dart';
import 'package:qaul_rpc/src/models/feed_message.dart';
import 'package:qaul_rpc/src/models/user.dart';
import 'package:qaul_rpc/src/rpc/rpc_module.dart';

part 'feed_translator.dart';
part 'users_translator.dart';
part 'router_translator.dart';

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
