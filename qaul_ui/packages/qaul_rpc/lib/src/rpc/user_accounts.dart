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
          print("************************************************************");
          print("true");
          print("************************************************************");
          final user = User(
            name: info.myUserAccount.name,
            idBase58: info.myUserAccount.idBase58,
          );

          print(user);
          reader(defaultUserProvider).state = user;
        } else {
          print("************************************************************");
          print("false");
          print("************************************************************");
          reader(defaultUserProvider).state = null;
        }
        break;
      default:
        throw UnhandledRpcMessageException.value(
          message.whichMessage().toString(),
          runtimeType.toString(),
        );
    }
  }

  Future<void> getDefaultUserAccount() async {
    // create message
    UserAccounts message = UserAccounts();
    message.getDefaultUserAccount = true;

    // send message
    await encodeAndSendMessage(message);
  }
}
