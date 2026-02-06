part of 'abstract_rpc_module_translator.dart';

class NodeTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.NODE;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Ref ref) async {
    final message = Node.fromBuffer(data);
    switch (message.whichMessage()) {
      case Node_Message.info:
        final msg = message.ensureInfo();
        final nodeInfo = NodeInfo(msg.idBase58, msg.addresses);
        return RpcTranslatorResponse(type, nodeInfo);
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type || res.data is! NodeInfo) return;
    ref.read(nodeInfoProvider.notifier).setNodeInfo(res.data);
  }
}
