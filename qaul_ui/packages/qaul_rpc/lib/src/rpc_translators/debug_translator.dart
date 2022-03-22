part of 'abstract_rpc_module_translator.dart';

class DebugTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.DEBUG;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = Debug.fromBuffer(data);
    switch (message.whichMessage()) {
      case Debug_Message.heartbeatResponse:
        message.ensureHeartbeatResponse();
        return RpcTranslatorResponse(Modules.DEBUG, true);
      default:
        return super.decodeMessageBytes(data);
    }
  }
}
