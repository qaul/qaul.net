part of 'abstract_rpc_module_translator.dart';

class FileSharingTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.CHATFILE;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data) async {
    final message = FileSharing.fromBuffer(data);
    switch (message.whichMessage()) {
      case FileSharing_Message.fileHistoryResponse:
        final entities = message
            .ensureFileHistoryResponse()
            .histories
            .map((e) => FileHistoryEntity.fromRpcEntry(e))
            .toList();
        return RpcTranslatorResponse(Modules.CHATFILE, entities);
      default:
        return super.decodeMessageBytes(data);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Reader reader) async {
    if (res.module != type || res.data is! List<FileHistoryEntity>) return;
    final provider = reader(fileHistoryEntitiesProvider.notifier);
    for (final file in res.data) {
      provider.contains(file) ? provider.update(file) : provider.add(file);
    }
  }
}
