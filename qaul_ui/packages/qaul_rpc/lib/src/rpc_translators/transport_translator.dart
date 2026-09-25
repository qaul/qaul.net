part of 'abstract_rpc_module_translator.dart';

class TransportTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.TRANSPORTS;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(
    List<int> data,
    Ref ref,
  ) async {
    final message = Transports.fromBuffer(data);
    switch (message.whichMessage()) {
      case Transports_Message.list:
        final transports = message
            .ensureList()
            .transports
            .map(NetworkTransport.fromRpc)
            .toList();
        return RpcTranslatorResponse(type, transports);
      case Transports_Message.setEnabledResult:
        return RpcTranslatorResponse(type, message.ensureSetEnabledResult());
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type || res.data is! List<NetworkTransport>) return;
    syncNetworkTransports(
      ref.read(networkTransportsProvider.notifier),
      res.data as List<NetworkTransport>,
    );
  }
}
