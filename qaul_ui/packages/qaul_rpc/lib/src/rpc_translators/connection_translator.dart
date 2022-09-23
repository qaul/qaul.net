part of 'abstract_rpc_module_translator.dart';

class ConnectionTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.CONNECTIONS;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Reader reader) async {
    final message = Connections.fromBuffer(data);
    switch (message.whichMessage()) {
      case Connections_Message.internetNodesList:
        final nodes = message
            .ensureInternetNodesList()
            .nodes
            .map((e) => InternetNode(e.address, isActive: e.enabled))
            .toList();
        return RpcTranslatorResponse(type, nodes);
      default:
        return super.decodeMessageBytes(data, reader);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Reader reader) async {
    if (res.module != type || res.data is! List<InternetNode>) return;
    reader(connectedNodesProvider.notifier).state = res.data;
  }
}
