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
      default:
        return super.decodeMessageBytes(data, ref);
    }
  }

  @override
  Future<void> processResponse(RpcTranslatorResponse res, Ref ref) async {
    if (res.module != type) return;
    final provider = ref.read(publicMessagesProvider.notifier);

    if (res.data is PaginatedPosts) {
      final paginated = res.data as PaginatedPosts;
      if (paginated.pagination != null) {
        // Paginated response — bulk update
        if (paginated.pagination!.offset == 0) {
          provider.setAll(paginated.posts, pagination: paginated.pagination);
        } else {
          provider.appendOlder(paginated.posts, pagination: paginated.pagination);
        }
      } else {
        // Legacy sync response (no pagination metadata) — prepend newer messages
        provider.prependNewer(paginated.posts);
      }
    }
  }
}
