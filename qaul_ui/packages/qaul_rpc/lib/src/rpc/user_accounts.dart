import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/src/generated/node/user_accounts.pb.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'package:qaul_rpc/src/models/user.dart';
import 'package:qaul_rpc/src/rpc/rpc_module.dart';

import '../providers.dart';

class RpcUserAccounts extends RpcModule {
  RpcUserAccounts(this.reader) : super(reader);
  final Reader reader;

  @override
  Modules get type => Modules.USERACCOUNTS;

  /// decode a binary node protobuf message and process it
  @override
  Future<void> decodeReceivedMessage(List<int> bytes) async {
    // decode bytes to message
    final message = UserAccounts.fromBuffer(bytes);

    // send message to the appropriate module
    debugPrint(
        '$runtimeType: ${message.whichMessage().toString()} message received');
    switch (message.whichMessage()) {
      case UserAccounts_Message.defaultUserAccount:
        final info = message.ensureDefaultUserAccount();
        final exists = info.userAccountExists;
        debugPrint('RpcUserAccounts default acc exists? $exists');

        if (exists) {
          _updateUserWithMyUserAccount(info.myUserAccount);
        } else {
          reader(defaultUserProvider).state = null;
        }
        break;
      case UserAccounts_Message.myUserAccount:
        final acc = message.ensureMyUserAccount();
        _updateUserWithMyUserAccount(acc);
        break;
      default:
        throw UnhandledRpcMessageException.value(
          message.whichMessage().toString(),
          runtimeType.toString(),
        );
    }
  }

  void _updateUserWithMyUserAccount(MyUserAccount account) {
    final user = User(
      name: account.name,
      idBase58: account.idBase58,
      id: account.id,
      key: account.key,
      keyType: account.keyType,
      keyBase58: account.keyBase58,
    );
    reader(defaultUserProvider).state = user;
  }

  Future<void> getDefaultUserAccount() async {
    UserAccounts message = UserAccounts(getDefaultUserAccount: true);
    await encodeAndSendMessage(message);
  }

  Future<void> createUserAccount(String name) async {
    final msg = UserAccounts(createUserAccount: CreateUserAccount(name: name));
    await encodeAndSendMessage(msg);
  }
}
