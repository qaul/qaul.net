part of 'abstract_rpc_module_translator.dart';

class ConnectionTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.CONNECTIONS;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(
    List<int> data,
    Ref ref,
  ) async {
    final message = Connections.fromBuffer(data);
    switch (message.whichMessage()) {
      case Connections_Message.internetNodesList:
        final nodes = message
            .ensureInternetNodesList()
            .nodes
            .map(InternetNode.fromRpcInternetNodesEntry)
            .toList();
        return RpcTranslatorResponse(type, nodes);
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type || res.data is! List<InternetNode>) return;
    ref.read(connectedNodesProvider.notifier).state = res.data;
  }
}
