part of 'abstract_rpc_module_translator.dart';

class AuthenticationTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.AUTH;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(
      List<int> data, Ref ref) async {
    final message = AuthRpc.fromBuffer(data);
    switch (message.whichMessage()) {
      case AuthRpc_Message.usersResponse:
        final response = message.ensureUsersResponse();
        if (response.errorMessage.isNotEmpty) {
          throw RpcRequestException(response.errorMessage);
        }
        return RpcTranslatorResponse(
          type,
          response.users
              .map(
                (u) => LocalAccount(
                  username: u.username,
                  userId: Uint8List.fromList(u.userId),
                  salt: u.hasSalt() ? u.salt : null,
                  hasPassword: u.hasPassword,
                ),
              )
              .toList(),
        );
      case AuthRpc_Message.authChallenge:
        return RpcTranslatorResponse(type, message.ensureAuthChallenge());
      case AuthRpc_Message.authResult:
        final result = message.ensureAuthResult();
        if (!result.success) throw RpcRequestException(result.errorMessage);
        return RpcTranslatorResponse(type, true);
      case AuthRpc_Message.sessionStatusResponse:
        return RpcTranslatorResponse(
          type,
          message.ensureSessionStatusResponse().authenticated,
        );
      case AuthRpc_Message.ack:
        return RpcTranslatorResponse(type, true);
      case AuthRpc_Message.error:
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
