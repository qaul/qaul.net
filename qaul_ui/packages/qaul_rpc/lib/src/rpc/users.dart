import 'dart:typed_data';

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
            id: Uint8List.fromList(u.id),
            status: _mapStatusFrom(u.connectivity),
            key: Uint8List.fromList(u.key),
            keyType: u.keyType,
            keyBase58: u.keyBase58,
            isBlocked: u.blocked,
            isVerified: u.verified,
          );

          if (provider.contains(domainUser)) {
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
    var entry = _baseUserEntryFrom(u);
    entry.verified = true;
    await encodeAndSendMessage(Users(userUpdate: entry));
  }

  Future<void> unverifyUser(User u) async {
    var entry = _baseUserEntryFrom(u);
    entry.verified = false;
    await encodeAndSendMessage(Users(userUpdate: entry));
  }

  Future<void> blockUser(User u) async {
    final entry = _baseUserEntryFrom(u);
    entry.blocked = true;
    await encodeAndSendMessage(Users(userUpdate: entry));
  }

  Future<void> unblockUser(User u) async {
    final entry = _baseUserEntryFrom(u);
    entry.blocked = false;
    await encodeAndSendMessage(Users(userUpdate: entry));
  }

  UserEntry _baseUserEntryFrom(User u) => UserEntry(
        name: u.name,
        idBase58: u.idBase58,
        id: u.id,
        key: u.key,
        keyType: u.keyType,
        keyBase58: u.keyBase58,
      );

  ConnectionStatus _mapStatusFrom(Connectivity c) {
    if (c == Connectivity.Online) return ConnectionStatus.online;
    if (c == Connectivity.Offline) return ConnectionStatus.offline;
    if (c == Connectivity.Reachable) return ConnectionStatus.reachable;
    throw ArgumentError.value(c, 'Connectivity', 'unmapped value provided');
  }
}
