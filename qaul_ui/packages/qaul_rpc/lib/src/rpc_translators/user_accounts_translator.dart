part of 'abstract_rpc_module_translator.dart';

class UserAccountsTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.USERACCOUNTS;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = UserAccounts.fromBuffer(data);
    switch (message.whichMessage()) {
      case UserAccounts_Message.defaultUserAccount:
        final info = message.ensureDefaultUserAccount();
        final exists = info.userAccountExists;
        if (exists) return _buildResponseWithUser(info.myUserAccount);
        break;
      case UserAccounts_Message.myUserAccount:
        final acc = message.ensureMyUserAccount();
        return _buildResponseWithUser(acc);
      default:
        return super.decodeMessageBytes(data);
    }
    return null;
  }

  RpcTranslatorResponse _buildResponseWithUser(MyUserAccount account) {
    final user = User(
      name: account.name,
      idBase58: account.idBase58,
      id: Uint8List.fromList(account.id),
      key: Uint8List.fromList(account.key),
      keyType: account.keyType,
      keyBase58: account.keyBase58,
    );

    return RpcTranslatorResponse(Modules.USERACCOUNTS, user);
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Reader reader) async {
    if (res.module != type || res.data is! User) return;
    reader(defaultUserProvider.state).state = res.data;
  }
}
