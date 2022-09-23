part of 'abstract_rpc_module_translator.dart';

class FeedTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.FEED;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Reader reader) async {
    final message = Feed.fromBuffer(data);
    switch (message.whichMessage()) {
      case Feed_Message.received:
        final newMessages = message
            .ensureReceived()
            .feedMessage
            .map((msg) => msg.toModelMessage)
            .toList();
        return RpcTranslatorResponse(type, newMessages);
      default:
        return super.decodeMessageBytes(data, reader);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Reader reader) async {
    if (res.module != type || res.data is! List<PublicPost>) return;
    final provider = reader(publicMessagesProvider.notifier);
    for (final msg in res.data) {
      if (!provider.contains(msg)) provider.add(msg);
    }
  }
}
