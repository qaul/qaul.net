part of 'abstract_rpc_module_translator.dart';

class NodeTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.NODE;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = Node.fromBuffer(data);
    switch (message.whichMessage()) {
      case Node_Message.info:
        final nodeInformation = message.ensureInfo();
        final nodeId = nodeInformation.idBase58;
        return RpcTranslatorResponse(Modules.NODE, nodeId);
      default:
        return super.decodeMessageBytes(data);
    }
  }
}
