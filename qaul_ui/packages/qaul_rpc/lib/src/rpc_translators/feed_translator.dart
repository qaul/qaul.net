part of 'abstract_rpc_module_translator.dart';

class FeedTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.FEED;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Ref ref) async {
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
        return super.decodeMessageBytes(data, ref);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type || res.data is! List<PublicPost>) return;
    final provider = ref.read(publicMessagesProvider.notifier);
    for (final msg in res.data) {
      if (!provider.contains(msg)) provider.add(msg);
    }
  }
}
