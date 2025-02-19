part of 'abstract_rpc_module_translator.dart';

class ChatFileTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.CHATFILE;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Ref ref) async {
    final message = ChatFile.fromBuffer(data);
    switch (message.whichMessage()) {
      case ChatFile_Message.fileHistoryResponse:
        final entities = message
            .ensureFileHistoryResponse()
            .histories
            .map((e) => FileHistoryEntity.fromRpcEntry(e))
            .toList();
        return RpcTranslatorResponse(type, entities);
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type || res.data is! List<FileHistoryEntity>) return;
    final provider = ref.read(fileHistoryEntitiesProvider.notifier);
    for (final file in res.data) {
      provider.contains(file) ? provider.update(file) : provider.add(file);
    }
  }
}
