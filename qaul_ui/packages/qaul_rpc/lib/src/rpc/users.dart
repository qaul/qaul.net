import 'package:flutter/foundation.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pbenum.dart';
import 'package:qaul_rpc/src/generated/router/users.pb.dart';
import 'package:qaul_rpc/src/rpc/rpc_module.dart';
import 'package:riverpod/riverpod.dart';

import '../../qaul_rpc.dart';

class RpcUsers extends RpcModule {
  RpcUsers(Reader read) : super(read);

  @override
  Modules get type => Modules.USERS;

  @override
  Future<void> decodeReceivedMessage(List<int> bytes) async {
    Users message = Users.fromBuffer(bytes);

    switch (message.whichMessage()) {
      case Users_Message.userList:
        final users = message.ensureUserList().user;
        debugPrint('UsersRpc received Users: $users');

        final provider = read(usersProvider.notifier);

        for (final u in users) {
          final domainUser = User(
            name: u.name,
            idBase58: u.idBase58,
            id: u.id,
            status: _mapStatusFrom(u.connectivity),
            key: u.key,
            keyType: u.keyType,
            keyBase58: u.keyBase58,
            isBlocked: u.blocked,
            isVerified: u.verified,
          );

          if (provider.contains(u.idBase58)) {
            provider.update(domainUser);
            continue;
          }
          provider.add(domainUser);
        }

        break;
      default:
        throw UnhandledRpcMessageException.value(
          message.whichMessage().toString(),
          runtimeType.toString(),
        );
    }
  }

  Future<void> requestUsers() async =>
      await encodeAndSendMessage(Users(userRequest: UserRequest()));

  Future<void> verifyUser(User u) async {
    final msg = Users(userUpdate: UserEntry(
      name: u.name,
      idBase58: u.idBase58,
      id: u.id,
      key: u.key,
      keyType: u.keyType,
      keyBase58: u.keyBase58,
      verified: true,
    ));
    await encodeAndSendMessage(msg);
  }

  Future<void> blockUser(User u) async {
    final msg = Users(userUpdate: UserEntry(
      name: u.name,
      idBase58: u.idBase58,
      id: u.id,
      key: u.key,
      keyType: u.keyType,
      keyBase58: u.keyBase58,
      blocked: true,
    ));
    await encodeAndSendMessage(msg);
  }

  ConnectionStatus _mapStatusFrom(Connectivity c) {
    if (c == Connectivity.Online) return ConnectionStatus.online;
    if (c == Connectivity.Offline) return ConnectionStatus.offline;
    if (c == Connectivity.Reachable) return ConnectionStatus.reachable;
    throw ArgumentError.value(c, 'Connectivity', 'unmapped value provided');
  }
}
