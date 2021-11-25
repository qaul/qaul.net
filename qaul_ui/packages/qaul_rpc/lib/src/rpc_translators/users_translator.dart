part of 'abstract_rpc_module_translator.dart';


class UsersTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.USERS;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = Users.fromBuffer(data);
    switch (message.whichMessage()) {
      case Users_Message.userList:
        final users = message
            .ensureUserList()
            .user
            .map((u) => User(
          name: u.name,
          idBase58: u.idBase58,
          id: Uint8List.fromList(u.id),
          status: ConnectionStatus.offline,
          key: Uint8List.fromList(u.key),
          keyType: u.keyType,
          keyBase58: u.keyBase58,
          isBlocked: u.blocked,
          isVerified: u.verified,
        ))
            .toList();

        return RpcTranslatorResponse(type, users);
      default:
        return super.decodeMessageBytes(data);
    }
  }
}
