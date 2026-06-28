part of 'abstract_rpc_module_translator.dart';

class AccountManagementTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.ACCOUNT_MANAGEMENT;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(
      List<int> data, Ref ref) async {
    final message = AccountManagement.fromBuffer(data);
    switch (message.whichMessage()) {
      case AccountManagement_Message.exportAccountResponse:
        return RpcTranslatorResponse(
          type,
          message.ensureExportAccountResponse().path,
        );
      case AccountManagement_Message.restoreAccountResponse:
        final response = message.ensureRestoreAccountResponse();
        return RpcTranslatorResponse(
          type,
          RestoreAccountResult(
            userId: Uint8List.fromList(response.userId),
            userIdBase58: response.userIdBase58,
          ),
        );
      case AccountManagement_Message.ack:
        return RpcTranslatorResponse(type, true);
      case AccountManagement_Message.error:
        final error = message.ensureError();
        throw RpcRequestException(
          error.message,
          code: error.code,
          details: error.details,
        );
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }
}
