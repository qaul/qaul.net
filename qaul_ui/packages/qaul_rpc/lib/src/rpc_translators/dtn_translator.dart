part of 'abstract_rpc_module_translator.dart';

class DTNTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.DTN;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(
      List<int> data, Reader reader) async {
    final message = DTN.fromBuffer(data);
    switch (message.whichMessage()) {
      case DTN_Message.dtnStateResponse:
        // TODO
        return super.decodeMessageBytes(data, reader);
      case DTN_Message.dtnConfigResponse:
        final users = reader(usersProvider);
        final dtnConfiguration = DTNConfiguration.fromRpcConfigResponse(
          message.ensureDtnConfigResponse(),
          users,
        );
        return RpcTranslatorResponse(type, dtnConfiguration);
      case DTN_Message.dtnAddUserResponse:
        final res = message.ensureDtnAddUserResponse();
        return _receiveResultResponse(res.status, res.message);
      case DTN_Message.dtnRemoveUserResponse:
        final res = message.ensureDtnRemoveUserResponse();
        return _receiveResultResponse(res.status, res.message);
      case DTN_Message.dtnSetTotalSizeResponse:
        final res = message.ensureDtnSetTotalSizeResponse();
        return _receiveResultResponse(res.status, res.message);
      default:
        return super.decodeMessageBytes(data, reader);
    }
  }

  RpcTranslatorResponse _receiveResultResponse(bool status, String message) {
    if (status == true) return RpcTranslatorResponse(type, true);
    throw ArgumentError.value(message, 'DTNTranslator');
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Reader reader) async {
    if (res.module != type || res.data == null) return;

    // Means _receiveResultResponse yielded a success message.
    if (res.data is bool && res.data == true) return;
    if (res.data is DTNConfiguration) {
      reader(dtnConfigurationProvider.notifier).state = res.data;
      return;
    }

    return super.processResponse(res, reader);
  }
}
