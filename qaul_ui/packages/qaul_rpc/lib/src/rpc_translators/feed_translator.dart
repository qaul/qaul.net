part of 'abstract_rpc_module_translator.dart';

class FeedTranslator extends RpcModuleTranslator {
  @override
  Modules get type => Modules.FEED;

  @override
  Future<RpcTranslatorResponse?> decodeMessageBytes(List<int> data, Ref ref) async {
    final message = Feed.fromBuffer(data);
    switch (message.whichMessage()) {
      case Feed_Message.received:
        final received = message.ensureReceived();
        final posts = received.feedMessage
            .map((msg) => msg.toModelMessage)
            .toList();

        PaginationState? pagination;
        if (received.hasPagination()) {
          final meta = received.pagination;
          pagination = PaginationState(
            hasMore: meta.hasMore,
            total: meta.total,
            offset: meta.offset,
            limit: meta.limit,
          );
        }
        return RpcTranslatorResponse(
          type,
          PaginatedPosts(posts: posts, pagination: pagination),
        );
      case Feed_Message.sendResponse:
        return RpcTranslatorResponse(type, message.ensureSendResponse());
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    // Feed list updates are applied via LibqaulWorker futures (FeedMessageStore).
  }
}
