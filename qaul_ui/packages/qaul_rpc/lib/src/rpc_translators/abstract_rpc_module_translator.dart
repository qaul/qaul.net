import 'dart:typed_data';

import 'package:fast_base58/fast_base58.dart';
import 'package:logging/logging.dart';
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
import '../generated/services/filesharing/filesharing_rpc.pb.dart';
import '../generated/services/group/group_rpc.pb.dart';
import '../models/models.dart';

part 'ble_translator.dart';

part 'chat_translator.dart';

part 'connection_translator.dart';

part 'debug_translator.dart';

part 'feed_translator.dart';

part 'filesharing_translator.dart';

part 'group_translator.dart';

part 'node_translator.dart';

part 'router_translator.dart';

part 'user_accounts_translator.dart';

part 'users_translator.dart';

class UnhandledRpcMessageException implements Exception {
  final String message;

  final String? source;

  const UnhandledRpcMessageException([this.message = "", this.source]);

  @override
  String toString() {
    String report = "UnhandledRpcMessageException";
    if (message.isNotEmpty) report = "$report: $message";
    Object? source = this.source;
    if (source != null) report = '$report, at $source';
    return report;
  }
}

abstract class RpcModuleTranslator {
  final _log = Logger('RpcModuleTranslator');

  @protected
  Modules get type;

  @protected
  @mustCallSuper
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    _log.severe(
      'Received libqaul message from module "$type" which could not be translated',
      UnhandledRpcMessageException(type.toString()),
      StackTrace.current,
    );
    return null;
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
