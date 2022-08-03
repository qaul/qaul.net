part of 'abstract_rpc_module_translator.dart';

class FileSharingTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.FILESHARE;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = FileSharing.fromBuffer(data);
    switch (message.whichMessage()) {
      case FileSharing_Message.fileHistoryResponse:
        final entities = message
            .ensureFileHistoryResponse()
            .histories
            .map((e) => FileHistoryEntity.fromRpcEntry(e));
        return RpcTranslatorResponse(Modules.FILESHARE, entities);
      default:
        return super.decodeMessageBytes(data);
    }
  }
}
