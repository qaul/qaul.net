part of 'abstract_rpc_module_translator.dart';

class UsersTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.USERS;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(
      List<int> data, Reader reader) async {
    final message = Users.fromBuffer(data);
    switch (message.whichMessage()) {
      case Users_Message.userList:
        final users = message
            .ensureUserList()
            .user
            .map((u) => User(
                  name: u.name,
                  id: Uint8List.fromList(u.id),
                  conversationId: Uint8List.fromList(u.groupId),
                  keyBase58: u.keyBase58,
                  isBlocked: u.blocked,
                  isVerified: u.verified,
                ))
            .toList();

        return RpcTranslatorResponse(type, users);
      case Users_Message.securityNumberResponse:
        final res = message.ensureSecurityNumberResponse();
        final secNo = SecurityNumber(
          userId: Uint8List.fromList(res.userId),
          securityHash: Uint8List.fromList(res.securityHash),
          securityNumberBlocks: res.securityNumberBlocks,
        );
        return RpcTranslatorResponse(type, secNo);
      case Users_Message.userUpdate:
        final userEntry = message.ensureUserUpdate();
        final user = User(
          name: userEntry.name,
          id: Uint8List.fromList(userEntry.id),
          conversationId: Uint8List.fromList(userEntry.groupId),
          keyBase58: userEntry.keyBase58,
          isBlocked: userEntry.blocked,
          isVerified: userEntry.verified,
          status: _mapFrom(userEntry.connectivity),
        );
        return RpcTranslatorResponse(type, user);
      default:
        return super.decodeMessageBytes(data, reader);
    }
  }

  ConnectionStatus _mapFrom(Connectivity c) {
    if (c == Connectivity.Online) return ConnectionStatus.online;
    if (c == Connectivity.Reachable) return ConnectionStatus.reachable;
    return ConnectionStatus.offline;
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Reader reader) async {
    if (res.module != type) return;
    final provider = reader(usersProvider.notifier);
    if (res.data is List<User>) {
      for (final user in res.data) {
        provider.contains(user) ? provider.update(user) : provider.add(user);
      }
    } else if (res.data is User) {
      if (provider.contains(res.data)) provider.update(res.data);
    }
    if (res.data is SecurityNumber) {
      reader(currentSecurityNoProvider.notifier).state = res.data;
    }
  }
}
