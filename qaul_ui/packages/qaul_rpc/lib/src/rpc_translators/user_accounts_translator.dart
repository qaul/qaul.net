part of 'abstract_rpc_module_translator.dart';

class UserAccountsTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.USERACCOUNTS;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Ref ref) async {
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
        return super.decodeMessageBytes(data, ref);
    }
    return null;
  }

  RpcTranslatorResponse _buildResponseWithUser(MyUserAccount account) {
    final user = User(
      name: account.name,
      id: Uint8List.fromList(account.id),
      keyBase58: account.keyBase58,
    );

    return RpcTranslatorResponse(type, user);
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type || res.data is! User) return;
    ref.read(defaultUserProvider.notifier).setUser(res.data);
  }
}
