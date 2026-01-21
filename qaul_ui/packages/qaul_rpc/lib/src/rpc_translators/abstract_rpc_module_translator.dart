import 'dart:async';
import 'dart:typed_data';

import 'package:collection/collection.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:hooks_riverpod/legacy.dart';
import 'package:logging/logging.dart';
import 'package:meta/meta.dart';

import '../generated/connections/ble/ble_rpc.pb.dart';
import '../generated/connections/connections.pb.dart';
import '../generated/node/node.pb.dart';
import '../generated/node/user_accounts.pb.dart';
import '../generated/router/users.pb.dart';
import '../generated/rpc/debug.pb.dart';
import '../generated/rpc/qaul_rpc.pb.dart';
import '../generated/services/chat/chat.pb.dart';
import '../generated/services/chat/chatfile_rpc.pb.dart';
import '../generated/services/dtn/dtn_rpc.pb.dart';
import '../generated/services/feed/feed.pb.dart';
import '../generated/services/group/group_rpc.pb.dart';
import '../internal/file_history.dart';
import '../models/models.dart';

part 'ble_translator.dart';

part 'chat_translator.dart';

part 'connection_translator.dart';

part 'debug_translator.dart';

part 'dtn_translator.dart';

part 'feed_translator.dart';

part 'chatfile_translator.dart';

part 'group_translator.dart';

part 'node_translator.dart';

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

  static final _translators = [
    BleTranslator(),
    ChatTranslator(),
    ConnectionTranslator(),
    DebugTranslator(),
    DTNTranslator(),
    FeedTranslator(),
    ChatFileTranslator(),
    GroupTranslator(),
    NodeTranslator(),
    UserAccountsTranslator(),
    UsersTranslator(),
  ];

  static RpcModuleTranslator translatorFactory(Modules module) {
    var t = _translators.firstWhereOrNull((element) => element.type == module);
    if (t == null) {
      throw UnimplementedError('Factory missing translator of type $module');
    }
    return t;
  }

  @protected
  Modules get type;

  @mustCallSuper
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Ref ref) async {
    _log.severe(
      'Received libqaul message from module "$type" which could not be translated',
      UnhandledRpcMessageException(type.toString()),
      StackTrace.current,
    );
    return null;
  }

  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    _log.severe(
      'unprocessed response from module "$type": ${res.data.runtimeType}',
      UnhandledRpcMessageException(res.toString()),
      StackTrace.current,
    );
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
